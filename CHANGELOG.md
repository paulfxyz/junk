# Changelog

All notable changes to Junk are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow [Semantic Versioning](https://semver.org/).

---

## [3.0.6] — 2026-06-06

### Fix: footer buttons unblocked

#### Fixed
- Markdown toggle, clipboard copy, and settings buttons in the footer were
  unresponsive because the `footer-drag-handle` div (added in v3.0.5) was
  `position:absolute` on top of them and intercepting their clicks.
- Fix: added `pointer-events: none` to `.footer-drag-handle`. Clicks pass
  straight through to buttons underneath. Drag still works because mousedown
  bubbles up through `<footer>` → `#window` where `start_dragging()` is called.

---

## [3.0.5] — 2026-06-06

### Footer drag handle restored

#### Fixed
- Dragging the window from the bottom bar works again. The footer's empty centre zone now has a dedicated invisible drag handle (`div.footer-drag-handle`) with `position: absolute` spanning the full footer height. Clicking it falls through to the `#window` `mousedown` listener which calls `start_dragging()` IPC.
- Added subtle grip-dot indicator in the footer centre so the draggable area is visually discoverable.
- `.footer` gets `position: relative` so the absolute handle is contained correctly.

---

## [3.0.4] — 2026-06-04

### macOS Native Window Shadow

#### Added
- `"shadow": true` in `tauri.conf.json` — macOS WindowServer now draws a native drop shadow outside the window frame.

#### Changed
- Simplified CSS `box-shadow` to a 0.5px edge ring only (decorative boundary, not a shadow).

#### Why
CSS `box-shadow` is clipped by `CALayer`'s `masksToBounds = YES`, which was set in v3.0.3 for rounded corners. When `masksToBounds` is active, the shadow renders inside the window bounds and is invisible. The OS-native shadow (drawn by WindowServer, outside the compositor tree) is the only reliable way to get a drop shadow on a transparent frameless window with `masksToBounds` enabled.

---

## [3.0.3] — 2026-06-03

### True macOS Rounded Corners via objc2 + CALayer

#### Added
- `set_macos_window_corner_radius()` — unsafe Rust function using `objc2::msg_send!` to set corner radius at the OS level.
- Dependencies: `objc2 = "0.6.0"`, `objc2-app-kit = "0.3.0"` (with `NSView` + `NSWindow` features enabled), `raw-window-handle = "0.6"`.

#### How it works
The function walks the native view hierarchy:

```
WKWebView NSView
  → [nsView window]       → NSWindow
  → [window contentView]  → NSView (contentView)
  → [contentView layer]   → CALayer
```

It then sets `cornerRadius = 14.0` and `masksToBounds = YES` on the `contentView`'s `CALayer`. The `masksToBounds` flag is the key — it clips the entire compositor subtree at the OS level, giving true rounded corners on every layer in the window.

#### Why not `window-vibrancy`'s `corner_radius`?
`window-vibrancy`'s corner radius only rounds the `NSVisualEffectView` subview, not the `NSWindow` frame. The corners of the window itself remain rectangular. This was confirmed by reading the source. The objc2 + CALayer approach is the only way to round the actual window frame.

---

## [3.0.2] — 2026-06-02

### macOS Frosted Glass Vibrancy

#### Added
- `window-vibrancy 0.6` crate.
- `NSVisualEffectMaterial::HudWindow` applied on macOS — frosted glass background that adapts to light/dark mode.
- `apply_acrylic` on Windows — semi-transparent blur tint.

#### Known issue (fixed in v3.0.3)
Attempted to use `window-vibrancy`'s `corner_radius` parameter for rounded corners. This rounds the blur layer (`NSVisualEffectView`) but not the `NSWindow` frame, leaving rectangular window corners. Fixed properly in v3.0.3.

---

## [3.0.1] — 2026-06-01

### Bug Fixes and Quality of Life

#### Fixed
- Clipboard copy button now shows "Saved!" feedback: SVG checkmark swap + `save-status` CSS flash animation.
- Launch at login toggle now reads and writes state correctly.
- Preferences panel scrolling: added `overflow-y: auto` and `-webkit-overflow-scrolling: touch` on `.prefs-inner`.

#### Added
- Window position memory — saves and restores `PhysicalPosition` via IPC after every drag. Stored in `localStorage`.
- Window size memory — saves and restores `PhysicalSize` via IPC after every resize. Stored in `localStorage`.

#### Changed
- Preferences panel height capped at `min(460px, calc(100% - 44px))` — ensures all content is always reachable regardless of window height.

---

## [3.0.0] — 2026-05-30

### Feature Release: Custom Shortcut, Position Memory, Font Size, Dark Mode, Markdown, Export

#### Added

**Custom shortcut**
- User can change the default ⌘J / Ctrl+J to any key via Preferences.
- Live re-registration in Rust via `set_hotkey()` IPC command — no restart required.
- Captures `KeyboardEvent.code` (layout-independent, works regardless of keyboard language).

**Window position memory**
- `get_window_position()` and `set_window_position()` IPC commands.
- Coordinates stored as physical pixels. Off-screen guard prevents the window from being restored to an unreachable position. Stored in `localStorage`.

**Font size control**
- 14–28px slider in Preferences.
- Drives `--font-size` CSS custom property — editor and Markdown preview update simultaneously, no re-render needed.

**Dark mode**
- Three modes: Light, Auto, Dark.
- Auto follows `prefers-color-scheme` with a live `matchMedia` listener (updates without reload when the OS theme changes).
- Driven by `data-theme="dark"` on `:root`.

**Markdown preview**
- ⌘M toggle (⌘M again to return to editor).
- Inline vanilla-JS parser, ~80 lines — no external library.
- Supports: `h1`–`h3`, bold, italic, inline code, fenced code blocks, blockquotes, horizontal rules, unordered and ordered lists, links.

**Plain-text export**
- Footer copy button: copies all text to clipboard.
- Preferences "Copy all text" button: same action, accessible from the prefs panel.
- Uses `navigator.clipboard.writeText()` (modern async API — works correctly in Tauri's custom-protocol context).

#### Changed
- Preferences panel redesigned with sections, scrollable content area, and full keyboard navigation.

---

## [2.8.0] — 2026-05-15

### IPC Reliability + Window Drag Fix

#### Fixed

**`withGlobalTauri`**
- Added `"withGlobalTauri": true` to `tauri.conf.json`. This is required for `window.__TAURI__.event.listen` to function. Without it, `tauri://focus` events (and all other Tauri events) never fire — and the failure is silent.

**Window drag on macOS**
- Replaced CSS `-webkit-app-region: drag` (unsupported in Tauri v2, silently ignored) with `start_dragging()` IPC command.
- Fixed event handler ordering: `e.preventDefault()` must be called before `ipc('start_dragging')`, not after. The OS requires drag state to be open when the Rust side processes the event.

**IPC bridge**
- `getInvoke()` helper now tries `__TAURI_INTERNALS__.invoke` (Tauri v2 primary path) and falls back to `__TAURI__.core.invoke`. Removes the fragile direct `window.__TAURI__.invoke` calls.

**Update check**
- `check_for_update()` was calling `resp.json::<T>().await` — this always fails because `tauri_plugin_http`'s bundled reqwest does not enable the `json` feature. Changed to `.text().await` + `serde_json::from_str()`.

**Version display**
- Version string in Preferences now loads and displays correctly.

---

## [2.0.0] — 2026-04-20

### Tauri v2 Migration

#### Changed
- Migrated from Tauri v1 to Tauri v2.
- Plugin system updated: `tauri-plugin-global-shortcut`, `tauri-plugin-autostart`, `tauri-plugin-http`.
- Permissions model replaced by Tauri v2 capabilities system (`capabilities/` directory).
- IPC updated from `#[tauri::command]` v1 API to v2 API.
- Window event handling updated to `RunEvent::WindowEvent` / `RunEvent::ExitRequested`.

---

## [1.0.0] — 2026-04-01

### Initial Release

#### Added
- Global hotkey ⌘J (macOS) / Ctrl+J (Windows/Linux) — toggles scratchpad window.
- Escape key hides the window.
- Auto-save to `localStorage` with 300ms debounce.
- Frameless transparent window with frosted-glass CSS background.
- Fly-in animation (180ms spring easing).
- Always on top; no Dock icon (macOS LSUIElement / Accessory activation policy).
- Launch at login toggle.
- Auto-update check via GitHub Releases API.
- Space Grotesk font, 22px, 1.8 line-height.
- Paste without focus — global paste handler intercepts the shortcut even when the window isn't focused.
- macOS universal binary (arm64 + x64, built and merged by CI).
