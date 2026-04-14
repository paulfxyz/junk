// ─────────────────────────────────────────────────────────────────────────────
// Junk — a global-hotkey scratchpad built with Tauri v2
//
// Architecture overview
// ─────────────────────────────────────────────────────────────────────────────
//
// ┌──────────────────────────────────────────────────────────────────────────┐
// │  OS Global Shortcuts (registered at OS level — bypass WebView entirely)  │
// │                                                                          │
// │  ⌘J / Ctrl+J  → toggle_window()  — show or hide the scratchpad          │
// │  Escape        → hide_if_visible() — always dismiss (v2.2.0 fix)         │
// │                                                                          │
// │  tauri_plugin_global_shortcut callback                                   │
// │         │                                                                │
// │         ├─ ⌘J/Ctrl+J: is_visible()? → true  → window.hide()            │
// │         │              is_visible()? → false → window.show()+set_focus() │
// │         └─ Esc:        is_visible()? → true  → window.hide()            │
// │                        is_visible()? → false → (no-op, already hidden)  │
// │                                                                          │
// │  Frontend (index.html) — belt-and-suspenders JS fallback                 │
// │    Esc key   → invoke("hide_window")  (may be unreliable — see v2.2.0)  │
// │    ⌘J/Ctrl+J → invoke("hide_window")  (belt-and-suspenders)             │
// │                                                                          │
// │  v2.2.0 fix: Esc is now a true OS-level global shortcut — it fires even │
// │  when the WebView has consumed the event or JS hasn't loaded yet.        │
// └──────────────────────────────────────────────────────────────────────────┘
//
// Why no unwrap() in production paths?
// Tauri window operations return Result. In a global-shortcut callback there
// is no caller to propagate errors to, so we log them instead of panicking.
// A panic in a callback could crash the entire app — a log message is always
// better.
//
// Why WebviewWindow, not Window?
// Tauri v2 renamed the window type. WebviewWindow wraps both the OS window
// and its embedded WebView; Window is now only used for raw OS-level windows
// that don't embed a WebView (rare). All IPC lives inside WebviewWindow.
// ─────────────────────────────────────────────────────────────────────────────

// Silence the console window that Windows would normally open for a GUI app.
// Without this, double-clicking junk.exe shows a black console window first.
// On macOS/Linux this attribute is a no-op, so it is always safe to include.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// ── IPC commands ──────────────────────────────────────────────────────────────

/// Hide the main scratchpad window.
///
/// This command is invoked from the frontend via:
///
/// ```js
/// await window.__TAURI__.core.invoke('hide_window');
/// ```
///
/// Two frontend code-paths call this:
///   1. The Escape key listener — quickest way to dismiss.
///   2. The CmdOrCtrl+J key listener inside the WebView — belt-and-suspenders
///      in case the OS-level global shortcut fires inside the app itself.
///
/// We receive the `AppHandle` via Tauri's dependency-injection magic — any
/// `#[tauri::command]` parameter whose type is `AppHandle` is automatically
/// filled by the framework. No argument needs to be passed from JS.
///
/// Returns Ok(()) on success so the Promise resolves cleanly on the JS side.
/// On error we log and return Err — the frontend silently ignores the error
/// (fire-and-forget pattern).
#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    // Look up our single named window. "main" is the label set in tauri.conf.json.
    // get_webview_window returns None only if the label doesn't exist, which
    // would be a programming error, so we map None to a string error.
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Window 'main' not found".to_string())?;

    // hide() is equivalent to clicking the close button but without destroying
    // the window. The WebView state (textarea content, scroll position) is
    // preserved. This is the core UX contract of Junk: fast show/hide with
    // zero loading time.
    window.hide().map_err(|e| {
        log::error!("Failed to hide window: {e}");
        e.to_string()
    })
}

// ── Window visibility toggle ──────────────────────────────────────────────────

