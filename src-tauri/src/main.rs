// ─────────────────────────────────────────────────────────────────────────────
// Junk — a global-hotkey scratchpad built with Tauri v2
//
// Architecture overview (v2.5.0)
// ─────────────────────────────────────────────────────────────────────────────
//
// ┌──────────────────────────────────────────────────────────────────────────┐
// │  OS Global Shortcuts (registered at OS level — bypass WebView entirely)  │
// │                                                                          │
// │  ⌘J / Ctrl+J  → toggle_window()       — show or hide the scratchpad     │
// │  Escape        → hide_if_visible()     — always dismiss                  │
// │  ⌘, / Ctrl+,  → show_prefs_in_window() — open preferences panel         │
// │                                                                          │
// │  Menu Bar / System Tray (v2.5.0)                                         │
// │    macOS: icon in the menu bar (right side, always visible)              │
// │    Windows/Linux: icon in the system tray / notification area            │
// │    Menu items: Show/Hide · Preferences · Check for Updates · Quit        │
// │    Single-click on the tray icon also toggles the window.                │
// │                                                                          │
// │  Window lifecycle (persistent process)                                   │
// │    close button → hide()   (never quits — stays in background)           │
// │    ⌘Q          → hide()   (intercept quit, hide instead)                 │
// │    Quit (tray) → app.exit(0) — the ONLY way to truly quit               │
// │                                                                          │
// │  IPC commands (called from JS in the prefs panel / update checker)       │
// │    hide_window()             → hides the main window                     │
// │    open_prefs()              → emits 'open-prefs' event to JS            │
// │    get_prefs()               → returns { launch_at_login }               │
// │    set_launch_at_login(bool) → enables/disables OS login item            │
// │    check_for_update()        → Rust HTTP fetch → GitHub API → JSON       │
// └──────────────────────────────────────────────────────────────────────────┘
//
// Why no unwrap() in production paths?
// Tauri window operations return Result. In a shortcut callback there is no
// caller to propagate errors to, so we log them instead of panicking. A panic
// in a callback crashes the entire app — a log message is always better.
// ─────────────────────────────────────────────────────────────────────────────

// Silence the console window that Windows would normally open for a GUI app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, RunEvent, WebviewWindow,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// ── Preferences types ─────────────────────────────────────────────────────────

/// The preferences struct returned to the JS frontend via `get_prefs()`.
///
/// Serialised as plain JSON via serde — the frontend receives e.g.:
///   { "launch_at_login": true }
#[derive(serde::Serialize)]
struct Prefs {
    /// Whether the OS login item is currently enabled.
    /// Read from the OS on every call (not cached) so it stays accurate if
    /// the user toggles it from System Settings / Task Manager externally.
    launch_at_login: bool,
}

/// Update check result returned from `check_for_update()`.
///
/// Serialised as JSON:
///   { "up_to_date": true, "current": "2.5.0", "latest": "2.5.0", "url": "..." }
/// or:
///   { "up_to_date": false, "current": "2.4.0", "latest": "2.5.0", "url": "..." }
#[derive(serde::Serialize)]
struct UpdateResult {
    up_to_date: bool,
    current: String,
    latest: String,
    url: String,
}

// ── IPC commands ──────────────────────────────────────────────────────────────

/// Hide the main scratchpad window.
///
/// Called from JS via `invoke('hide_window')`. This is a thin IPC wrapper
/// around `window.hide()` — JS cannot call native window APIs directly.
#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window.hide().map_err(|e| e.to_string())
}

/// Open the preferences panel in the frontend.
///
/// This command:
///   1. Shows and focuses the window if it was hidden (so ⌘, works globally)
///   2. Emits the 'open-prefs' Tauri event to the JS frontend
///
/// The JS side listens for 'open-prefs' and slides the prefs panel up.
#[tauri::command]
fn open_prefs(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;

    // Ensure the window is visible before emitting — if it was hidden, the
    // JS event listener is still active (WebView lives forever) but the user
    // needs to see the panel.
    if let Ok(false) = window.is_visible() {
        show_and_focus(&window);
    }

    // Emit the event that triggers the JS prefs panel slide-up animation.
    window
        .emit("open-prefs", ())
        .map_err(|e: tauri::Error| e.to_string())
}

