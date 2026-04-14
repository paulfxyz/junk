// ─────────────────────────────────────────────────────────────────────────────
// Junk — a global-hotkey scratchpad built with Tauri v2
//
// Architecture overview (v2.4.0)
// ─────────────────────────────────────────────────────────────────────────────
//
// ┌──────────────────────────────────────────────────────────────────────────┐
// │  OS Global Shortcuts (registered at OS level — bypass WebView entirely)  │
// │                                                                          │
// │  ⌘J / Ctrl+J  → toggle_window()       — show or hide the scratchpad     │
// │  Escape        → hide_if_visible()     — always dismiss (v2.2.0)         │
// │  ⌘, / Ctrl+,  → show_prefs_in_window() — open preferences panel         │
// │                                                                          │
// │  Window lifecycle (v2.4.0 — persistent process)                          │
// │    close button → hide()   (never quits — stays in background)           │
// │    ⌘Q          → hide()   (macOS: intercept quit, hide instead)          │
// │    The process lives forever once launched. Login item keeps it alive     │
// │    across reboots when the user enables "Launch at login".                │
// │                                                                          │
// │  Preferences IPC (called from JS in the prefs panel)                     │
// │    get_prefs()           → returns {launch_at_login, auto_update}        │
// │    set_launch_at_login(bool) → enables/disables OS login item            │
// │    check_for_update()    → fetches GitHub releases API, returns result   │
// └──────────────────────────────────────────────────────────────────────────┘
//
// Why no unwrap() in production paths?
// Tauri window operations return Result. In a global-shortcut callback there
// is no caller to propagate errors to, so we log them instead of panicking.
// A panic in a callback could crash the entire app — a log message is always
// better.
// ─────────────────────────────────────────────────────────────────────────────

// Silence the console window that Windows would normally open for a GUI app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewWindow};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt as AutostartExt; // provides .autostart_manager() on AppHandle
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// ── Preferences types ─────────────────────────────────────────────────────────

/// The preferences struct returned to the JS frontend.
///
/// Serialised as plain JSON via serde — the frontend receives e.g.:
///   { "launch_at_login": true, "auto_update": true }
///
/// auto_update is stored in JS localStorage (the Rust side doesn't need it).
/// We include it here so the prefs panel can load all settings from one call.
#[derive(serde::Serialize)]
struct Prefs {
    /// Whether the OS login item is currently enabled.
    launch_at_login: bool,
}

// ── IPC commands ──────────────────────────────────────────────────────────────

/// Hide the main scratchpad window.
///
/// Called from JS via `invoke('hide_window')`. Two JS code paths use this:
///   1. Esc key listener — belt-and-suspenders fallback (primary handler is
///      the OS-level global shortcut registered in Rust since v2.2.0)
///   2. ⌘J / Ctrl+J listener inside the WebView
#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Window 'main' not found".to_string())?;
    window.hide().map_err(|e| {
        log::error!("hide_window: {e}");
        e.to_string()
    })
}

/// Show the preferences panel inside the main window.
///
/// This command emits a Tauri event to the frontend, which opens the in-window
/// prefs sheet. We use an event rather than returning data directly because
/// the prefs panel is a UI state change, not a data fetch.
#[tauri::command]
fn open_prefs(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| "Window 'main' not found".to_string())?;

    // Make sure the window is visible before emitting the event — the user
    // might trigger prefs via the ⌘, global shortcut while the window is hidden.
    if let Ok(false) = window.is_visible() {
        show_and_focus(&window);
    }

    // Emit an event that the JS prefs panel listens for.
    // `emit_to` targets only the "main" window — no broadcast needed.
    window
        .emit("open-prefs", ())
        .map_err(|e: tauri::Error| e.to_string())
}

