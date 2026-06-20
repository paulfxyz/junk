// ─────────────────────────────────────────────────────────────────────────────
// Junk — a global-hotkey scratchpad built with Tauri v2
//
// Architecture overview (v3.0.4)
// ─────────────────────────────────────────────────────────────────────────────
//
// ┌──────────────────────────────────────────────────────────────────────────┐
// │  OS Global Shortcuts (registered at OS level — bypass WebView entirely)  │
// │                                                                          │
// │  ⌘J / Ctrl+J        → toggle_window()   — show or hide the scratchpad   │
// │  Escape              → hide_if_visible() — always dismiss                │
// │  ⌘, / Ctrl+,        → open preferences panel                            │
// │                                                                          │
// │  Custom shortcut (v3.0.0):                                               │
// │    The user can change the toggle key via Preferences. The new shortcut  │
// │    key is stored in localStorage on the JS side, sent to Rust via        │
// │    invoke('set_hotkey', {key, modifier}), which unregisters the old      │
// │    shortcut and registers the new one at runtime. No restart needed.     │
// │                                                                          │
// │  Window position memory (v3.0.0):                                        │
// │    get_window_position() / set_window_position() IPC commands let JS     │
// │    read and write the window's screen coordinates. On show, JS calls     │
// │    set_window_position to restore the last-dragged position.             │
// │    Position is stored in localStorage on the JS side — no extra Rust     │
// │    state needed.                                                         │
// │                                                                          │
// │  Window lifecycle                                                        │
// │    close button (×) → hide()   — process stays alive, shortcut works    │
// │    ⌘Q              → exit(0)  — quit for real                           │
// │    Esc              → hide()   — via OS shortcut                         │
// │                                                                          │
// │  IPC commands (v3.0.0)                                                   │
// │    hide_window()              → hides the main window                    │
// │    start_dragging()           → initiates native OS window drag          │
// │    open_prefs()               → emits 'open-prefs' event to JS           │
// │    get_prefs()                → { launch_at_login }                      │
// │    set_launch_at_login(bool)  → enables/disables OS login item           │
// │    check_for_update()         → Rust HTTP → GitHub API → JSON            │
// │    get_window_position()      → { x, y } physical screen coords          │
// │    set_window_position(x, y)  → moves window to (x, y) on screen        │
// │    set_hotkey(key, modifier)  → re-registers the toggle shortcut         │
// └──────────────────────────────────────────────────────────────────────────┘
//
//
// IMPORTANT LESSONS LEARNED (v3.0.4 → v3.0.9):
//
// 1. THE DEEP-COMMENT BUG (v3.0.4 → v3.0.8):
//    Adding extensive inline comments to index.html, bloating it from
//    ~1567 to 2067 lines, caused 'Can\'t find variable: windowEl' in
//    WebKit/WKWebView at runtime. The file passed Node.js syntax checks
//    perfectly — the failure was silent until the app was launched.
//    Hypothesis: JavaScriptCore has a different compilation path for large
//    ES modules that does not hoist variables the same way V8 does.
//    Fix: revert to the pre-comment file (commit 4a18c56, 1567 lines).
//    Rule: keep index.html under ~1700 lines.
//
// 2. ALWAYS ON TOP REGRESSION (v3.0.2 → v3.0.9):
//    The v3.0.2 visual rework (frosted glass, rounded corners, shadow)
//    changed alwaysOnTop from true to false in tauri.conf.json. This
//    silently broke the core scratchpad UX — Junk no longer floated
//    above other windows. tauri.conf only sets the INITIAL window level;
//    runtime changes require set_always_on_top() IPC. Restored in v3.0.9
//    with a Preferences toggle (default ON, localStorage-persisted).
//
// 3. box-shadow vs native shadow (v3.0.3 → v3.0.4):
//    masksToBounds=YES (required for macOS rounded corners) clips the
//    entire compositor subtree — including any CSS box-shadow drawn
//    outside the window bounds. Use 'shadow: true' in tauri.conf.json
//    to get the OS-level WindowServer shadow that sits below the clip.
//
// ─────────────────────────────────────────────────────────────────────────────
// Why no unwrap() in production paths?
// Tauri window operations return Result. In a shortcut callback there is no
// caller to propagate errors to, so we log them instead of panicking. A panic
// in a callback crashes the entire app — a log message is always better.
// ─────────────────────────────────────────────────────────────────────────────