/// Return the current preference values to the frontend.
///
/// Called when the prefs panel opens to populate the toggle states.
/// Reads real OS state on every call — accurate even if the user changed
/// the login item via System Settings since the app last ran.
#[tauri::command]
fn get_prefs(app: AppHandle) -> Result<Prefs, String> {
    let launch_at_login = app
        .autolaunch()
        .is_enabled()
        .unwrap_or(false);

    Ok(Prefs { launch_at_login })
}

/// Enable or disable the OS login item for Junk.
///
/// On macOS: creates/removes a LaunchAgent plist in ~/Library/LaunchAgents/.
///   This is a per-user item — no admin/root privileges required.
///   launchd reads the plist on next login and starts Junk automatically.
///
/// On Windows: writes/removes a value in
///   HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
///
/// On Linux: creates/removes ~/.config/autostart/junk.desktop
#[tauri::command]
fn set_launch_at_login(app: AppHandle, enabled: bool) -> Result<(), String> {
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

/// Check GitHub releases API for a newer version of Junk.
///
/// This is a Rust-side HTTP fetch (via tauri-plugin-http / reqwest) rather
/// than a JS fetch() for two reasons:
///
///   1. Reliability: The WebView's fetch() can be blocked by the Content
///      Security Policy set on the webview. The Tauri HTTP plugin makes
///      the request from the Rust process, bypassing the WebView CSP
///      entirely. This is why "Check for updates" was failing before v2.5.0.
///
///   2. Version access: Rust has compile-time access to CARGO_PKG_VERSION
///      (the version string from Cargo.toml) without any IPC round-trip.
///
/// Response schema:
///   { "up_to_date": bool, "current": "2.5.0", "latest": "2.5.0", "url": "..." }
///
/// The `url` field points to the releases page so the frontend can render a
/// "Download v2.x.x" link directly.
#[tauri::command]
async fn check_for_update() -> Result<UpdateResult, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let api_url = "https://api.github.com/repos/paulfxyz/junk/releases/latest";
    let releases_page = "https://github.com/paulfxyz/junk/releases".to_string();

    // Use tauri-plugin-http's reqwest client.
    // We must set a User-Agent — GitHub API rejects requests without one.
    let client = tauri_plugin_http::reqwest::Client::builder()
        .user_agent(format!("Junk/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| format!("HTTP client build failed: {e}"))?;

    let resp = client
        .get(api_url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned {}", resp.status()));
    }

    // tauri_plugin_http's reqwest re-export doesn't enable the `json` feature,
    // so we read the body as text and parse it ourselves with serde_json.
    let text = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let body: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse failed: {e}"))?;

    // tag_name is "v2.5.0" — strip the leading "v" for semver comparison
    let tag = body
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches('v')
        .to_string();

    if tag.is_empty() {
        return Err("Could not parse tag_name from GitHub response".to_string());
    }

    // Simple semver comparison: compare the trimmed version strings.
    // For Junk's versioning scheme (major.minor.patch, no pre-release suffixes)
    // this is reliable. A full semver library would be overkill for 3 integers.
    let up_to_date = tag == current;
    let url = if up_to_date {
        releases_page.clone()
    } else {
        format!("https://github.com/paulfxyz/junk/releases/tag/v{}", tag)
    };

    log::info!("Update check: current={current} latest={tag} up_to_date={up_to_date}");

    Ok(UpdateResult {
        up_to_date,
        current,
        latest: tag,
        url,
    })
}

// ── Window visibility helpers ─────────────────────────────────────────────────

/// Toggle the main window between visible and hidden.
/// Called by the ⌘J / Ctrl+J global shortcut and the tray icon click.
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

// ── System tray / menu bar (v2.5.0) ──────────────────────────────────────────

/// Build and register the system tray icon with its context menu.
///
/// macOS: appears in the menu bar (right side, next to the clock).
/// Windows: appears in the notification area (system tray, bottom-right).
/// Linux: appears in the status area (compositor-dependent).
///
/// Menu structure:
///   Show / Hide Junk      ← toggles the scratchpad window
///   ─────────────────
///   Preferences           ← opens the prefs panel (same as ⌘,)
///   Check for Updates     ← triggers async update check
///   ─────────────────
///   Quit Junk             ← the ONLY way to truly quit the process
///
/// Why add a tray icon when we already have no Dock icon?
///   Without a Dock icon or tray icon, Junk is completely invisible to the OS
///   UI once the window is hidden. The tray icon provides:
///     1. Visual confirmation the app is running (users know it's alive)
///     2. A fallback to show/hide when the global shortcut fails or conflicts
///     3. A clear, discoverable "Quit" action (vs. Activity Monitor)
///     4. Quick access to Preferences without remembering ⌘,
fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    // ── Tray menu items ────────────────────────────────────────────────────

    // "Show / Hide Junk" — toggles the main window
    let show_hide = MenuItem::with_id(app, "show_hide", "Show / Hide Junk", true, None::<&str>)?;

    // Separator
    let sep1 = PredefinedMenuItem::separator(app)?;

    // "Preferences" — opens the prefs panel
    let prefs = MenuItem::with_id(app, "prefs", "Preferences", true, Some("CmdOrCtrl+,"))?;

    // "Check for Updates" — triggers the async update check and shows a dialog
    let updates = MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?;

    // Separator
    let sep2 = PredefinedMenuItem::separator(app)?;

    // "Quit Junk" — the only true exit point. We use a custom item (not
    // PredefinedMenuItem::quit) so we can give it the "Junk" app name.
    let quit = MenuItem::with_id(app, "quit", "Quit Junk", true, None::<&str>)?;

    // Build the menu
    let menu = Menu::with_items(app, &[
        &show_hide,
        &sep1,
        &prefs,
        &updates,
        &sep2,
        &quit,
    ])?;

    // ── Load the tray icon ─────────────────────────────────────────────────
    // We use the app icon (16×16 or 32×32 PNG). Tauri loads it from the
    // icon path configured in tauri.conf.json's bundle.icon array.
    // Image::from_path loads PNG directly from the resource directory.
    //
    // On macOS, menu bar icons should be:
    //   - 16×16 or 22×22 logical pixels
    //   - Template images (dark/light adaptive) ideally, or a simple monochrome icon
    // We use the app's existing icon — not ideal for menu bar but functional.
    // app.default_window_icon() returns Option<&Image> — clone it for the tray.
    // If no icon is configured (shouldn't happen — tauri.conf.json always ships
    // icons), return early gracefully rather than crashing.
    let icon = match app.default_window_icon() {
        Some(i) => i.clone(),
        None => {
            log::warn!("No default window icon found — tray icon will be empty");
            return Ok(());
        }
    };

    // ── Build the tray ─────────────────────────────────────────────────────
    let app_handle = app.clone();
    let app_handle2 = app.clone();

    TrayIconBuilder::with_id("junk-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("Junk — press ⌘J to toggle")
        // Left-click on the tray icon toggles the window (macOS: click, not right-click)
        .on_tray_icon_event(move |_tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(window) = app_handle.get_webview_window("main") {
                    toggle_window(&window);
                }
            }
        })
        // Menu item click handler
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        toggle_window(&window);
                    }
                }
                "prefs" => {
                    // Show window first (if hidden), then emit open-prefs event
                    if let Some(window) = app.get_webview_window("main") {
                        if let Ok(false) = window.is_visible() {
                            show_and_focus(&window);
                        }
                        let _ = window.emit("open-prefs", ());
                    }
                }
                "check_updates" => {
                    // Trigger the update check from the tray.
                    // We spawn a thread to avoid blocking the menu event handler.
                    // The result is emitted back to JS as a 'update-result' event.
                    let app2 = app.clone();
                    tauri::async_runtime::spawn(async move {
                        match check_for_update().await {
                            Ok(result) => {
                                if let Some(window) = app2.get_webview_window("main") {
                                    // Make sure window is visible for the result
                                    if let Ok(false) = window.is_visible() {
                                        show_and_focus(&window);
                                    }
                                    let _ = window.emit("update-result", &result);
                                }
                            }
                            Err(e) => {
                                log::error!("Tray update check failed: {e}");
                            }
                        }
                    });
                }
                "quit" => {
                    // This is the ONLY path that truly exits the process.
                    // ⌘Q and the × button are intercepted and converted to hide().
                    log::info!("Quit requested via tray menu — exiting.");
                    app_handle2.exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;

    log::info!("System tray icon registered.");
    Ok(())
}