/// Toggle the main window between visible and hidden.
///
/// Called by the global shortcut callback (see `register_global_shortcut`).
/// This is the heart of the app's UX loop.
///
/// Design decision: we deliberately do NOT hide on blur (the window has no
/// blur handler). This lets users:
///   1. Open Junk
///   2. Click a URL in Junk
///   3. Copy something from the browser
///   4. Cmd+Tab back to Junk and paste
/// Hiding on blur would break step 3 — the window would vanish the moment
/// the user clicked the browser.
fn toggle_window(window: &WebviewWindow) {
    // is_visible() can fail if the window handle is stale (e.g. destroyed by
    // a race condition). We treat that as "window is not visible" and attempt
    // to show it; the show() call will also fail gracefully if the handle is
    // truly invalid.
    match window.is_visible() {
        Ok(true) => {
            // Window is currently on screen — hide it.
            if let Err(e) = window.hide() {
                log::error!("toggle_window: hide failed: {e}");
            }
        }
        Ok(false) | Err(_) => {
            // Window is hidden (or we can't determine state) — show it.
            show_and_focus(window);
        }
    }
}

/// Show the window and give it keyboard focus.
///
/// Separated from `toggle_window` so we can call it from the startup path
/// without duplicating the focus logic.
///
/// On macOS, `show()` alone is not always enough — the window appears but the
/// app may not be the active application (especially if it was launched at
/// login without a Dock icon). `set_focus()` brings both the window and the
/// app to the front.
fn show_and_focus(window: &WebviewWindow) {
    if let Err(e) = window.show() {
        log::error!("show_and_focus: show failed: {e}");
        return; // No point trying set_focus if show failed
    }
    if let Err(e) = window.set_focus() {
        // Non-fatal: the window is visible, it just might not have keyboard
        // focus. Log and continue.
        log::warn!("show_and_focus: set_focus failed: {e}");
    }
}

// ── Global shortcut registration ──────────────────────────────────────────────

/// Register the platform-correct global shortcut (⌘J / Ctrl+J) with the OS.
///
/// Why register after the app is built rather than in a setup hook?
/// Tauri v2's shortcut plugin requires the app to be fully initialised
/// (all plugins registered, event loop running) before it can talk to the OS
/// shortcut daemon. The `setup` closure runs before the event loop, which
/// works, but using a `RunEvent::Ready` or post-build call is equally valid
/// and sometimes clearer. Here we call this from `setup` because Tauri v2
/// guarantees plugins are ready by then.
///
/// The shortcut is defined as a `Shortcut` value composed of:
///   - `Modifiers::SUPER` on macOS  → ⌘J
///   - `Modifiers::CONTROL` on Windows / Linux → Ctrl+J
///
/// IMPORTANT: Do NOT use `Modifiers::SUPER | Modifiers::CONTROL` combined.
/// On macOS, registering SUPER|CONTROL means the OS requires *both* ⌘ and
/// Ctrl to be held simultaneously — i.e. ⌘^J, not ⌘J. This was the v2.0.0
/// bug: the shortcut appeared to register without error but never fired on
/// a plain ⌘J press. The fix is to select exactly one modifier per platform
/// using compile-time cfg guards.
///
/// Returns an error string if registration fails (e.g. another app already
/// owns that shortcut).
fn register_global_shortcut(app: &AppHandle) -> Result<(), String> {
    // Select the correct modifier for this platform at compile time.
    //
    // macOS:          Modifiers::SUPER   → the ⌘ (Command) key
    // Windows/Linux:  Modifiers::CONTROL → the Ctrl key
    //
    // Using cfg() here means the wrong branch is completely absent from the
    // compiled binary — there is zero runtime cost and no logic error possible.
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER;      // ⌘J on macOS

    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;    // Ctrl+J on Windows / Linux

    let shortcut = Shortcut::new(Some(modifiers), Code::KeyJ);

    // Clone the handle so we can move it into the closure without borrowing
    // `app` beyond this function's lifetime.
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            // The callback fires for both KeyDown and KeyUp on some platforms.
            // We only want to act on KeyDown (the initial press) to avoid
            // toggling twice per keystroke.
            if event.state == ShortcutState::Pressed {
                // Retrieve the window inside the callback. If it was somehow
                // destroyed between registration and the callback firing, we
                // log and bail rather than panic.
                match app_handle.get_webview_window("main") {
                    Some(window) => toggle_window(&window),
                    None => log::error!("Global shortcut fired but 'main' window not found"),
                }
            }
        })
        .map_err(|e| {
            log::error!("Failed to register global shortcut CmdOrCtrl+J: {e}");
            e.to_string()
        })
}