// Silence the console window that Windows would normally open for a GUI app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewWindow};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt as AutostartExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// Native window effects.
// macOS strategy (two-step):
//   1. apply_vibrancy — adds NSVisualEffectView for frosted-glass background.
//      Its corner_radius only rounds the blur subview, NOT the window frame.
//   2. set_macos_window_corner_radius — walks nsView → NSWindow → contentView
//      → CALayer and sets cornerRadius + masksToBounds so the compositor
//      actually clips everything (WKWebView included) to rounded corners.
// Windows: Acrylic blur effect (Windows 10/11).
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;

// ── Shared state ──────────────────────────────────────────────────────────────

/// Holds the currently registered toggle shortcut so we can unregister it
/// when the user changes it from Preferences. Wrapped in Mutex because the
/// Tauri state system requires Send + Sync.
struct CurrentShortcut(Mutex<Shortcut>);

// ── Preferences types ─────────────────────────────────────────────────────────

/// The preferences struct returned to the JS frontend via `get_prefs()`.
#[derive(serde::Serialize)]
struct Prefs {
    /// Whether the OS login item is currently enabled.
    /// Read from the OS on every call (not cached).
    launch_at_login: bool,
}

/// Update check result returned from `check_for_update()`.
#[derive(serde::Serialize)]
struct UpdateResult {
    up_to_date: bool,
    current: String,
    latest: String,
    url: String,
}

/// Window position returned from `get_window_position()`.
/// Physical screen coordinates, top-left origin.
#[derive(serde::Serialize)]
struct WindowPosition {
    x: i32,
    y: i32,
}

/// Window size returned from `get_window_size()`.
/// Physical pixel dimensions.
#[derive(serde::Serialize, serde::Deserialize)]
struct WindowSize {
    width: u32,
    height: u32,
}

// ── IPC commands ──────────────────────────────────────────────────────────────

/// Hide the main scratchpad window.
///
/// Called from JS via `invoke('hide_window')`. Thin wrapper around
/// `window.hide()` — JS cannot call native window APIs directly.
#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window.hide().map_err(|e| e.to_string())
}

/// Initiate a native OS window drag.
///
/// The CORRECT way to drag a frameless transparent Tauri window on macOS.
/// See the architecture deep-dive in README for the full story of how we
/// arrived at this approach (CSS ignored → plugin namespace → timing fix).
///
/// Key insight: `e.preventDefault()` must be called in JS BEFORE the async
/// invoke() so the OS keeps its drag-candidate state open until this Rust
/// function executes and calls NSWindow.performWindowDragWithEvent:.
#[tauri::command]
fn start_dragging(window: WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
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
///   - JS window.fetch() can be blocked by the WebView Content Security Policy.
///   - Rust has compile-time access to CARGO_PKG_VERSION via env!().
///
/// GOTCHA: tauri_plugin_http's reqwest re-export does NOT enable the `json`
/// feature, so resp.json::<T>().await fails to compile. Always use
/// .text().await + serde_json::from_str() instead.
#[tauri::command]
async fn check_for_update() -> Result<UpdateResult, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let api_url = "https://api.github.com/repos/paulfxyz/junk/releases/latest";
    let releases_page = "https://github.com/paulfxyz/junk/releases".to_string();

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

    let text = resp
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    let body: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse failed: {e}"))?;

    let tag = body
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim_start_matches('v')
        .to_string();

    if tag.is_empty() {
        return Err("Could not parse tag_name from GitHub response".to_string());
    }

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

/// Return the current window position as physical screen coordinates.
///
/// Called from JS on startup to restore the last remembered position.
///
/// Why physical coordinates?
///   `window.outer_position()` returns PhysicalPosition — the actual pixel
///   offset from the top-left of the primary monitor. This is consistent
///   across DPI scaling changes, which LogicalPosition is not. We store
///   physical coords and restore with set_window_position(), so the window
///   always lands exactly where the user put it.
#[tauri::command]
fn get_window_position(app: AppHandle) -> Result<WindowPosition, String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let pos = window
        .outer_position()
        .map_err(|e| format!("get_window_position failed: {e}"))?;
    Ok(WindowPosition { x: pos.x, y: pos.y })
}