// ── Global shortcut registration ──────────────────────────────────────────────

/// Register ⌘J (macOS) / Ctrl+J (Windows/Linux) as a global toggle shortcut.
///
/// IMPORTANT: never combine SUPER|CONTROL — on macOS that requires both
/// ⌘ and Ctrl simultaneously (⌘^J). Always use exactly one modifier per
/// platform. This was the v2.0.0 bug.
fn register_toggle_shortcut(app: &AppHandle) -> Result<(), String> {
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
/// OS-level shortcut bypasses the WebView entirely — more reliable than JS.
/// Only hides if the window is currently visible (stray Esc presses from
/// other apps while Junk is hidden are silent no-ops).
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
/// Works even when the window is hidden — we show + focus it before emitting.
fn register_prefs_shortcut(app: &AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER;

    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;

    let shortcut = Shortcut::new(Some(modifiers), Code::Comma);
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let Some(window) = app_handle.get_webview_window("main") else {
                    return;
                };
                if let Ok(false) = window.is_visible() {
                    show_and_focus(&window);
                }
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

/// Remove Junk from the macOS Dock and ⌘-Tab app switcher.
///
/// Uses `Accessory` policy — hides Dock icon but still receives keyboard focus
/// and can have windows. Must be called before any window is shown to avoid
/// a flash of the Dock icon on startup.
///
/// Note: with a tray icon, the macOS menu bar still shows Junk's icon — only
/// the Dock entry is hidden. This is the correct behaviour for utility apps.
#[cfg(target_os = "macos")]
fn set_macos_activation_policy(app: &AppHandle) {
    use tauri::ActivationPolicy;
    match app.set_activation_policy(ActivationPolicy::Accessory) {
        Ok(()) => log::info!("macOS activation policy → Accessory (no Dock icon)"),
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
        // Global shortcut plugin — registers OS-level hotkeys
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Autostart plugin — MacosLauncher::LaunchAgent writes a per-user
        // LaunchAgent plist (~/Library/LaunchAgents/). No root required.
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None::<Vec<&str>>,
        ))
        // HTTP plugin — allows Rust to make outbound HTTPS requests.
        // Used by check_for_update() to fetch api.github.com from Rust,
        // bypassing the WebView's Content Security Policy.
        .plugin(tauri_plugin_http::init())
        // ── Setup hook ────────────────────────────────────────────────────────
        .setup(|app| {
            let handle = app.handle();

            // macOS: hide Dock icon before any window appears
            #[cfg(target_os = "macos")]
            set_macos_activation_policy(handle);

            // Register OS-level global shortcuts
            if let Err(e) = register_toggle_shortcut(handle) {
                log::warn!("⌘J shortcut registration failed: {e}");
                log::warn!("Another app may own CmdOrCtrl+J — use the tray icon instead.");
            }
            if let Err(e) = register_esc_shortcut(handle) {
                log::warn!("Esc shortcut registration failed: {e}");
            }
            if let Err(e) = register_prefs_shortcut(handle) {
                log::warn!("Prefs shortcut (⌘,) registration failed: {e}");
            }

            // Set up the system tray / menu bar icon (v2.5.0)
            if let Err(e) = setup_tray(handle) {
                log::error!("Tray setup failed: {e}");
            }

            log::info!("Setup complete. Junk v{} is ready.", env!("CARGO_PKG_VERSION"));
            Ok(())
        })
        // ── IPC commands ──────────────────────────────────────────────────────
        // All commands are callable from JS via invoke('command_name', args).
        // check_for_update is async — it makes an HTTP request.
        .invoke_handler(tauri::generate_handler![
            hide_window,
            open_prefs,
            get_prefs,
            set_launch_at_login,
            check_for_update,
        ])
        // ── Persistent process — intercept close/quit events ──────────────────
        //
        // Junk never exits via normal close/quit paths. The process stays alive
        // so the global shortcut remains registered. The tray's "Quit Junk"
        // item is the only true exit path.
        //
        // Pattern used by: Alfred, Paste, Magnet, Rectangle, Raycast.
        .build(tauri::generate_context!())
        .expect("Fatal: Tauri failed to start.")
        .run(|app, event| match event {
            // Window × button or ⌘W → hide instead of close
            RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::CloseRequested { api, .. },
                ..
            } if label == "main" => {
                api.prevent_close();
                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.hide() {
                        log::error!("CloseRequested: hide failed: {e}");
                    }
                }
            }

            // ⌘Q on macOS → hide instead of quit
            // Note: the tray "Quit Junk" item calls app.exit(0) directly,
            // bypassing this handler — that's the only true exit path.
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
                if let Some(window) = app.get_webview_window("main") {
                    if let Ok(true) = window.is_visible() {
                        let _ = window.hide();
                    }
                }
            }

            _ => {}
        });
}