/// Register the Escape key as an OS-level global shortcut to hide the window.
///
/// v2.2.0 fix: Esc was previously handled only in JS (index.html, keydown
/// listener with `{ capture: true }`). That approach is unreliable because:
///
///   1. `-webkit-app-region: drag` on the window div can swallow keyboard
///      events at the compositor level before they reach the WebView's JS.
///   2. In Tauri v2 with `<script type="module">`, the IPC bridge
///      (`window.__TAURI__.core`) is injected asynchronously — if the user
///      presses Esc before the module finishes loading, `invoke` is undefined
///      and the call silently fails.
///   3. Some OS-level focus states (e.g. immediately after a global shortcut
///      summons the window) leave the WebView in a state where it does not
///      receive key events reliably.
///
/// By registering Esc as an OS global shortcut in Rust we bypass the WebView
/// entirely. The OS delivers the event directly to our callback, which then
/// calls `window.hide()` through the Tauri Rust API — no JS involved.
///
/// Conflict risk: Escape without modifiers is a "hot" key. However, the OS
/// routes the event to the currently focused application first, and global
/// shortcuts only intercept if no other handler consumed it — in practice
/// bare-Escape conflicts are extremely rare at the global-shortcut level.
/// We also guard on `is_visible()` so if Junk is already hidden we no-op,
/// making any stray event completely harmless.
///
/// Returns an error string if registration fails (non-fatal — caller logs and
/// falls back to the JS handler in index.html).
fn register_esc_shortcut(app: &AppHandle) -> Result<(), String> {
    // `None` modifier means "no modifier key required" — plain Escape.
    // This is the correct choice: we want a bare Esc press, not ⌘Esc or Ctrl+Esc.
    let esc_shortcut = Shortcut::new(None, Code::Escape);

    // Clone the handle so it can be moved into the 'static closure required
    // by on_shortcut().
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(esc_shortcut, move |_app, _shortcut, event| {
            // Only act on the key-down phase to avoid double-firing on key-up.
            if event.state != ShortcutState::Pressed {
                return;
            }

            // Retrieve the window. If it doesn't exist (shouldn't happen in
            // normal operation) we log and bail — never panic in a callback.
            let Some(window) = app_handle.get_webview_window("main") else {
                log::error!("Esc shortcut: 'main' window not found");
                return;
            };

            // Only hide if the window is actually visible.
            // If it's already hidden, a stray Esc from another context would
            // cause a no-op hide() — harmless but noisy in logs. We skip it.
            match window.is_visible() {
                Ok(true) => {
                    log::debug!("Esc pressed — hiding window");
                    if let Err(e) = window.hide() {
                        log::error!("Esc shortcut: hide() failed: {e}");
                    }
                }
                Ok(false) => {
                    // Window already hidden — silent no-op.
                    // This fires if the user presses Esc in another app while
                    // Junk is in the background. We intentionally do nothing.
                }
                Err(e) => {
                    // is_visible() itself failed — stale handle? Log and ignore.
                    log::warn!("Esc shortcut: is_visible() error: {e}");
                }
            }
        })
        .map_err(|e| {
            log::error!("Failed to register Esc global shortcut: {e}");
            e.to_string()
        })
}

// ── macOS dock / activation policy ───────────────────────────────────────────

