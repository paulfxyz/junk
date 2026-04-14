// ─────────────────────────────────────────────────────────────────────────────
// Junk — a global-hotkey scratchpad built with Tauri v2
//
// Architecture overview (v2.6.0)
// ─────────────────────────────────────────────────────────────────────────────
//
// ┌──────────────────────────────────────────────────────────────────────────┐
// │  OS Global Shortcuts (registered at OS level — bypass WebView entirely)  │
// │                                                                          │
// │  ⌘J / Ctrl+J  → toggle_window()        — show or hide the scratchpad    │
// │  Escape        → hide_if_visible()      — always dismiss                 │
// │  ⌘, / Ctrl+,  → show_prefs_in_window() — open preferences panel         │
// │                                                                          │
// │  Window dragging (v2.6.0)                                                │
// │    The entire window surface is a drag region (-webkit-app-region:drag)  │
// │    Interactive elements (textarea, buttons, links) override with         │
// │    no-drag so clicks still register normally. This means you can drag    │
// │    Junk from the header, the empty margins, or anywhere that isn't       │
// │    the text editing area — natural and zero-JS.                          │
// │                                                                          │
// │  Window lifecycle                                                        │
// │    close button (×) → hide()   — process stays alive, shortcut works    │
// │    ⌘Q              → exit(0)  — quit for real (v2.6.0 behaviour)        │
// │    Esc              → hide()   — via OS shortcut                         │
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

use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewWindow};
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
/// Serialised as JSON and sent to JS:
///   { "up_to_date": true,  "current": "2.6.0", "latest": "2.6.0", "url": "..." }
///   { "up_to_date": false, "current": "2.5.0", "latest": "2.6.0", "url": "..." }
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
/// Called from JS via `invoke('hide_window')`. Thin IPC wrapper around
/// `window.hide()` — JS cannot call native window APIs directly.
#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window.hide().map_err(|e| e.to_string())
}

/// Open the preferences panel in the frontend.
///
/// 1. Shows and focuses the window if it was hidden (⌘, works globally).
/// 2. Emits the 'open-prefs' Tauri event — JS slides the panel up.
#[tauri::command]
fn open_prefs(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    if let Ok(false) = window.is_visible() {
        show_and_focus(&window);
    }
    window
        .emit("open-prefs", ())
        .map_err(|e: tauri::Error| e.to_string())
}

/// Return the current preference values to the frontend.
///
/// Reads real OS state on every call — accurate even if the user changed
/// the login item via System Settings since the app last ran.
#[tauri::command]
fn get_prefs(app: AppHandle) -> Result<Prefs, String> {
    let launch_at_login = app.autolaunch().is_enabled().unwrap_or(false);
    Ok(Prefs { launch_at_login })
}

/// Enable or disable the OS login item for Junk.
///
/// macOS:   creates/removes ~/Library/LaunchAgents/<bundle-id>.plist
/// Windows: writes/removes HKCU\...\Microsoft\Windows\CurrentVersion\Run
/// Linux:   creates/removes ~/.config/autostart/junk.desktop
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
/// HTTP fetch is done from Rust (tauri-plugin-http / reqwest) rather than JS:
///   - JS fetch() can be blocked by the WebView Content Security Policy.
///   - Rust has compile-time access to CARGO_PKG_VERSION, no extra IPC needed.
///
/// Returns UpdateResult: { up_to_date, current, latest, url }
#[tauri::command]
async fn check_for_update() -> Result<UpdateResult, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let api_url = "https://api.github.com/repos/paulfxyz/junk/releases/latest";
    let releases_page = "https://github.com/paulfxyz/junk/releases".to_string();

    // Build a reqwest client with the required User-Agent header.
    // GitHub API rejects requests without a User-Agent with 403.
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
    // so we read the body as text and parse it with serde_json.
    let text = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let body: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse failed: {e}"))?;

    // tag_name is "v2.6.0" — strip the leading "v" for semver comparison
    let tag = body
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches('v')
        .to_string();

    if tag.is_empty() {
        return Err("Could not parse tag_name from GitHub response".to_string());
    }

    // Simple version string comparison — reliable for major.minor.patch
    // without pre-release suffixes. Full semver library would be overkill.
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
/// Called by the ⌘J / Ctrl+J global shortcut.
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
/// IMPORTANT: never combine SUPER|CONTROL — on macOS that would require both
/// ⌘ and Ctrl simultaneously. Always use exactly one modifier per platform.
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
/// OS-level bypasses the WebView entirely — more reliable than JS.
/// Only hides if the window is visible — stray Esc presses while hidden
/// are silent no-ops.
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
/// Shows and focuses the window first if it was hidden, so the prefs panel
/// is always reachable via keyboard even when Junk is in the background.
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
/// `Accessory` policy: hides Dock icon, still receives keyboard focus and
/// can have windows. Must be called before any window appears to avoid a
/// flash of the Dock icon on startup.
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
        // Autostart plugin — LaunchAgent plist on macOS (per-user, no root needed)
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None::<Vec<&str>>,
        ))
        // HTTP plugin — used by check_for_update() to fetch api.github.com
        // from Rust, bypassing the WebView Content Security Policy.
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
            }
            if let Err(e) = register_esc_shortcut(handle) {
                log::warn!("Esc shortcut registration failed: {e}");
            }
            if let Err(e) = register_prefs_shortcut(handle) {
                log::warn!("Prefs shortcut (⌘,) registration failed: {e}");
            }

            log::info!("Setup complete. Junk v{} is ready.", env!("CARGO_PKG_VERSION"));
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
        // ── Window event handling ─────────────────────────────────────────────
        .build(tauri::generate_context!())
        .expect("Fatal: Tauri failed to start.")
        .run(|app, event| match event {
            // Window × button → hide instead of close.
            // The process stays alive so the global shortcut remains registered.
            // This mirrors how Alfred, Paste, and Magnet behave on macOS.
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

            // ⌘Q → quit for real (v2.6.0).
            // In v2.5.0 ⌘Q was intercepted and converted to hide() because
            // the tray "Quit Junk" item was the only exit path.
            // Now that the tray is gone, ⌘Q is the natural quit gesture again.
            // We still get the ExitRequested event and call app.exit(0) explicitly
            // so the shutdown is clean and logged.
            RunEvent::ExitRequested { .. } => {
                log::info!("ExitRequested (⌘Q) — shutting down.");
                app.exit(0);
            }

            _ => {}
        });
}
