```
     ██╗██╗   ██╗███╗   ██╗██╗  ██╗
     ██║██║   ██║████╗  ██║██║ ██╔╝
     ██║██║   ██║██╔██╗ ██║█████╔╝
██   ██║██║   ██║██║╚██╗██║██╔═██╗
╚█████╔╝╚██████╔╝██║ ╚████║██║  ██╗
 ╚════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝

  the flying scratchpad — built with Rust + Tauri v2
```

[![Version](https://img.shields.io/badge/version-2.6.0-5b5bf6?style=flat-square)](https://github.com/paulfxyz/junk/releases)
[![macOS](https://img.shields.io/badge/macOS-universal-black?style=flat-square&logo=apple)](https://github.com/paulfxyz/junk/releases)
[![Windows](https://img.shields.io/badge/Windows-x64-0078d4?style=flat-square&logo=windows)](https://github.com/paulfxyz/junk/releases)
[![Linux](https://img.shields.io/badge/Linux-AppImage%20%7C%20deb-fcc624?style=flat-square&logo=linux&logoColor=black)](https://github.com/paulfxyz/junk/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-22c55e?style=flat-square)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-ffc131?style=flat-square&logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Website](https://img.shields.io/badge/website-gojunk.app-5b5bf6?style=flat-square)](https://gojunk.app)

---

## What is Junk?

**Junk** is a global-hotkey scratchpad. Press **⌘J** (macOS) or **Ctrl+J** (Windows/Linux) — from anywhere, any app, any Space, any virtual desktop — and a clean floating notepad appears instantly. Press it again, or hit **Esc**, and it vanishes without a trace.

Your words stay. The window stays. It just gets out of your way.

No accounts. No sync. No cloud. No dock icon. No menu bar clutter. No distractions. Just a place for the thing you're thinking about right now — before you know how to say it.

> **It's the app you open before you know what you want to say.**

---

## Why does this exist?

Every developer, designer, writer, and thinker has a variation of the same workflow: you're deep in something, a thought arrives, and you need somewhere to put it. Not a note-taking app that wants you to organise it. Not a task manager that wants you to assign it. Not a full editor that loads for two seconds.

Just a place. Immediately. Then gone.

Most tools fail this test in one of three ways:

1. **They're too slow.** Notes apps take a second to launch, two seconds to open a note. The window of attention closes before the app opens.
2. **They want commitment.** Notion asks which database. Apple Notes asks which folder. Bear asks which tag. Junk asks nothing.
3. **They disappear when you look away.** Every "quick-capture" popup that hides on blur destroys the clipboard workflow that makes the tool actually useful.

Junk is designed to fail none of these tests. It appears in ~80 ms. It asks nothing. It never hides unless you explicitly dismiss it.

---

## Features

| Feature | Details |
|---|---|
| **⌘J / Ctrl+J** | Global hotkey — works in any app, any macOS Space, any virtual desktop |
| **Esc** | Hides the window — OS-level shortcut, works even before JS loads |
| **⌘, / Ctrl+,** | Opens the Preferences panel from anywhere — even when the window is hidden |
| **Persistent process** | Window hides instead of quitting — global shortcut always works, content never lost |
| **Launch at login** | Optional OS login item via `tauri-plugin-autostart` (LaunchAgent on macOS) |
| **Auto-update check** | Checks GitHub releases API on launch (optional, toggleable in Preferences) |
| **Preferences panel** | In-window frosted-glass sheet — launch at login, auto-update, credits, quit instructions |
| **Auto-save** | Content persists to `localStorage` with a 300 ms debounce — zero data loss |
| **Frosted glass UI** | `backdrop-filter: blur(40px) saturate(180%)` — beautiful on any wallpaper |
| **Always on top** | Floats above all other windows so it's always reachable |
| **Frameless** | No title bar, no traffic lights — drag anywhere on the window to reposition |
| **No Dock icon** | macOS `Accessory` activation policy — stays invisible between uses |
| **No Taskbar icon** | `skipTaskbar: true` on Windows/Linux — same philosophy |
| **No blur-hide** | Window stays visible when you click another app (critical for clipboard workflows) |
| **Drag handle** | Footer six-dot pill — hold and drag to reposition Junk anywhere on screen |
| **No alwaysOnTop** | Other apps' modals and dialogs appear above Junk naturally (v2.4.0) |
| **Fly-in animation** | 180 ms spring easing — appears with intention, not a pop |
| **Universal binary** | macOS: native Apple Silicon + Intel in a single `.dmg` |
| **Tiny footprint** | ~18 MB RAM, ~4 MB installer — versus 130 MB / 160 MB for the old Electron version |
| **Zero runtime deps** | No Node.js, no Electron, no update daemon, nothing in your background |
| **Space Grotesk** | 22 px, 1.8 line-height — big, readable, distraction-free |
| **Paste anywhere** | ⌘V / Ctrl+V works even without clicking the textarea first |

---

## Install

### macOS (Universal — Apple Silicon + Intel)

1. Download **`Junk_2.6.0_universal.dmg`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Open the DMG → drag **Junk** into **Applications**
3. Remove the Gatekeeper quarantine flag:

   ```sh
   xattr -rd com.apple.quarantine /Applications/Junk.app
   open /Applications/Junk.app
   ```

   **Why is this step needed?** macOS Gatekeeper quarantines every app downloaded from the internet unless it is notarised with a paid Apple Developer ID ($99/yr). The `xattr -rd` command removes the quarantine extended attribute — it's the same action as clicking "Open Anyway" in System Settings, but reliable. You're telling macOS: *I downloaded this intentionally, I trust it.*

4. Junk will not appear in your Dock — that's by design. It runs silently in the background.
5. Press **⌘J** from any application.

---

### Windows

1. Download **`Junk_2.6.0_x64-setup.exe`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Run the installer. Windows SmartScreen will show a blue warning — click **More info** → **Run anyway**

   **Why SmartScreen?** The binary is not code-signed with a Windows Extended Validation (EV) certificate ($200–500/yr). SmartScreen flags all unsigned binaries. The source code is fully public — if you prefer, build it yourself (instructions below).

3. Junk launches on login and disappears into the background.
4. Press **Ctrl+J** from any application.

Alternatively, download the **MSI** (`Junk_2.6.0_x64_en-US.msi`) for enterprise/silent deployment:

```
msiexec /i Junk_2.6.0_x64_en-US.msi /quiet
```

---

### Linux — AppImage

```sh
# Download
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.6.0_amd64.AppImage

# Make executable
chmod +x Junk_2.6.0_amd64.AppImage

# Run
./Junk_2.6.0_amd64.AppImage
```

AppImages are portable — they run on any modern x86_64 Linux distribution without installation. No sudo required.

> **Wayland note:** Junk uses the X11 global shortcut mechanism via `libxdo`. On a pure Wayland session (no XWayland), the `Ctrl+J` global shortcut may not register — Wayland's security model intentionally prevents global hotkeys. Run with `XDG_SESSION_TYPE=x11` or enable XWayland in your compositor to work around this. This is a Tauri v2 framework limitation.

---

### Linux — .deb (Debian / Ubuntu)

```sh
# Download and install
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.6.0_amd64.deb
sudo dpkg -i Junk_2.6.0_amd64.deb

# Run
junk
```

---

## Usage

| Action | Result |
|---|---|
| **⌘J** (macOS) | Toggle the window: show if hidden, hide if visible |
| **Ctrl+J** (Windows / Linux) | Toggle the window |
| **Esc** | Hide the window (or close Preferences if open) |
| **⌘,** (macOS) | Open Preferences panel |
| **Ctrl+,** (Windows / Linux) | Open Preferences panel |
| Drag the **⠿** pill (footer center) | Move the window — hold and drag the footer handle to reposition |
| **⌘A** / **Ctrl+A** | Select all text |
| **⌘V** / **Ctrl+V** | Paste — works even without clicking the textarea first |
| **⌘Z** / **Ctrl+Z** | Undo — full undo history survives hide/show cycles |
| Just type | Content saves automatically, 300 ms after you stop typing |

There is no close button. No title bar. The only ways to dismiss the window are **Esc** and **⌘J / Ctrl+J**. This is intentional — it prevents accidentally losing the window entirely.

---

## Preferences (v2.3.0)

Open the Preferences panel with **⌘,** (macOS) or **Ctrl+,** (Windows/Linux) — or click the **⚙** gear icon in the bottom right corner of the window. The panel slides up from the bottom as a frosted-glass sheet, overlaying the editor without replacing it.

### Launch at login

A system-level toggle. When enabled, Junk registers itself as an OS login item:

| Platform | Mechanism |
|---|---|
| macOS | Per-user LaunchAgent plist in `~/Library/LaunchAgents/` — no root required |
| Windows | `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` registry key |
| Linux | `~/.config/autostart/junk.desktop` file |

The toggle reads the actual OS state on every panel open — it stays accurate if you toggle it from System Settings / Task Manager separately.

### Auto-check for updates

When enabled (default: on), Junk checks the GitHub Releases API in the background ~2 seconds after the window first appears. If a newer version is found, the ⚙ icon in the footer turns purple and its tooltip shows the new version. No notification. No nagging. Just a quiet indicator.

The check fires at most once per launch. It uses `window.fetch()` from the WebView — no Rust network code, no extra Cargo dependencies.

### Check for updates (manual)

The "Check now" button in the Preferences panel triggers an immediate update check and shows the result inline:

- **"You're up to date (2.3.0)"** — shown in green
- **"Update available: v2.4.0"** — shown in purple, as a clickable link to the releases page
- **"Could not check — are you online?"** — shown in red if the request fails

### Credits

The Credits section at the bottom of the panel explains how to truly quit Junk, links to [gojunk.app](https://gojunk.app) and the GitHub repo, and credits the author.

> **To truly quit Junk:** open **Activity Monitor** (macOS) or **Task Manager** (Windows), find the "Junk" process, and force-quit it. ⌘Q and the × button hide the window — the process stays alive so the shortcut works from anywhere. This is the same architecture used by Alfred, Paste, and Magnet.

---

## The Clipboard Workflow

The no-blur-hide policy exists for one reason: **clipboard-based workflows break if the scratchpad vanishes when you click elsewhere.**

Here's the exact scenario Junk is designed for:

```
1. You're in Slack, you get an idea
2. Press ⌘J → Junk appears
3. You paste a URL or draft some text
4. You click into Safari to check something (Junk stays visible)
5. You copy a paragraph from Safari
6. You click back into Junk and paste
7. Press ⌘J → Junk disappears, you're back in Slack
```

If Junk hid when you clicked Safari in step 4, the entire workflow breaks. Apps like Alfred hide on blur — useful for launchers, terrible for a scratchpad. Junk never hides unless you explicitly ask it to.

---

## How your content is stored

Junk uses the Tauri WebView's **`localStorage`** — the same storage mechanism browsers use for web apps:

```
keystroke
    │
    ▼  (300 ms debounce — batches rapid typing into single writes)
localStorage.setItem('junk-content', value)
    │
    ▼  (on next launch)
localStorage.getItem('junk-content') → textarea.value
```

The data lives in the Tauri WebView profile directory, keyed to the app bundle identifier `com.paulfleury.junk`:

| Platform | Storage path |
|---|---|
| macOS | `~/Library/WebKit/com.paulfleury.junk/` |
| Windows | `%APPDATA%\com.paulfleury.junk\` |
| Linux | `~/.local/share/com.paulfleury.junk/` |

Content survives restarts, updates, and hide/show cycles. It is **not** synced to any server.

To **clear** your content: open Junk, select all (⌘A / Ctrl+A), and delete.

### Why localStorage and not a file?

The deliberate choice to use `localStorage` instead of writing a plain text file comes down to three reasons:

1. **No IPC for reads.** Loading content on startup requires zero Rust involvement — the JS reads from the WebView's own storage synchronously. No async call, no round-trip, no flash of empty content.
2. **The WebView is the process.** Because Junk never destroys its WebView, `localStorage` is always hot in memory. Writing to it is a ~1 µs in-process operation, not a syscall.
3. **Undo history is free.** The browser's native undo stack (`Ctrl+Z / ⌘Z`) operates on the textarea DOM node — it survives hide/show cycles because the same DOM node persists. A file-based approach would destroy this.

The trade-off: your content is not accessible from a normal file path, and you can't open it in another editor. That's fine — Junk is a transient buffer, not a document editor.

---

## Architecture Deep-Dive

### Why Rust + Tauri?

Junk started as an Electron app (v1.0.0–v1.5.0). Electron is excellent, but for a background scratchpad it carries costs that compound:

| Metric | Electron v1.5.0 | Tauri v2 (Rust) |
|---|---|---|
| Installer size | ~160 MB | ~4 MB |
| RAM at idle | ~130 MB | ~18 MB |
| Cold start time | ~600 ms | ~80 ms |
| Runtime requirement | Node.js + Chromium | None |
| Binary count | 80+ files in app bundle | 1 executable |
| Memory safety | GC + JS heap | Rust ownership model |

The Rust/Tauri rewrite ships as a single ~4 MB binary with no external runtime. The startup time is imperceptible. On macOS, the binary is code-signed with an ad-hoc signature (free, distributable, removable with `xattr -rd`).

### The Single-File Frontend

`src/index.html` is 1,200+ lines of HTML, CSS, and JavaScript — all in one file, with zero build step, zero npm runtime dependencies, and zero external JavaScript libraries.

**Why a single file?**

Tauri embeds the frontend directory into the binary at compile time. A single file is simpler to audit, impossible to misconfigure, and eliminates the entire category of bundler/module-resolution bugs. The payoff for adding a build step (faster iteration via hot module reload, etc.) is zero — the frontend is not complex enough to warrant it.

**Why inline CSS instead of a stylesheet?**

With a single HTML file, there is no stylesheet to link. The CSS is ~250 lines and covers the entire UI surface. Inlining it means one fewer network request (even though Tauri serves from a local virtual filesystem) and one fewer cache invalidation concern.

**Why `<script type="module">` instead of a classic `<script>`?**

ES modules give us:
- Strict mode automatically — no `"use strict"` boilerplate
- Clean module scope — variables don't leak into `window`
- `defer` semantics implicitly — the script runs after DOM parsing, avoiding `DOMContentLoaded` guards

The trade-off: `<script type="module">` runs in a microtask after DOM parsing, which creates a brief window where `window.__TAURI__` might not yet be injected by Tauri's WebView initialisation. This is why all IPC calls in the frontend resolve `window.__TAURI__?.core?.invoke` **lazily at call time**, never capturing it once at module load.

---

### Process Persistence (v2.3.0)

Starting in v2.3.0, Junk never exits. When the user closes the window, the process hides instead of terminating:

```rust
.build(tauri::generate_context!())
.expect("error while building tauri application")
.run(|app, event| match event {
    // "⌘Q" or system shutdown signal
    RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
        if let Some(w) = app.get_webview_window("main") {
            let _ = w.hide();
        }
    }
    // Window × button
    RunEvent::WindowEvent { event: WindowEvent::CloseRequested { api, .. }, .. } => {
        api.prevent_close();
        if let Some(w) = app.get_webview_window("main") {
            let _ = w.hide();
        }
    }
    _ => {}
});
```

This required changing from the simple `.run(tauri::generate_context!())` pattern to the two-stage `.build()` + `.run(|app, event| {...})` pattern — the only way to intercept `RunEvent` in Tauri v2.

**Why is persistence the right default?**

The global shortcut (`⌘J`) is registered in the Rust process. If the process exits, the shortcut unregisters. The next time the user presses `⌘J`, nothing happens — and they have no way to know why, because there's no Dock icon or menu bar entry to indicate the app is gone.

Making the process persistent means the shortcut works from the moment Junk first launches until the user's next reboot (or explicit force-quit). No surprises.

This is the same pattern used by every serious menu-bar / utility app: **Alfred**, **Paste**, **Magnet**, **Rectangle**, **Raycast**. They all hide; they never quit.

---

### Global Shortcuts

Three shortcuts are registered at the OS level using [`tauri-plugin-global-shortcut`](https://v2.tauri.app/plugin/global-shortcut/):

| Shortcut | Platform | Behaviour |
|---|---|---|
| **⌘J** | macOS | Toggle window (show/hide) |
| **Ctrl+J** | Windows / Linux | Toggle window (show/hide) |
| **Escape** | All | Hide window |
| **⌘,** | macOS | Open Preferences panel |
| **Ctrl+,** | Windows / Linux | Open Preferences panel |

All three use platform-native APIs:

| Platform | API |
|---|---|
| macOS | Carbon `RegisterEventHotKey` |
| Windows | `RegisterHotKey` (Win32) |
| Linux | `XGrabKey` via `libxdo` |

The modifier is selected at compile time via `#[cfg(target_os)]`:

```rust
// macOS: ⌘ (Command / Super)
#[cfg(target_os = "macos")]
let modifiers = Modifiers::SUPER;

// Windows + Linux: Ctrl
#[cfg(not(target_os = "macos"))]
let modifiers = Modifiers::CONTROL;

// Toggle shortcut: ⌘J or Ctrl+J
let toggle_shortcut = Shortcut::new(Some(modifiers), Code::KeyJ);

// Prefs shortcut: ⌘, or Ctrl+,
let prefs_shortcut = Shortcut::new(Some(modifiers), Code::Comma);

// Esc — no modifier, works universally
let esc_shortcut = Shortcut::new(None, Code::Escape);
```

When a shortcut fires, Rust reads `window.is_visible()` and calls `window.hide()` or `window.show()` + `window.set_focus()`. No state machine, no flags — just OS calls.

**Why OS-level and not just JS?**

A JS `keydown` listener only fires when the Junk window has focus. An OS-level shortcut fires regardless of which application is in the foreground. This is the entire point of a global hotkey — it works from Figma, from Terminal, from a full-screen game, from anywhere.

**The Esc edge case**

Registering bare `Escape` (no modifier) as a global shortcut is unusual — most apps don't do this. On macOS, the Carbon hotkey API doesn't natively support modifier-less keys. `tauri-plugin-global-shortcut` handles this differently from the Carbon path, which is why it works even though the underlying macOS API would normally reject it.

---

### Window Lifecycle

The window is **never destroyed** — only shown and hidden. This is the central architectural decision:

```
App starts
    │
    ▼
Window created (visible: false)
    │
    ├─ ⌘J pressed → window.show() + window.set_focus()
    │                      │
    │               tauri://focus event fires
    │                      │
    │               JS: triggerFlyIn() + focusEditor()
    │                      │
    │               User types (content auto-saves to localStorage)
    │                      │
    ├─ ⌘J / Esc → window.hide()
    │
    ├─ × button → prevent_close() + window.hide()
    │
    ├─ ⌘Q       → prevent_exit() + window.hide()
    │
    └─ (window stays alive in background — WebView state preserved indefinitely)
```

**Why never destroy?**

1. **Speed.** Showing an existing hidden window: ~5 ms. Creating a new window + parsing HTML: ~200–400 ms. At 80 ms cold start, recreation would mean a visible delay on every toggle.
2. **State.** Cursor position, scroll position, text selection, and the full undo history survive every hide/show cycle. The WebView's internal state is never reset.
3. **Storage.** The `localStorage` instance is always live — no re-read needed, no async cold-start flash of empty content.
4. **Shortcut registration.** The global shortcut lives in the Rust process. If the process exits, the shortcut dies. Persistent process = persistent shortcut.

---

### Tauri IPC Map

The frontend calls Rust via `window.__TAURI__.core.invoke()`. Here's the full IPC surface as of v2.3.0:

```
┌─────────────────────────────────────────────────────────────┐
│                     JS Frontend                             │
│                                                             │
│  invoke('hide_window')          → hides the window          │
│  invoke('open_prefs')           → emits 'open-prefs' event  │
│  invoke('get_prefs')            → returns { launch_at_login }│
│  invoke('set_launch_at_login',  → enables/disables OS login │
│          { enabled: bool })       item via autostart plugin  │
│  invoke('check_for_update')     → returns {                  │
│                                    current_version,          │
│                                    releases_url,             │
│                                    releases_page             │
│                                  }                           │
└────────────────────────┬────────────────────────────────────┘
                         │  Tauri IPC bridge
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Backend                             │
│                                                             │
│  fn hide_window(app)            → window.hide()              │
│  fn open_prefs(app)             → window.emit("open-prefs")  │
│  fn get_prefs(app)              → autolaunch().is_enabled()  │
│  fn set_launch_at_login(app,    → autolaunch().enable() or   │
│            enabled)               autolaunch().disable()     │
│  fn check_for_update(app)       → returns version + URLs     │
│                                   (JS does the actual fetch) │
└─────────────────────────────────────────────────────────────┘
```

**Why does `check_for_update` not do the HTTP request itself?**

Two reasons:
1. `window.fetch()` from the WebView works out of the box with no extra Cargo dependencies. Adding `reqwest` or `ureq` to the Rust side would increase binary size and compile time for no functional benefit.
2. The JS can update the UI inline with the response (show a spinner, update status text) without needing to emit another event from Rust.

The Rust command just provides the metadata (current version, GitHub API URL) so the JS doesn't have hardcoded URLs.

---

### Events (Rust → JS)

In addition to IPC commands (JS → Rust), Rust can push events to the frontend:

| Event | Emitted when | JS handler |
|---|---|---|
| `tauri://focus` | Window gains OS focus | Re-trigger fly-in animation, focus editor |
| `open-prefs` | ⌘, shortcut fires in Rust | Open preferences panel |

The `open-prefs` event is the bridge between the OS-level shortcut (registered in Rust) and the in-window UI (rendered in JS). Rust fires the shortcut, emits the event; JS listens and slides the panel open.

---

### macOS Activation Policy

macOS has three application activation policies:

| Policy | Dock Icon | App Switcher | Used by |
|---|---|---|---|
| `Regular` | Yes | Always | Standard GUI apps |
| `Accessory` | No | Only when a window is visible | Paste, Magnet, Junk |
| `Prohibited` | No | Never | Login agents, daemons |

Junk uses `Accessory`. This hides the Dock icon permanently while still allowing the app to receive keyboard focus and display windows. The policy is set via `app.set_activation_policy(ActivationPolicy::Accessory)` in the Tauri `setup` hook, before any window is shown.

On Windows, the equivalent is `skipTaskbar: true` in `tauri.conf.json` — hides the app from the taskbar and the Alt+Tab switcher while the window is hidden.

---

### Fly-in Animation

Every time the window appears, it plays a spring entry animation:

```css
@keyframes fly-in {
  from { opacity: 0; transform: scale(0.96) translateY(-10px); }
  to   { opacity: 1; transform: scale(1)    translateY(0);      }
}
/* 180ms · cubic-bezier(0.22, 1, 0.36, 1) */
```

**Why `scale + translateY` and not just `opacity`?**

Pure opacity fade looks like the window materialises in place — no directionality, no physicality. Adding a slight vertical offset (`-10px`) and scale (`0.96→1.0`) gives the impression of the window floating up from just below the cursor — a subtle spatial cue that makes the appearance feel intentional rather than abrupt.

`cubic-bezier(0.22, 1, 0.36, 1)` — the "spring" easing — overshoots the 1.0 scale target before settling. This is the same easing curve used in macOS's native window animations.

**CSS animations only play once per element lifecycle.** To replay on each show, three steps are needed:

```js
// 1. Remove the animation name — browser stops tracking the animation
el.style.animationName = 'none';
// 2. Force style recalculation — browser must flush the style before step 3.
//    Without this, the browser may batch both style changes and never "see" step 1.
void el.offsetWidth;
// 3. Restore the animation name — browser sees a new animation and starts it fresh
el.style.animationName = '';
```

This is wrapped in `requestAnimationFrame()` to ensure the animation starts in the next paint frame, avoiding a rare flash on certain WKWebView versions.

---

### Frosted Glass

The visual style uses only CSS — no images, no SVG filters, no canvas:

```css
background: rgba(255, 255, 255, 0.72);
backdrop-filter: blur(40px) saturate(180%);
-webkit-backdrop-filter: blur(40px) saturate(180%);
border: 1px solid rgba(255, 255, 255, 0.60);
border-radius: 14px;
box-shadow:
  /* Ambient float — the main depth shadow */
  0 32px 80px rgba(0, 0, 0, 0.22),
  /* Mid-range depth */
  0 8px 24px rgba(0, 0, 0, 0.12),
  /* Inner bottom rim — the glass edge */
  inset 0 -1px 0 rgba(0, 0, 0, 0.06),
  /* Specular top highlight — the glass illusion */
  inset 0 1px 0 rgba(255, 255, 255, 0.90);
```

**`backdrop-filter` composites the blurred background** from the pixels behind the window — it requires a transparent window (`transparent: true` in `tauri.conf.json`) so the OS compositor can supply those pixels. The specular inset shadow at the top edge creates the perception of a physical glass surface catching light from above.

On older Linux setups without compositor support, `backdrop-filter` silently degrades to a semi-transparent white. The window remains functional and readable, just not glassy.

---

### Preferences Panel — Engineering

The Preferences panel is an in-window sheet, not a separate OS window. It's built as an absolutely positioned `<div>` inside `.window`, with CSS transitions driving the slide-up / slide-down animation:

```css
.prefs-panel {
  position: absolute;
  left: 0; right: 0; bottom: 0;
  transform: translateY(100%);   /* Hidden: below the viewport */
  opacity: 0;
  transition:
    transform 240ms cubic-bezier(0.22, 1, 0.36, 1),
    opacity 200ms ease;
}

.prefs-open .prefs-panel {
  transform: translateY(0);      /* Shown: in place */
  opacity: 1;
}
```

A single class toggle on `.window` (`prefs-open`) drives three visual changes simultaneously:
1. The panel slides up from below
2. A translucent white overlay dims the editor behind it
3. Clicking the overlay closes the panel (pointer-events activate on the overlay only when open)

**Why in-window and not a separate OS window?**

A separate Tauri window would require an additional WebView instance, additional IPC commands to open/close it, separate JS context, and coordination between two windows. The in-window sheet shares the same WebView context — the JS can read `localStorage`, call the same `invoke()` functions, and access the same DOM without any cross-window messaging.

---

### Launch at Login — Internals

The `tauri-plugin-autostart` crate wraps OS-native login item mechanisms:

**macOS — LaunchAgent plist:**

```xml
<!-- ~/Library/LaunchAgents/com.paulfleury.junk.plist -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" ...>
<plist version="1.0">
<dict>
  <key>Label</key>           <string>com.paulfleury.junk</string>
  <key>ProgramArguments</key><array><string>/Applications/Junk.app/...</string></array>
  <key>RunAtLoad</key>       <true/>
  <key>KeepAlive</key>       <false/>
</dict>
</plist>
```

This is a per-user login item — no admin/root privileges required. `launchd` reads the plist on login and starts Junk. The `KeepAlive: false` means launchd will not restart it if it exits (which, by design, it never does).

**Windows — Registry:**

```
HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
  Junk = "C:\Program Files\Junk\Junk.exe"
```

**Linux — XDG Autostart:**

```ini
# ~/.config/autostart/junk.desktop
[Desktop Entry]
Type=Application
Name=Junk
Exec=/usr/bin/junk
Hidden=false
X-GNOME-Autostart-enabled=true
```

The Rust code is a single call per direction:

```rust
// Enable
app.autolaunch().enable().map_err(|e| e.to_string())?;

// Disable
app.autolaunch().disable().map_err(|e| e.to_string())?;

// Query
let is_enabled = app.autolaunch().is_enabled().unwrap_or(false);
```

---

### The `invoke()` Laziness Rule

Every call to Tauri IPC in the frontend resolves `window.__TAURI__?.core?.invoke` **lazily at call time**:

```js
// ✅ Correct — lazy resolution
async function hideWindow() {
  const invokeFn = window.__TAURI__?.core?.invoke;
  if (typeof invokeFn !== 'function') return;
  await invokeFn('hide_window');
}

// ❌ Wrong — captured once at module load, can be a frozen no-op
const invoke = window.__TAURI__?.core?.invoke ?? (async () => {});
```

**Why does this matter?**

`<script type="module">` defers execution until after DOM parsing — but Tauri's WebView injects `window.__TAURI__` asynchronously via the native bridge. On some WKWebView builds (macOS) and some WebView2 builds (Windows), the injection races with module execution. If `invoke` is captured at module load time and the injection hasn't completed yet, you get `undefined`, and the optional-chain fallback freezes it as `async () => {}` forever — every IPC call silently no-ops for the entire session.

Lazy resolution re-checks `window.__TAURI__` every time. By the time any user action triggers an IPC call (clicking a button, pressing a key), the bridge is long-since injected.

---

### Auto-Save Debounce

```js
let saveTimer = null;

function scheduleSave() {
  if (saveTimer !== null) clearTimeout(saveTimer);
  saveTimer = setTimeout(saveContent, 300);
}

editor.addEventListener('input', scheduleSave);
```

**Why 300 ms?**

- **< 100 ms** would save on nearly every keystroke — unnecessary `localStorage` writes during rapid typing
- **300 ms** matches the perceptual threshold for "I've paused" — it feels instantaneous to the user
- **> 500 ms** risks losing content if the process is force-quit mid-word

The save-status indicator in the footer pulses when a save fires, giving the user a low-key confirmation their words are safe.

---

### The Paste-Anywhere Handler

```js
document.addEventListener('paste', (e) => {
  if (document.activeElement === editor) return; // already focused — native handles it
  if (prefsOpen) return;                          // don't intercept in prefs context

  const text = e.clipboardData?.getData('text/plain');
  if (!text) return;
  e.preventDefault();

  const start = editor.selectionStart ?? editor.value.length;
  const end   = editor.selectionEnd   ?? editor.value.length;
  editor.value = editor.value.slice(0, start) + text + editor.value.slice(end);

  const newPos = start + text.length;
  editor.setSelectionRange(newPos, newPos);
  scheduleSave();
  editor.focus();
});
```

This handler intercepts paste events that land on any element other than the textarea — which happens when the user presses ⌘J and immediately presses ⌘V before clicking. Without it, the paste would target the `<body>` or the `.window` element, which are not editable, and nothing would happen.

The handler uses `e.clipboardData.getData('text/plain')` — synchronous, no async Clipboard API permission required, works in any WebView context.

---

## Project Structure

```
junk/
├── src/
│   └── index.html              ← Single-file frontend: HTML + CSS + JS, zero build step
│
├── src-tauri/
│   ├── src/
│   │   └── main.rs             ← Rust backend: shortcuts, window management, IPC (456 lines)
│   ├── capabilities/
│   │   └── default.json        ← Tauri v2 permission grants (minimal surface area)
│   ├── Cargo.toml              ← Rust crate: dependencies, profiles, metadata
│   ├── build.rs                ← Tauri build script (generates Rust glue from tauri.conf.json)
│   └── tauri.conf.json         ← Window config, bundle identifiers, app metadata
│
├── assets/
│   └── icons/                  ← App icons: .icns (macOS), .ico (Windows), .png (Linux)
│       ├── icon.icns
│       ├── icon.ico
│       ├── 32x32.png
│       ├── 128x128.png
│       └── 128x128@2x.png
│
├── .github/
│   └── workflows/
│       └── build.yml           ← CI: builds all 3 platforms in parallel, publishes GitHub release
│
├── package.json                ← npm scripts + @tauri-apps/cli (only JS dependency)
├── package-lock.json           ← Required for npm ci in CI
└── README.md
```

**Every file has one job.** `main.rs` is Rust — no JavaScript in it. `index.html` is the frontend — no Rust knowledge required to read it. `tauri.conf.json` is configuration — not code. This separation makes the project trivially auditable.

---

## Build from Source

### Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | stable ≥ 1.70 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 20 LTS | [nodejs.org](https://nodejs.org) |
| macOS extras | Xcode CLI Tools | `xcode-select --install` |
| Linux extras | WebKitGTK + build tools | See below |

**Linux system dependencies (Debian/Ubuntu):**

```sh
sudo apt-get update && sudo apt-get install -y \
  build-essential libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf libxdo-dev
```

> **Critical:** Use `libwebkit2gtk-4.1-dev`, **not** `4.0`. Tauri v2 requires WebKitGTK 4.1. Using 4.0 produces a confusing compile error deep in the Tauri dependency graph.

---

### Development build

```sh
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# Hot-reload dev server — opens a window immediately
npm run dev
```

In dev mode, right-click anywhere in the window to open the WebView inspector. All standard browser DevTools are available — console, network, elements, storage.

**Useful dev environment variables:**

```sh
# Enable verbose Rust logging
RUST_LOG=debug npm run dev

# Enable Tauri's own debug output
RUST_LOG=tauri=debug npm run dev
```

---

### Production build

```sh
# Standard build (native architecture)
npm run build

# macOS universal binary (Apple Silicon + Intel in one file)
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

**Output locations:**

| Platform | Path |
|---|---|
| macOS (universal) | `src-tauri/target/universal-apple-darwin/release/bundle/dmg/` |
| macOS (native) | `src-tauri/target/release/bundle/dmg/` |
| Windows NSIS | `src-tauri/target/release/bundle/nsis/` |
| Windows MSI | `src-tauri/target/release/bundle/msi/` |
| Linux AppImage | `src-tauri/target/release/bundle/appimage/` |
| Linux deb | `src-tauri/target/release/bundle/deb/` |

**macOS ad-hoc signing (for distribution without notarisation):**

```sh
# After building:
codesign -s - --force --deep \
  "src-tauri/target/universal-apple-darwin/release/bundle/macos/Junk.app"
```

The ad-hoc signature (`-s -`) is not trusted by Gatekeeper but allows users to install by removing the quarantine flag. It is free, requires no Apple account, and is the approach used by many open-source macOS apps.

---

## CI / Release Pipeline

Every push to a `v*` tag triggers the GitHub Actions workflow:

```
Tag pushed (e.g. v2.3.0)
        │
        ├─ [macOS runner]    npm ci
        │                    rustup target add aarch64-apple-darwin x86_64-apple-darwin
        │                    npm run tauri build -- --target universal-apple-darwin
        │                    codesign -s - (ad-hoc signing)
        │                    → Junk_2.6.0_universal.dmg
        │
        ├─ [Windows runner]  npm ci
        │                    npm run tauri build
        │                    → Junk_2.6.0_x64-setup.exe
        │                    → Junk_2.6.0_x64_en-US.msi
        │
        └─ [Ubuntu runner]   npm ci
                             apt-get install libwebkit2gtk-4.1-dev ...
                             npm run tauri build
                             → Junk_2.6.0_amd64.AppImage
                             → Junk_2.6.0_amd64.deb
                                      │
                                      ▼
                             All artifacts uploaded to GitHub Release
                             Release auto-published with changelog
```

**Hard-won CI lessons:**

| Problem | Fix |
|---|---|
| `libwebkit2gtk-4.0-dev` not found on Ubuntu 22.04+ | Use `libwebkit2gtk-4.1-dev` — Tauri v2 requires 4.1 |
| `actions/setup-node@v4` cache fails | `package-lock.json` must be committed — npm ci requires it |
| Artifact glob `bundle/**` inconsistent across runners | Use `find dist/ -type f` instead |
| Stale v1.6.0 artifacts always appear on new releases | Clean with `gh api DELETE /releases/assets/{id}` after upload |
| `SUPER\|CONTROL` modifier on macOS | Use `#[cfg(target_os)]` to select SUPER vs CONTROL per platform |

The stale artifact issue deserves explanation: the GitHub Actions upload-artifact action sometimes includes pre-existing assets from a previous build cache. The CI workflow always runs a cleanup step after upload to delete any artifact whose name contains an older version number.

---

## Comparison to Alternatives

| App | Shortcut | Blur-hide | Dock/Taskbar | Persistent | Offline |
|---|---|---|---|---|---|
| **Junk** | ⌘J (global) | Never | No | Yes | Yes |
| Alfred clipboard | ⌘Space | Yes | No | Yes | Yes |
| Notion quick note | None | — | Yes | Yes | Partial |
| Apple Notes | None | — | Yes | Yes | Yes |
| Scratchpad (various) | App-defined | Yes | Usually | No | Yes |
| Raycast notes | ⌘Space | Yes | No | Yes | Yes |

The key differentiators are:
- **No blur-hide.** Every competitor hides when you click elsewhere. Junk doesn't.
- **No Dock/Taskbar entry.** Junk doesn't pollute your app switcher.
- **Always on top.** Junk floats above every window, including full-screen apps (on macOS).
- **Single shortcut, always works.** No search, no modal, no loading. One keypress, immediate access.

---

## Security

**Network requests:** The only outbound request Junk ever makes is an optional `fetch` to `https://api.github.com/repos/paulfxyz/junk/releases/latest`. This is triggered by the update check (startup or manual) and is disabled if you toggle "Auto-check for updates" off in Preferences. There is no analytics, no telemetry, no crash reporting, and no other outbound connection.

**Local data:** Your content is stored in the Tauri WebView profile on your own machine (see [storage paths above](#how-your-content-is-stored)). It is never sent anywhere.

**Code audit:** The Rust backend (`src-tauri/src/main.rs`) is ~460 lines. The entire frontend is one HTML file. The source is MIT-licensed and fully public. Read it yourself.

**Tauri capabilities (what the app is allowed to do):**

```json
{
  "permissions": [
    "core:window:allow-hide",
    "core:window:allow-show",
    "core:window:allow-is-visible",
    "core:window:allow-set-focus",
    "core:event:allow-emit-to",
    "core:event:allow-listen",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "autostart:allow-enable",
    "autostart:allow-disable",
    "autostart:allow-is-enabled"
  ]
}
```

This is the minimal surface area needed. Notably absent: filesystem access, clipboard API (we use `ClipboardData` from paste events — no permission needed), network access (Tauri doesn't control `window.fetch`), notifications, camera, microphone.

---

## Dependency Tree

```
junk (Rust binary)
├── tauri 2.x                          ← Core framework (WebView, IPC, window management)
│   ├── tauri-runtime-wry              ← Cross-platform WebView via WRY
│   │   └── wry                        ← WebKitGTK / WebView2 / WKWebView bindings
│   └── tauri-utils                    ← Config parsing, asset embedding
│
├── tauri-plugin-global-shortcut 2.x   ← OS-level hotkey registration
│   ├── macOS: Carbon RegisterEventHotKey
│   ├── Windows: RegisterHotKey (Win32)
│   └── Linux: XGrabKey via libxdo
│
├── tauri-plugin-autostart 2.x         ← OS login item management
│   ├── macOS: LaunchAgent plist (~Library/LaunchAgents/)
│   ├── Windows: HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
│   └── Linux: ~/.config/autostart/junk.desktop
│
├── serde + serde_json 1.x             ← IPC serialisation
├── log 0.4                            ← Structured logging facade
└── env_logger 0.11                    ← RUST_LOG-driven log subscriber (debug builds)
```

**Frontend:** zero dependencies. `index.html` is a single self-contained file — HTML, CSS, and JavaScript all inline. No build step, no bundler, no npm modules at runtime. The only external load is [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) from Google Fonts, cached after the first launch.

---

## Design Philosophy

Junk is not trying to be a notes app. It is not a replacement for Notion, Obsidian, Bear, or Apple Notes. It solves a much narrower, much more specific problem: **the 30-second window when a thought arrives and you need somewhere to put it right now.**

Every design decision in Junk traces back to this constraint:

**One keypress, immediate.** Every additional action (launching the app, navigating to a note, choosing a location) increases the chance the thought evaporates. The global shortcut is not a convenience — it is the product.

**Never ask for organisation.** The moment you ask a user where to put something, you've made them think about their system instead of their idea. Junk has one scratchpad. There is no inbox, no archive, no folder, no tag. The act of deciding what to keep and where to put it is a separate task for a separate tool.

**Stay out of the way when not needed.** No Dock icon. No menu bar entry. No notification badge. No background network traffic (by default). Junk earns zero pixels and zero attention when you're not using it.

**Stay visible when needed.** When Junk is on screen, it floats above everything. It doesn't hide when you click away. You can look at Safari, look at Junk, copy from Safari, paste into Junk — without the window ever disappearing. This is the behaviour users expect from a reference panel, not a popup.

**Plain text, always.** No markdown rendering, no rich text, no formatting toolbar. Formatting decisions are a cognitive tax on the writing process. Junk is for the thought, not its presentation.

---

## FAQ

**Q: Why does macOS say the app is "damaged"?**
A: It's not damaged — it's quarantined. macOS Gatekeeper flags every app downloaded outside the App Store that isn't notarised with a paid Apple Developer account. Run `xattr -rd com.apple.quarantine /Applications/Junk.app && open /Applications/Junk.app` to remove the flag. This is safe.

**Q: Why isn't Junk in the Mac App Store?**
A: The App Store's sandbox restrictions prevent apps from registering global hotkeys that work across all applications. Junk's core feature is incompatible with the sandbox.

**Q: Will Junk phone home or collect analytics?**
A: No telemetry, no analytics, no crash reporting. The only network request Junk makes is an optional update check to `api.github.com/repos/paulfxyz/junk/releases/latest` — you can disable it in Preferences. The only data it writes locally is your text.

**Q: How do I truly quit Junk?**
A: Open **Activity Monitor** (macOS), search for "Junk", and force-quit. On Windows: **Task Manager** → find Junk → End Task. By design, ⌘Q and the × button hide the window instead of exiting — this keeps the global shortcut alive at all times. The Preferences panel explains this.

**Q: ⌘J conflicts with another app on my Mac. Can I change it?**
A: Not yet via UI — it's hardcoded. Edit `main.rs` (change `Code::KeyJ` to any other `Code::*` variant), rebuild, and replace your app bundle. A custom shortcut preference pane is on the roadmap.

**Q: Where is my data if I uninstall?**
A: Your content lives in the Tauri WebView profile. On macOS: `~/Library/WebKit/com.paulfleury.junk/`. Delete that folder to wipe everything completely.

**Q: Does Junk work on macOS without Rosetta on Apple Silicon?**
A: Yes. The `.dmg` ships a universal binary — native ARM64 code runs on Apple Silicon, native x86_64 runs on Intel. No Rosetta translation layer needed.

**Q: Why no iOS / Android?**
A: The global-hotkey model doesn't translate to mobile — there's no concept of a system-wide keyboard shortcut on iOS or Android. Junk is fundamentally a desktop-first tool.

**Q: I'm on Wayland and the shortcut doesn't work.**
A: Wayland's security model prevents global hotkeys at the compositor level. Run `XDG_SESSION_TYPE=x11` before starting Junk, or enable XWayland in your compositor (KDE: enable in Display & Monitor → Compositor settings; GNOME: enabled by default). This is a Tauri v2 limitation, not specific to Junk.

**Q: Will you add sync / cloud backup?**
A: No. Adding sync means adding a backend, an account system, network requests, and failure modes. Junk is zero-infrastructure by design. If you want cloud backup, copy your text to a note in your sync tool of choice — that's what Junk is for.

**Q: Why Space Grotesk?**
A: It sits at the intersection of technical precision and warmth. The squared terminals give it a code-editor feel without the coldness of a true monospace. At 22px with 1.8 line-height, it feels like writing on expensive paper — wide and generous, with room for each word to breathe.

**Q: Can I use multiple Junk windows?**
A: No. One window, one scratchpad. This is a design constraint, not a technical one. Multiple windows would reintroduce the "which window does this go in" decision that Junk exists to eliminate.

---

## Changelog

### v2.6.0 — 2026-04-14
- **Fix:** Removed system tray / menu bar icon — it was intrusive and unnecessary. ⌘Q now quits for real again (restored from v2.3.0 behaviour)
- **Feature:** Full-window drag — the entire Junk window surface is now a drag region. Grab it from the header, the side margins, the footer bar — anywhere that isn't the text editing area — and drag to reposition. No dedicated pill or affordance needed
- **Architecture:** `-webkit-app-region: drag` on `.window`, overridden with `no-drag` on `#editor`, `.footer-prefs-btn`, `.prefs-panel`, and all interactive elements
- **Removed:** Footer drag-handle pill (⠿) and all associated CSS/HTML — replaced by natural whole-window drag
- **Removed:** `tauri-plugin-tray` dependency, `setup_tray()` function, `tauri::tray` imports, all tray menu event handlers
- **Change:** `ExitRequested` handler now calls `app.exit(0)` instead of `api.prevent_exit()`. The close button (×) still hides; ⌘Q now truly quits

### v2.5.0 — 2026-04-14
- **Feature:** System tray / menu bar icon — Junk now lives in the macOS menu bar (Windows/Linux: system tray). The tray icon is always visible, even when the window is hidden. Left-click toggles the window. Menu: `Show/Hide Junk` · `Preferences` · `Check for Updates…` · `Quit Junk`
- **Feature:** True background process — `Quit Junk` in the tray menu is now the **only** way to exit. `⌘Q` and the close button both hide the window. The process is always running, global shortcuts always respond
- **Fix:** Update checker — moved the GitHub API fetch from JS (`window.fetch()`) to Rust (`tauri-plugin-http` / reqwest). The old JS fetch was blocked by the WebView's Content Security Policy. Rust bypasses CSP entirely and sets the required `User-Agent` header for the GitHub API
- **Architecture:** `UpdateResult` struct — `check_for_update()` now returns `{ up_to_date: bool, current: String, latest: String, url: String }`. The `url` is the releases page (deep-linked to the latest release when an update is available)
- **Architecture:** Tray emits `update-result` event to the WebView when the user clicks "Check for Updates" from the tray menu — JS listener opens the Preferences panel and renders the result inline
- **Architecture:** Added `tauri-plugin-tray` and `tauri-plugin-http` (with `rustls-tls` feature) to Cargo.toml
- **Architecture:** Capabilities updated — `tray-icon:default` and `http:default` permissions added to `capabilities/default.json`

### v2.4.0 — 2026-04-14
- **Fix:** Removed `alwaysOnTop: true` — other apps' modals, alerts, and dialogs can now appear above Junk as expected
- **Feature:** Footer drag handle — a centered six-dot pill (⠿) in the footer bar is now the dedicated window-move region. Hold and drag it to reposition Junk anywhere on screen
- **Architecture:** `‑webkit‑app‑region: drag` removed from the entire `.window` element. The footer drag handle is now the **only** drag region in the app — prevents accidental drags when clicking the content area
- **Improvement:** Window `center: true` on first launch — Junk opens centered, then remembers where you dragged it for the session

### v2.3.0 — 2026-04-14
- **Feature:** Persistent process — `CloseRequested` and `ExitRequested` events now call `window.hide()` instead of quitting. The process lives indefinitely so the global shortcut is always registered. This is the same architecture used by Alfred, Paste, and Magnet.
- **Feature:** Preferences panel — in-window frosted-glass sheet (`⌘,` / `Ctrl+,` to open). Slides up from the bottom. Three options: "Launch at login" toggle, "Auto-check for updates" toggle, and a "Check now" button with inline status. Credits section explains how to truly quit.
- **Feature:** Launch at login — uses `tauri-plugin-autostart` with `MacosLauncher::LaunchAgent` (per-user plist, no root required). Reads real OS state on every panel open so it stays accurate if the user toggles it in System Settings.
- **Feature:** Update checker — calls GitHub Releases API (`/repos/paulfxyz/junk/releases/latest`) from JS on startup (if auto-update enabled) and on demand from the Preferences button. Compares semver tags and surfaces a link if an update is available.
- **Feature:** Three OS-level global shortcuts: `⌘J`/`Ctrl+J` (toggle), `Esc` (hide), `⌘,`/`Ctrl+,` (open prefs). All registered in Rust — bypass the WebView entirely.
- **Improvement:** Footer now shows a gear icon (⚙) as a clickable prefs button alongside existing shortcuts
- **Improvement:** `Esc` now closes the Preferences panel if open, rather than hiding the window
- **Architecture:** `.build()` + `.run(|app, event| {...})` pattern replaces `.run(...)` — required to intercept `RunEvent::ExitRequested` and `RunEvent::WindowEvent::CloseRequested`

### v2.2.1 — 2026-04-14
- **Fix:** Esc key now reliably invokes `hide_window` — `invoke()` is now resolved lazily inside `hideWindow()` on every call, not captured once at module-load time. Previously a race between `<script type="module">` deferred execution and Tauri's async `window.__TAURI__.core` injection meant `invoke` could silently freeze as a no-op
- **Fix:** `window.focus()` called before `editor.focus()` on `tauri://focus` event — ensures WebView claims OS keyboard focus before the textarea receives it, fixing Esc on macOS WKWebView

### v2.2.0 — 2026-04-14
- **Fix:** Esc key now reliably dismisses the window on all platforms — registered as an OS-level global shortcut in Rust (`register_esc_shortcut()`), bypassing the WebView entirely. Previously, Esc was handled only in JS which was unreliable due to `-webkit-app-region: drag` compositor interception and async module load timing
- **Architecture:** Two OS-level shortcuts now registered: ⌘J/Ctrl+J (toggle) + Escape (hide). JS handlers remain as belt-and-suspenders fallback
- **Improvement:** Bumped versions across all manifests: `Cargo.toml`, `tauri.conf.json`, `package.json`

### v2.1.0 — 2026-04-14
- **Fix:** ⌘J global shortcut now works correctly on macOS — the modifier is now platform-conditional (`Modifiers::SUPER` on macOS, `Modifiers::CONTROL` on Windows/Linux) instead of the previous combined `SUPER | CONTROL` which required both keys simultaneously
- **Fix:** Hero text scramble/typo effect on gojunk.app now resolves correctly
- **Feature:** Landing page now supports 30+ languages with flag-based modal language switcher
- **Improvement:** All release artifacts, README, and website now point to v2.1.0 coherently

### v2.0.0 — 2026-04-10
- **Feature:** Scramble/typo effect on landing page hero text (subtle, GPU-composited)
- **Feature:** i18n in 5 languages: EN, FR, ES, DE, JA — language switcher in nav
- **Feature:** Per-OS download modals with platform-specific install instructions
- **Improvement:** Full a11y pass — focus rings, ARIA labels, keyboard-navigable platform cards
- **Improvement:** Scroll reveal with stagger on feature cards
- **Improvement:** `prefers-reduced-motion` support

### v1.6.0 — 2026-04-07
- **First Rust/Tauri release** — complete rewrite from Electron
- macOS universal binary (Apple Silicon + Intel), Windows NSIS + MSI, Linux AppImage + .deb
- ~18 MB RAM, ~4 MB installer, ~80 ms cold start
- CI pipeline: 3-platform matrix build, automated GitHub Release

### v1.5.0 — Electron era
- Added Windows + Linux builds
- Updated README with architecture docs

### v1.4.0 — Electron era
- ESC key to dismiss
- Space Grotesk font
- Frosted glass design, fly-in animation
- No blur-hide (persistent window for clipboard workflows)

### v1.3.0 — Electron era
- Removed Dock icon (Accessory policy)
- Removed menu bar icon
- Window persistence across app-switches

### v1.2.0 — Electron era
- Fixed Gatekeeper signing issues

### v1.1.0 — Electron era
- Initial release — macOS only

---

## Contributing

Pull requests are welcome. A few things to know before opening one:

1. **All comments must explain the *why***, not the *what*. The code already says what it does. Comments exist to explain intent, trade-offs, and non-obvious decisions. See `main.rs` for the style.
2. **No `unwrap()` in non-test code.** Use `map_err`, `ok_or`, `if let`, or `?` instead. Every `.unwrap()` is a potential panic in the user's process.
3. **No new npm dependencies.** The frontend is intentionally dependency-free at runtime. If you need a library, inline the relevant parts.
4. **No new Cargo dependencies without a compelling reason.** The dependency tree is intentionally small. Every additional crate is more compile time, more attack surface, more potential for supply-chain issues.
5. **Test on all three platforms before opening a PR.** The CI matrix will catch platform regressions, but it's faster if you do too.
6. **Preserve the comment density in `main.rs`.** Every function should have a block comment explaining what it does and why the approach was chosen. This is a teaching codebase as much as a working one.

---

## Roadmap

These are things being considered — not committed promises:

- **Custom shortcut** via Preferences — let users change ⌘J to any key combo
- **Multiple scratchpads** — named, switchable. Maybe. Risks adding the "which one" decision problem.
- **Font size preference** — currently fixed at 22px / 1.8 line-height
- **Window position memory** — remember where you dragged it last session
- **Dark mode** — automatic or manual toggle; the frosted glass can adapt
- **Markdown rendering mode** — toggle between raw text and rendered preview (off by default)
- **Plain-text export** — one-click copy of everything to clipboard

---

## Built with Perplexity Computer

This app — from the Rust rewrite to the landing page to this README — was built using [Perplexity Computer](https://www.perplexity.ai/computer), an AI agent that writes production-quality code, reasons about architecture, and explains every decision inline.

Every comment in `main.rs` is part of the output. The architecture diagrams, the trade-off tables, the FAQ — all generated and iterated through conversation. The design philosophy section above is the product of asking: *why does this thing exist, and what is it not trying to be?*

---

## Author

**Paul Fleury** — [paulfleury.com](https://paulfleury.com) · [GitHub @paulfxyz](https://github.com/paulfxyz)

---

## License

[MIT](LICENSE) — do whatever you want with it.