/// Move the window to absolute screen coordinates (physical pixels).
///
/// Called from JS after restoring a saved position from localStorage.
/// Uses `set_position(PhysicalPosition)` so the result is pixel-exact
/// regardless of display scale factor.
///
/// Edge case handled in JS: if the saved position is off-screen (e.g.
/// the user moved to a smaller monitor since last session), JS falls back
/// to centering by calling set_window_position with centered coords.
#[tauri::command]
fn set_window_position(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window
        .set_position(tauri::PhysicalPosition::new(x, y))
        .map_err(|e| format!("set_window_position({x},{y}) failed: {e}"))
}

/// Return the current window size in physical pixels.
#[tauri::command]
fn get_window_size(app: AppHandle) -> Result<WindowSize, String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let size = window
        .outer_size()
        .map_err(|e| format!("get_window_size failed: {e}"))?;
    Ok(WindowSize { width: size.width, height: size.height })
}

/// Resize the window to the given physical pixel dimensions.
/// Clamps to a safe minimum (300×200) and reasonable maximum (3000×2000).
#[tauri::command]
fn set_window_size(app: AppHandle, width: u32, height: u32) -> Result<(), String> {
    let w = width.clamp(300, 3000);
    let h = height.clamp(200, 2000);
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window
        .set_size(tauri::PhysicalSize::new(w, h))
        .map_err(|e| format!("set_window_size({w},{h}) failed: {e}"))
}

/// Enable or disable the "always on top" window level.
///
/// Junk is a scratchpad — staying on top of other windows is its core UX.
/// This was the default behaviour before v3.0.2 (when `alwaysOnTop: true` was
/// set in tauri.conf.json). The aesthetic rework changed that to `false` to
/// allow the native shadow to render correctly, but the feature itself must
/// be available via the toggle in Preferences.
///
/// The JS side stores the user's preference in localStorage and calls this
/// on startup to restore it.
#[tauri::command]
fn set_always_on_top(app: AppHandle, always_on_top: bool) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    window
        .set_always_on_top(always_on_top)
        .map_err(|e| format!("set_always_on_top({always_on_top}) failed: {e}"))
}


/// Re-register the toggle shortcut to a new key combination.
///
/// Called from JS when the user changes the hotkey in Preferences.
///
/// Flow:
///   1. Parse the key name string into a tauri_plugin_global_shortcut::Code
///   2. Unregister the currently registered shortcut (stored in app state)
///   3. Register the new shortcut with the same platform modifier (⌘/Ctrl)
///   4. Update the stored shortcut in app state
///
/// Why store the current shortcut in state?
///   GlobalShortcutExt::unregister() requires the exact Shortcut struct that
///   was registered. We can't reconstruct it from a string, so we keep the
///   last-registered struct in a Mutex<Shortcut> managed by Tauri's state.
///
/// Key name format: "KeyJ", "KeyK", "Space", "F1" — matches
/// tauri_plugin_global_shortcut::Code variant names exactly.
/// If the key name is unrecognised we return Err so JS can show an error.
#[tauri::command]
fn set_hotkey(
    app: AppHandle,
    key: String,
    current_shortcut_state: tauri::State<CurrentShortcut>,
) -> Result<(), String> {
    // Parse the key name into a Code variant.
    // We support a curated subset of keys that make sense as hotkeys.
    // The Code enum has ~180 variants — we map the most useful ones.
    let code = parse_key_code(&key)?;

    // Platform modifier: ⌘ on macOS, Ctrl on Windows/Linux
    #[cfg(target_os = "macos")]
    let modifiers = Modifiers::SUPER;
    #[cfg(not(target_os = "macos"))]
    let modifiers = Modifiers::CONTROL;

    let new_shortcut = Shortcut::new(Some(modifiers), code);

    // Unregister the previous shortcut. Lock the mutex to read its value.
    {
        let old = current_shortcut_state.0.lock().map_err(|e| e.to_string())?;
        if let Err(e) = app.global_shortcut().unregister(*old) {
            // Non-fatal: if the old shortcut wasn't registered (e.g. first run),
            // we just log and continue. The new one will still be registered.
            log::warn!("set_hotkey: unregister old shortcut failed: {e}");
        }
    }

    // Register the new shortcut with the same toggle handler.
    let app_handle = app.clone();
    app.global_shortcut()
        .on_shortcut(new_shortcut, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                match app_handle.get_webview_window("main") {
                    Some(window) => toggle_window(&window),
                    None => log::error!("toggle shortcut: 'main' window not found"),
                }
            }
        })
        .map_err(|e| format!("Failed to register new shortcut: {e}"))?;

    // Update the stored current shortcut.
    let mut current = current_shortcut_state.0.lock().map_err(|e| e.to_string())?;
    *current = new_shortcut;

    log::info!("Hotkey changed to {modifiers:?}+{key}");
    Ok(())
}