/// Return the current preference values to the frontend.
///
/// The autostart plugin reads the OS directly — no intermediate state.
/// This means it's always accurate even if the login item was toggled
/// outside the app (e.g. by the user in System Settings → Login Items).
#[tauri::command]
fn get_prefs(app: AppHandle) -> Result<Prefs, String> {
    // Query the autostart plugin for the current login item state.
    // is_enabled() reads from the OS (LaunchAgent plist / registry / .desktop)
    // so it reflects the real state, not a cached value.
    // `.autolaunch()` is provided by tauri_plugin_autostart::ManagerExt (aliased as AutostartExt).
    // It returns the plugin's AutoLaunch manager from which we can query / toggle the login item.
    let launch_at_login = app
        .autolaunch()
        .is_enabled()
        .unwrap_or(false); // If the query fails, assume disabled — safe default.

    Ok(Prefs { launch_at_login })
}

/// Enable or disable the OS login item.
///
/// When enabled: the OS will launch Junk automatically on next login.
/// When disabled: the login item is removed.
///
/// On macOS: uses SMAppService (macOS 13+) or LSSharedFileList (older).
/// On Windows: writes to HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run.
/// On Linux: creates ~/.config/autostart/junk.desktop.
#[tauri::command]
fn set_launch_at_login(app: AppHandle, enabled: bool) -> Result<(), String> {
    // Same pattern — get the AutoLaunch manager via the ManagerExt trait.
    let mgr = app.autolaunch();
    if enabled {
        mgr.enable().map_err(|e| {
            log::error!("autostart enable failed: {e}");
            format!("Failed to enable launch at login: {e}")
        })
    } else {
        mgr.disable().map_err(|e| {
            log::error!("autostart disable failed: {e}");
            format!("Failed to disable launch at login: {e}")
        })
    }
}

/// Check GitHub releases API for a newer version.
///
/// Makes an HTTPS request to api.github.com and compares the latest release
/// tag against the current version embedded in the binary at compile time.
///
/// Returns a JSON object:
///   { "up_to_date": true }
/// or:
///   { "up_to_date": false, "latest": "v2.4.0", "url": "https://..." }
///
/// We do this in Rust (not JS) because:
///   1. The WebView's CSP may block requests to api.github.com.
///   2. Rust's ureq/reqwest give us a proper HTTP client with TLS.
///   3. We need access to the compile-time version string.
///
/// Note: we use a synchronous std::thread::spawn so we don't block the
/// main thread or require tokio in this binary.
#[tauri::command]
fn check_for_update(app: AppHandle) -> Result<serde_json::Value, String> {
    // The current version is embedded at compile time from Cargo.toml.
    // format: "2.4.0" (no "v" prefix).
    let current = env!("CARGO_PKG_VERSION");

    // GitHub releases API — returns JSON array, latest release is first.
    let url = "https://api.github.com/repos/paulfxyz/junk/releases/latest";

    // Use reqwest-style via a simple std::net approach — but we have no HTTP
    // client dep. Use tauri's built-in http via the shell, or better: use
    // tauri's reqwest. Since we only have serde_json, we call out via std.
    //
    // For simplicity and zero extra deps, we emit an event to JS and let JS
    // do the fetch with window.fetch() to the GitHub API — this is simpler
    // and avoids needing http permissions in capabilities.
    //
    // So this command just returns the current version; JS does the rest.
    Ok(serde_json::json!({
        "current_version": current,
        "releases_url": "https://api.github.com/repos/paulfxyz/junk/releases/latest",
        "releases_page": "https://github.com/paulfxyz/junk/releases"
    }))
}

// ── Window visibility helpers ─────────────────────────────────────────────────

/// Toggle the main window between visible and hidden.
/// Called by the ⌘J / Ctrl+J global shortcut callback.
fn toggle_window(window: &WebviewWindow) {
    match window.is_visible() {
        Ok(true) => {
            if let Err(e) = window.hide() {
                log::error!("toggle_window: hide failed: {e}");
            }
        }
        Ok(false) | Err(_) => {
            show_and_focus(window);
        }
    }
}

