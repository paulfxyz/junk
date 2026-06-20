# Changelog

All notable changes to Junk are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Versions follow [Semantic Versioning](https://semver.org/).

---

## [3.1.4] — 2026-06-20

### Fix: use NSWindow.alphaValue for true window-level opacity

#### Fixed
- **CSS opacity was wrong approach** — setting `opacity` on the DOM `.window`
  element makes the UI semi-transparent but the native macOS window background
  (vibrancy/frosted-glass layer) remains fully opaque. The window chrome stays
  solid; content behind the window does not show through.
- **Fix: NSWindow.alphaValue via Rust IPC** — new `set_window_opacity(opacity: f64)`
  command calls `NSWindow.setAlphaValue()` via `objc2` `msg_send!`. This sets
  opacity on the entire window compositor surface, including the vibrancy layer,
  so the window genuinely becomes see-through at the OS level.
- **CSS .window--blurred class removed** — no longer needed. Opacity is managed
  entirely at the native window level.
- On Windows/Linux: `set_window_opacity` is a no-op; CSS opacity on `.window`
  is sufficient there since there is no native vibrancy layer to work around.

---

## [3.1.3] — 2026-06-20

### Fix: fly-in animation no longer overrides dim-on-blur opacity

#### Fixed
- **Root cause of dim not working** — the fly-in animation keyframes animated
  `opacity: 0 → 1` with `fill-mode: both`, which locked `.window` opacity
  at 1 after every show event, overriding `.window--blurred` (opacity: 0.5).
  Every `tauri://focus` re-triggered the animation via `triggerFlyIn()`,
  resetting opacity to 1 regardless of blur state.
- **Fix** — removed `opacity` from `@keyframes fly-in` entirely. The animation
  now only touches `transform` (scale + translateY). Opacity is left free for
  the blur state system (`.window--blurred` at 0.5, default 1.0).

---

## [3.1.2] — 2026-06-20

### Fix: dim on blur now works reliably on macOS

#### Fixed
- **tauri://blur does not fire on always-on-top windows on macOS** — The v3.1.1
  implementation listened for `tauri://blur` (a WebKit-level JS event) which is
  not dispatched reliably when the window has a raised NSWindowLevel. The window
  never dimmed.
- **Fix: emit from Rust via `WindowEvent::Focused`** — Added a `RunEvent::
  WindowEvent { Focused }` arm in `main.rs`. When `focused == false`, Rust emits
  a custom `junk://blur` event to the WebView; when `focused == true` it emits
  `junk://focus-change`. These map directly to `NSWindowDelegate`’s
  `windowDidResignKey` / `windowDidBecomeKey` and fire unconditionally regardless
  of window level.
- **Opacity changed from 70% to 50%** — per user preference.

---

## [3.1.1] — 2026-06-20

### Dim when unfocused

#### Added
- **Dim when unfocused** — When the user switches to another app, Junk fades
  to 70% opacity. On re-focus it snaps back to 100% instantly. The transition
  is 0.18s ease, defined in CSS on `.window` so both directions animate.
  Implemented via `tauri://blur` (adds `.window--blurred`) and `tauri://focus`
  (removes it). Default: **ON**. Toggle in Preferences: “Dim when unfocused —
  Fade to 70% opacity when you switch to another app”. Persisted to
  `localStorage['junk-dim-blur']`.

---

## [3.1.0] — 2026-06-06

### Fix: always-on-top now actually stays on top on macOS

#### Fixed
- **macOS window level reset on hide/show** — When Junk is hidden (Esc) and
  re-summoned (⌘J), macOS resets the CGWindowLevel of the window. The v3.0.9
  implementation called `set_always_on_top()` once at startup, which was
  correct for the initial show but lost on every subsequent toggle. The fix:
  re-assert always-on-top on every `tauri://focus` event (fired each time the
  window is shown), reading the user’s persisted preference from localStorage.
- **`alwaysOnTop: true` in tauri.conf.json** — Restoring the original default
  so the window is on top from the very first frame before JS runs. This is a
  belt-and-suspenders fix: JS re-asserts on focus, config sets the initial
  level. Both together are robust on macOS.

#### Root cause
`tauri.conf.json` `alwaysOnTop` sets the initial CGWindowLevel once. Tauri’s
`window.set_always_on_top()` call in Rust uses `NSWindow.setLevel()` which
survives for the lifetime of the window process — but on macOS, hiding a
window (`orderOut`) and re-showing it (`makeKeyAndOrderFront`) can cause the
window manager to re-evaluate window levels and reset non-standard levels to
normal. Solution: re-call `set_always_on_top()` on every show.

---

## [3.0.9] — 2026-06-06

### Restore always-on-top; position/font/theme memory confirmed; fix version label

#### Added
- **Always on top toggle** in Preferences — Junk’s core UX (floating above all windows)
  was inadvertently disabled during the v3.0.2 visual rework (`alwaysOnTop: false` in
  tauri.conf). Now re-implemented as a runtime toggle: default ON, persisted in
  localStorage, applied via `set_always_on_top()` IPC on startup and on toggle change.
  New Rust command: `set_always_on_top(always_on_top: bool)`. New capability permission:
  `core:window:allow-set-always-on-top`.

#### Fixed
- **Version label in Preferences** was hardcoded to `v3.0.4` — now driven by the
  `check_for_update` IPC result and updated to `v3.0.9` as the static default.
- **Window position memory** — confirmed fully wired: saved after every drag and resize,
  restored on startup and on every `tauri://focus` event.
- **Font size memory** — confirmed fully wired: slider saves to localStorage, restored
  via CSS custom property `--font-size` on startup.
- **Dark mode persistence** — confirmed fully wired: light/auto/dark preference saved to
  localStorage, restored on startup, auto mode listens to `prefers-color-scheme`.

---

## [3.0.8] — 2026-06-06

### Fix: restore clean index.html, resolves windowEl ReferenceError

#### Fixed
- **Critical JS crash on macOS** — `ReferenceError: Can't find variable: windowEl` crashed
  all JavaScript in the WebKit/WKWebView renderer. Root cause: the deep-comment pass in
  v3.0.4 rewrote `index.html` from 1567 → 2067 lines. The resulting file passes Node.js
  syntax checks but triggers a JavaScriptCore-specific parsing failure at runtime, making
  all IPC (clipboard, markdown toggle, settings, drag) non-functional.
- Fix: reverted `src/index.html` to commit `4a18c56` (pre-deep-comment, 1567 lines,
  proven working). All buttons and drag restore to full working order.

---

## [3.0.7] — 2026-06-06

### Revert broken footer changes; restore working state

#### Fixed
- Reverted all v3.0.5 and v3.0.6 footer changes (drag handle div, `.footer-drag-handle`
  CSS, `position: relative` on `.footer`) — they broke Markdown toggle, clipboard copy,
  and settings buttons by adding an absolutely-positioned overlay that interfered with
  click events in multiple ways.
- Footer drag (grabbing from the empty centre of the bottom bar) works as it did in
  v3.0.4: the empty gap between `.footer-left` and `.footer-right` is the `<footer>`
  element itself, which is not in the INTERACTIVE selector list, so mousedown bubbles
  to `#window` and triggers `start_dragging()` normally.
- All footer buttons (Markdown toggle, clipboard copy, settings gear) work correctly.

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
- **Root cause note** — this fix made buttons appear correct in the diff but the window was still broken due to the unrelated deep-comment `windowEl` crash in `index.html` (see v3.0.8). v3.0.6 was never fully functional.

---

## [3.0.5] — 2026-06-06

### Footer drag handle restored

#### Fixed
- Dragging the window from the bottom bar works again. The footer's empty centre zone now has a dedicated invisible drag handle (`div.footer-drag-handle`) with `position: absolute` spanning the full footer height. Clicking it falls through to the `#window` `mousedown` listener which calls `start_dragging()` IPC.
- Added subtle grip-dot indicator in the footer centre so the draggable area is visually discoverable.
- `.footer` gets `position: relative` so the absolute handle is contained correctly.
- **Regression introduced** — the `footer-drag-handle` div was `position:absolute` covering the full footer, blocking all button clicks. This was not caught before release. See v3.0.6 for the pointer-events fix, and v3.0.8 for the full resolution.

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