/// Parse a key name string into a Code variant.
///
/// JS sends the key as a string like "KeyJ", "KeyK", "Space", "F1".
/// We map a curated safe subset — enough for a hotkey picker, not all 180 variants.
fn parse_key_code(key: &str) -> Result<Code, String> {
    match key {
        "KeyA" => Ok(Code::KeyA), "KeyB" => Ok(Code::KeyB), "KeyC" => Ok(Code::KeyC),
        "KeyD" => Ok(Code::KeyD), "KeyE" => Ok(Code::KeyE), "KeyF" => Ok(Code::KeyF),
        "KeyG" => Ok(Code::KeyG), "KeyH" => Ok(Code::KeyH), "KeyI" => Ok(Code::KeyI),
        "KeyJ" => Ok(Code::KeyJ), "KeyK" => Ok(Code::KeyK), "KeyL" => Ok(Code::KeyL),
        "KeyM" => Ok(Code::KeyM), "KeyN" => Ok(Code::KeyN), "KeyO" => Ok(Code::KeyO),
        "KeyP" => Ok(Code::KeyP), "KeyQ" => Ok(Code::KeyQ), "KeyR" => Ok(Code::KeyR),
        "KeyS" => Ok(Code::KeyS), "KeyT" => Ok(Code::KeyT), "KeyU" => Ok(Code::KeyU),
        "KeyV" => Ok(Code::KeyV), "KeyW" => Ok(Code::KeyW), "KeyX" => Ok(Code::KeyX),
        "KeyY" => Ok(Code::KeyY), "KeyZ" => Ok(Code::KeyZ),
        "Digit0" => Ok(Code::Digit0), "Digit1" => Ok(Code::Digit1),
        "Digit2" => Ok(Code::Digit2), "Digit3" => Ok(Code::Digit3),
        "Digit4" => Ok(Code::Digit4), "Digit5" => Ok(Code::Digit5),
        "Digit6" => Ok(Code::Digit6), "Digit7" => Ok(Code::Digit7),
        "Digit8" => Ok(Code::Digit8), "Digit9" => Ok(Code::Digit9),
        "F1"  => Ok(Code::F1),  "F2"  => Ok(Code::F2),  "F3"  => Ok(Code::F3),
        "F4"  => Ok(Code::F4),  "F5"  => Ok(Code::F5),  "F6"  => Ok(Code::F6),
        "F7"  => Ok(Code::F7),  "F8"  => Ok(Code::F8),  "F9"  => Ok(Code::F9),
        "F10" => Ok(Code::F10), "F11" => Ok(Code::F11), "F12" => Ok(Code::F12),
        "Space"      => Ok(Code::Space),
        "Backquote"  => Ok(Code::Backquote),
        "BracketLeft"  => Ok(Code::BracketLeft),
        "BracketRight" => Ok(Code::BracketRight),
        "Semicolon"  => Ok(Code::Semicolon),
        "Quote"      => Ok(Code::Quote),
        "Backslash"  => Ok(Code::Backslash),
        "Slash"      => Ok(Code::Slash),
        "Minus"      => Ok(Code::Minus),
        "Equal"      => Ok(Code::Equal),
        "Period"     => Ok(Code::Period),
        _ => Err(format!("Unrecognised key code: '{key}'. Use standard KeyboardEvent.code names (e.g. KeyJ, F1, Space).")),
    }
}

// ── Window visibility helpers ─────────────────────────────────────────────────

/// Toggle the main window between visible and hidden.
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

/// Register the toggle shortcut (default: ⌘J on macOS, Ctrl+J elsewhere).
///
/// Returns the registered Shortcut so it can be stored in app state and
/// later unregistered when the user changes it via set_hotkey().
///
/// IMPORTANT: never combine SUPER|CONTROL — on macOS that would require both
/// ⌘ and Ctrl simultaneously. Always use exactly one modifier per platform.
fn register_toggle_shortcut(app: &AppHandle) -> Result<Shortcut, String> {
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
        })?;

    Ok(shortcut)
}