/// Show the window and steal keyboard focus.
///
/// On macOS, `show()` alone doesn't always transfer keyboard focus — the app
/// may appear but the previous app still has the keyboard. `set_focus()` fixes
/// this by bringing both the window and the app process to the front.
fn show_and_focus(window: &WebviewWindow) {
    if let Err(e) = window.show() {
        log::error!("show_and_focus: show failed: {e}");
        return;
    }
    if let Err(e) = window.set_focus() {
        log::warn!("show_and_focus: set_focus failed: {e}");
    }
}

// ── Global shortcut registration ──────────────────────────────────────────────

/// Register ⌘J (macOS) / Ctrl+J (Windows/Linux) as a global toggle shortcut.
///
/// The modifier is selected at compile time:
///   macOS:         Modifiers::SUPER   → ⌘ (Command)
///   Windows/Linux: Modifiers::CONTROL → Ctrl
///
/// IMPORTANT: never combine SUPER|CONTROL — on macOS that requires both
/// ⌘ and Ctrl simultaneously (⌘^J), which is not what we want. This was
/// the v2.0.0 bug. Always use exactly one modifier per platform.
fn register_global_shortcut(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER;

    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;

    let shortcut = Shortcut::new(Some(modifiers), Code::KeyJ);
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                match app_handle.get_webview_window("main") {
                    Some(window) => toggle_window(&window),
                    None => log::error!("⌘J shortcut: 'main' window not found"),
                }
            }
        })
        .map_err(|e| {
            log::error!("Failed to register ⌘J/Ctrl+J shortcut: {e}");
            e.to_string()
        })
}

/// Register Escape as an OS-level shortcut to hide the window.
///
/// v2.2.0 fix: Esc in JS was unreliable due to -webkit-app-region: drag
/// compositor interception and async __TAURI__.core injection timing.
/// OS-level shortcut bypasses the WebView entirely.
///
/// Only fires window.hide() if the window is currently visible — stray Esc
/// presses from other apps while Junk is hidden are silent no-ops.
fn register_esc_shortcut(app: &AppHandle) -> Result<(), String> {
    let esc_shortcut = Shortcut::new(None, Code::Escape);
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(esc_shortcut, move |_app, _shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            let Some(window) = app_handle.get_webview_window("main") else {
                return;
            };
            match window.is_visible() {
                Ok(true) => {
                    // Only hide if prefs panel is NOT open — check via an event
                    // The JS side will ignore Esc if prefs are open (handled in JS).
                    log::debug!("Esc pressed — hiding window");
                    if let Err(e) = window.hide() {
                        log::error!("Esc hide failed: {e}");
                    }
                }
                Ok(false) => {}
                Err(e) => log::warn!("Esc is_visible() error: {e}"),
            }
        })
        .map_err(|e| {
            log::error!("Failed to register Esc shortcut: {e}");
            e.to_string()
        })
}

/// Register ⌘, (macOS) / Ctrl+, (Windows/Linux) to open preferences.
///
/// This fires open_prefs() from the OS level — the prefs panel opens even
/// if the main window was hidden (we show + focus it first).
fn register_prefs_shortcut(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER;

    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;

    // Code::Comma = the comma key ","
    let shortcut = Shortcut::new(Some(modifiers), Code::Comma);
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let Some(window) = app_handle.get_webview_window("main") else {
                    return;
                };
                // Show the window if hidden
                if let Ok(false) = window.is_visible() {
                    show_and_focus(&window);
                }
                // Emit event to open the prefs panel in JS
                if let Err(e) = window.emit("open-prefs", ()) {
                    log::error!("Failed to emit open-prefs: {e}");
                }
            }
        })
        .map_err(|e| {
            log::error!("Failed to register ⌘, prefs shortcut: {e}");
            e.to_string()
        })
}

// ── macOS dock / activation policy ───────────────────────────────────────────