/// Remove the app from the macOS Dock and application switcher.
///
/// macOS has three activation policies:
///   - Regular   : Dock icon + App Switcher (default for GUI apps)
///   - Accessory : No Dock icon, appears in App Switcher only when a window
///                 is visible (used by e.g. Paste, Magnet)
///   - Prohibited: No Dock icon, never in App Switcher (used by daemon tools)
///
/// We use `Accessory` — it hides the Dock icon but still allows the app to
/// receive keyboard focus and display windows, which is exactly what we want.
///
/// This function is compiled only on macOS (cfg guard). On Windows/Linux the
/// equivalent is `skip_taskbar: true` in tauri.conf.json — no runtime call
/// is needed there.
#[cfg(target_os = "macos")]
fn set_macos_activation_policy(app: &AppHandle) {
    use tauri::ActivationPolicy;
    // set_activation_policy returns a bool indicating whether the policy
    // changed. We don't need to act on it, but we log for diagnostics.
    // set_activation_policy returns Result<(), tauri::Error> in Tauri v2.
    // We ignore the return value — if it fails the worst outcome is the Dock
    // icon appears, which is cosmetic. Log success or the error string.
    match app.set_activation_policy(ActivationPolicy::Accessory) {
        Ok(()) => log::info!("macOS activation policy set to Accessory"),
        Err(e) => log::warn!("Could not set activation policy: {e}"),
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    // Initialise the logger. In debug builds RUST_LOG defaults to "junk=debug"
    // so all our log! calls are visible. In release builds the binary is silent
    // unless RUST_LOG is set explicitly by the user.
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "junk=debug,tauri=info");

    env_logger::init();
    log::info!("Junk starting up");

    tauri::Builder::default()
        // ── Plugins ───────────────────────────────────────────────────────────
        // Register the global shortcut plugin. In Tauri v2 every OS capability
        // that was previously baked into core is now a separate plugin. This
        // keeps the core small and lets you opt in only to what you need.
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // ── Setup hook ────────────────────────────────────────────────────────
        // `setup` runs once after all plugins are initialised but before the
        // event loop starts. This is the canonical place to:
        //   1. Configure platform-specific behaviour (Dock policy)
        //   2. Register shortcuts
        //   3. Do any one-time startup work
        //
        // The closure receives a `&mut App` — we call `.handle()` to get a
        // cloneable `AppHandle` that can be moved into closures/threads.
        .setup(|app| {
            let handle = app.handle();

            // ── macOS: hide Dock icon ─────────────────────────────────────
            // Must be called BEFORE any windows are shown. Changing the policy
            // after a window is visible can cause the window to flicker.
            #[cfg(target_os = "macos")]
            set_macos_activation_policy(handle);

            // ── Register ⌘J / Ctrl+J ─────────────────────────────────────
            // Errors here are non-fatal — the user can still type in Junk,
            // they just can't summon it with the shortcut. We log and continue
            // rather than aborting the app startup.
            if let Err(e) = register_global_shortcut(handle) {
                log::warn!("Could not register global shortcut: {e}");
                log::warn!("Another app may already own CmdOrCtrl+J.");
            }

            // ── Register Esc (v2.2.0 fix) ─────────────────────────────────
            // Register Escape as an OS-level shortcut so it reliably hides
            // the window regardless of WebView focus or JS module load state.
            // See `register_esc_shortcut` doc comment for full rationale.
            // Non-fatal: if registration fails we fall back to the JS handler
            // in index.html (belt-and-suspenders, may be unreliable).
            if let Err(e) = register_esc_shortcut(handle) {
                log::warn!("Could not register Esc shortcut: {e}");
                log::warn!("Esc dismiss will fall back to JS handler.");
            }

            log::info!("Setup complete. Junk v2.2.0 is ready.");
            Ok(())
        })
        // ── IPC commands ──────────────────────────────────────────────────────
        // Register every Rust function the frontend can invoke(). Any function
        // decorated with #[tauri::command] must appear here or Tauri will
        // return an error when JS calls invoke()..
        .invoke_handler(tauri::generate_handler![hide_window])
        // ── Run ───────────────────────────────────────────────────────────────
        // generate_context!() reads tauri.conf.json at compile time and embeds
        // the configuration + frontend assets into the binary. This macro is
        // what makes Tauri apps single-file executables.
        .run(tauri::generate_context!())
        // If run() itself fails (e.g. the WebView engine is missing on Linux),
        // we panic with a human-readable message. This is the one place a panic
        // is acceptable — the app literally cannot start.
        .expect("Fatal: Tauri failed to start. Is a WebView engine installed?");
}