/// Register Escape as an OS-level shortcut to hide the window.
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
/// `Accessory` policy: hides Dock icon, still receives keyboard focus.
/// Must be called before any window appears to avoid a flash of the icon.
#[cfg(target_os = "macos")]
fn set_macos_activation_policy(app: &AppHandle) {
    use tauri::ActivationPolicy;
    match app.set_activation_policy(ActivationPolicy::Accessory) {
        Ok(()) => log::info!("macOS activation policy → Accessory (no Dock icon)"),
        Err(e) => log::warn!("Could not set activation policy: {e}"),
    }
}

// ── macOS: set rounded corners on the NSWindow contentView CALayer ───────────
//
// Why this is needed:
//   apply_vibrancy() adds an NSVisualEffectView subview and rounds IT, but the
//   NSWindow frame remains rectangular. The WKWebView sits ABOVE the blur view
//   and is still square-clipped by the window frame, so CSS alone can't fix it.
//
// What this does:
//   nsView (WKWebView) → [nsView window] → NSWindow
//                      → [window contentView] → NSView (content view)
//                      → [contentView layer] → CALayer
//   Sets layer.cornerRadius = 14.0 and layer.masksToBounds = YES.
//   masksToBounds is what does the actual pixel clip at the compositor level.
//
// Safety: all pointers are non-null (guarded); called on the main thread.
#[cfg(target_os = "macos")]
unsafe fn set_macos_window_corner_radius(window: &tauri::WebviewWindow, radius: f64) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    let Ok(handle) = window.window_handle() else { return };
    let RawWindowHandle::AppKit(appkit) = handle.as_raw() else { return };

    // ns_view is the WKWebView — a NonNull<c_void> pointer to the NSView.
    let ns_view: *mut AnyObject = appkit.ns_view.as_ptr().cast();

    // 1. Walk up to the NSWindow that owns this view.
    let ns_window: *mut AnyObject = msg_send![ns_view, window];
    if ns_window.is_null() {
        log::warn!("set_macos_window_corner_radius: [nsView window] returned nil");
        return;
    }

    // 2. Get the NSWindow's contentView (the root view of the window).
    let content_view: *mut AnyObject = msg_send![ns_window, contentView];
    if content_view.is_null() {
        log::warn!("set_macos_window_corner_radius: [nsWindow contentView] returned nil");
        return;
    }

    // 3. Enable layer-backing on the content view (idempotent if already set).
    let () = msg_send![content_view, setWantsLayer: true];

    // 4. Grab the backing CALayer.
    let layer: *mut AnyObject = msg_send![content_view, layer];
    if layer.is_null() {
        log::warn!("set_macos_window_corner_radius: [contentView layer] returned nil");
        return;
    }

    // 5. Set corner radius — this alone doesn't clip; we also need masksToBounds.
    // CGFloat = f64 on all 64-bit Apple platforms; cast explicitly to be safe.
    let () = msg_send![layer, setCornerRadius: radius as f64];

    // 6. masksToBounds = YES — this is what actually clips child layers to the
    //    rounded rect. Without it, cornerRadius has no visual effect.
    let () = msg_send![layer, setMasksToBounds: true];

    log::info!("macOS contentView layer cornerRadius={radius} masksToBounds=YES");
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "junk=debug,tauri=info");

    env_logger::init();
    log::info!("Junk v{} starting up", env!("CARGO_PKG_VERSION"));

    // Default shortcut — ⌘J on macOS, Ctrl+J elsewhere.
    // We need this value before setup() runs so we can put it in app state.
    // The actual registration happens in setup(); here we just build the
    // struct so Tauri's manage() system has something to hold.
    #[cfg(target_os = "macos")]
    let default_modifiers = Modifiers::SUPER;
    #[cfg(not(target_os = "macos"))]
    let default_modifiers = Modifiers::CONTROL;

    let default_shortcut = Shortcut::new(Some(default_modifiers), Code::KeyJ);

    tauri::Builder::default()
        // ── App state ─────────────────────────────────────────────────────────
        // Store the currently-registered toggle shortcut so set_hotkey() can
        // unregister it before registering the new one.
        .manage(CurrentShortcut(Mutex::new(default_shortcut)))
        // ── Plugins ───────────────────────────────────────────────────────────
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None::<Vec<&str>>,
        ))
        .plugin(tauri_plugin_http::init())
        // ── Setup hook ────────────────────────────────────────────────────────
        .setup(|app| {
            let handle = app.handle();

            #[cfg(target_os = "macos")]
            set_macos_activation_policy(handle);

            // ── Native window effects ──────────────────────────────────────────
            if let Some(window) = handle.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                {
                    const RADIUS: f64 = 14.0;

                    // Step 1: frosted-glass NSVisualEffectView background.
                    // HudWindow adapts to light/dark automatically.
                    // The corner_radius here only rounds the blur subview itself,
                    // not the window frame — step 2 does that.
                    if let Err(e) = apply_vibrancy(
                        &window,
                        NSVisualEffectMaterial::HudWindow,
                        None,
                        Some(RADIUS),
                    ) {
                        log::warn!("apply_vibrancy failed (non-fatal): {e}");
                    } else {
                        log::info!("macOS vibrancy applied (HudWindow, r={RADIUS})");
                    }

                    // Step 2: set cornerRadius + masksToBounds on the NSWindow's
                    // contentView CALayer. This clips the entire compositor subtree
                    // (including the WKWebView) to the rounded rect. Without this,
                    // the WKWebView renders over the rounded blur corners as a square.
                    unsafe { set_macos_window_corner_radius(&window, RADIUS) };
                }

                #[cfg(target_os = "windows")]
                {
                    // Acrylic effect: semi-transparent blur tinted white.
                    if let Err(e) = apply_acrylic(&window, None) {
                        log::warn!("apply_acrylic failed (non-fatal): {e}");
                    } else {
                        log::info!("Windows acrylic effect applied");
                    }
                }
            }

            // Register OS-level global shortcuts
            match register_toggle_shortcut(handle) {
                Ok(shortcut) => {
                    // Store the registered shortcut so set_hotkey() can replace it later.
                    // We already put a default in state above; overwrite with the
                    // actually-registered one (same value, but proves it succeeded).
                    if let Some(state) = handle.try_state::<CurrentShortcut>() {
                        if let Ok(mut lock) = state.0.lock() {
                            *lock = shortcut;
                        }
                    }
                }
                Err(e) => log::warn!("⌘J shortcut registration failed: {e}"),
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
            start_dragging,
            open_prefs,
            get_prefs,
            set_launch_at_login,
            check_for_update,
            get_window_position,
            set_window_position,
            get_window_size,
            set_window_size,
            set_always_on_top,
            set_hotkey,
        ])
        // ── Window event handling ─────────────────────────────────────────────
        .build(tauri::generate_context!())
        .expect("Fatal: Tauri failed to start.")
        .run(|app, event| match event {
            // Window × button → hide instead of close.
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

            // Focus change → emit custom events to JS for dim-on-blur.
            //
            // WHY custom events instead of tauri://blur / tauri://focus?
            //   On macOS, always-on-top windows (NSWindowLevel above normal)
            //   do not always receive the standard WebKit blur/focus events
            //   when the user switches to another app. Tauri’s Rust-side
            //   WindowEvent::Focused fires reliably regardless of window level
            //   because it maps directly to NSWindowDelegate
            //   windowDidBecomeKey / windowDidResignKey.
            //
            // We emit 'junk://blur' (focused=false) and 'junk://focus-change'
            // (focused=true). JS listens for these instead of 'tauri://blur'.
            // Note: 'junk://focus-change' is separate from 'tauri://focus'
            //   (which fires on show). Both are needed: tauri://focus handles
            //   the show/restore flow; junk://focus-change handles the
            //   dim-restore when switching back from another app.
            RunEvent::WindowEvent {
                label,
                event: tauri::WindowEvent::Focused(focused),
                ..
            } if label == "main" => {
                if let Some(window) = app.get_webview_window("main") {
                    let event_name = if focused { "junk://focus-change" } else { "junk://blur" };
                    if let Err(e) = window.emit(event_name, ()) {
                        log::warn!("Failed to emit {event_name}: {e}");
                    }
                }
            }

            // ⌘Q → quit for real.
            RunEvent::ExitRequested { .. } => {
                log::info!("ExitRequested (⌘Q) — shutting down.");
                app.exit(0);
            }

            _ => {}
        });
}