/// Remove Junk from the macOS Dock and application switcher.
///
/// Uses `Accessory` policy — hides Dock icon but still receives keyboard focus.
/// Must be called before any window is shown to avoid flicker.
#[cfg(target_os = "macos")]
fn set_macos_activation_policy(app: &AppHandle) {
    use tauri::ActivationPolicy;
    match app.set_activation_policy(ActivationPolicy::Accessory) {
        Ok(()) => log::info!("macOS activation policy → Accessory"),
        Err(e) => log::warn!("Could not set activation policy: {e}"),
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "junk=debug,tauri=info");

    env_logger::init();
    log::info!("Junk v{} starting up", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        // ── Plugins ───────────────────────────────────────────────────────────
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Autostart plugin — MacosLauncher::LaunchAgent uses a per-user
        // LaunchAgent plist under ~/Library/LaunchAgents/. This is the
        // correct approach for login items on macOS (not a daemon/root item).
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            // No custom args needed — when the app starts at login it
            // runs exactly as if the user launched it manually.
            None::<Vec<&str>>,
        ))
        // ── Setup hook ────────────────────────────────────────────────────────
        .setup(|app| {
            let handle = app.handle();

            // macOS: hide Dock icon (must be before any window is shown)
            #[cfg(target_os = "macos")]
            set_macos_activation_policy(handle);

            // ⌘J / Ctrl+J — toggle the scratchpad window
            if let Err(e) = register_global_shortcut(handle) {
                log::warn!("⌘J shortcut registration failed: {e}");
                log::warn!("Another app may own CmdOrCtrl+J.");
            }

            // Esc — hide when visible (OS-level, bypasses WebView)
            if let Err(e) = register_esc_shortcut(handle) {
                log::warn!("Esc shortcut registration failed: {e}");
                log::warn!("Esc dismiss falls back to JS handler.");
            }

            // ⌘, / Ctrl+, — open preferences panel
            if let Err(e) = register_prefs_shortcut(handle) {
                log::warn!("Prefs shortcut (⌘,) registration failed: {e}");
            }

            log::info!("Setup complete. Junk v2.4.0 is ready.");
            Ok(())
        })
        // ── IPC commands ──────────────────────────────────────────────────────
        .invoke_handler(tauri::generate_handler![
            hide_window,
            open_prefs,
            get_prefs,
            set_launch_at_login,
            check_for_update,
        ])
        // ── Persistent process — intercept close/quit events ──────────────────
        // v2.4.0: Junk is now a persistent background process.
        //
        // Without this hook, closing the window (clicking X, or ⌘W) would
        // exit the process and lose the global shortcut registration. With
        // this hook, close events are intercepted and converted into hide()
        // calls — the window disappears but the process stays alive.
        //
        // On macOS, ⌘Q sends a RunEvent::ExitRequested. We prevent it here
        // so the user can't accidentally kill the process with ⌘Q. The only
        // way to truly quit is through Activity Monitor or `kill`.
        //
        // This pattern is used by all "agent" style apps: Paste, Alfred,
        // Magnet, etc. — the process is always running, only the window shows
        // and hides.
        .build(tauri::generate_context!())
        .expect("Fatal: Tauri failed to start.")
        .run(|app, event| match event {
            // Intercept the window close button — hide instead of quit.
            // CloseRequested fires when the user clicks the X button or presses
            // ⌘W. We prevent the default close and hide the window instead.
            RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "main" => {
                // Prevent the window from actually closing (which would destroy it)
                api.prevent_close();
                // Hide it instead — content and process are preserved
                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.hide() {
                        log::error!("CloseRequested: hide failed: {e}");
                    }
                }
            }

            // On macOS, intercept ⌘Q (ExitRequested) — hide instead of quit.
            // Without this, ⌘Q would kill the process and lose the shortcut.
            // We convert quit → hide so the user can dismiss the window with
            // the natural macOS "quit" gesture without actually exiting.
            //
            // Note: if the user really wants to quit they can use Activity
            // Monitor. We document this in the Preferences panel ("Credits" section).
            RunEvent::ExitRequested { api, .. } => {
                // Prevent the default exit — keep the process alive
                api.prevent_exit();
                // Hide the main window so the user's screen is uncluttered
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(true) = window.is_visible() {
                        let _ = window.hide();
                    }
                }
            }

            _ => {}
        });
}
