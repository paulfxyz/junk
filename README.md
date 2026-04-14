```
     ██╗██╗   ██╗███╗   ██╗██╗  ██╗
     ██║██║   ██║████╗  ██║██║ ██╔╝
     ██║██║   ██║██╔██╗ ██║█████╔╝
██   ██║██║   ██║██║╚██╗██║██╔═██╗
╚█████╔╝╚██████╔╝██║ ╚████║██║  ██╗
 ╚════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝

  the flying scratchpad — built with Rust + Tauri v2
```

[![Version](https://img.shields.io/badge/version-2.8.0-5b5bf6?style=flat-square)](https://github.com/paulfxyz/junk/releases)
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
| **Preferences panel** | In-window frosted-glass sheet — launch at login, auto-update, credits |
| **Auto-save** | Content persists to `localStorage` with a 300 ms debounce — zero data loss |
| **Frosted glass UI** | `backdrop-filter: blur(40px) saturate(180%)` — beautiful on any wallpaper |
| **Always on top** | Floats above all other windows so it's always reachable |
| **Frameless + draggable** | No title bar, no traffic lights — drag anywhere (except textarea) to reposition |
| **No Dock icon** | macOS `Accessory` activation policy — stays invisible between uses |
| **No Taskbar icon** | `skipTaskbar: true` on Windows/Linux — same philosophy |
| **No blur-hide** | Window stays visible when you click another app (critical for clipboard workflows) |
| **Fly-in animation** | 180 ms spring easing — appears with intention, not a pop |
| **Universal binary** | macOS: native Apple Silicon + Intel in a single `.dmg` |
| **Tiny footprint** | ~18 MB RAM, ~4 MB installer — versus 130 MB / 160 MB for the old Electron version |
| **Zero runtime deps** | No Node.js, no Electron, no update daemon, nothing in your background |
| **Space Grotesk** | 22 px, 1.8 line-height — big, readable, distraction-free |
| **Paste anywhere** | ⌘V / Ctrl+V works even without clicking the textarea first |

---

## Install

### macOS (Universal — Apple Silicon + Intel)

1. Download **`Junk_2.8.0_universal.dmg`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Open the DMG → drag **Junk** into **Applications**
3. Remove the Gatekeeper quarantine flag:

   ```sh
   xattr -rd com.apple.quarantine /Applications/Junk.app
   open /Applications/Junk.app
   ```

   > **Why this step?** macOS Gatekeeper quarantines every app downloaded from the internet unless it's notarised with a paid Apple Developer ID ($99/yr). The `xattr -rd` command removes the quarantine flag — it's the same as clicking "Open Anyway" in System Settings, but more reliable.

4. Junk will not appear in your Dock — that's by design. It runs silently in the background.
5. Press **⌘J** from any application.

---

### Windows

1. Download **`Junk_2.8.0_x64-setup.exe`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Run the installer. Windows SmartScreen will show a blue warning — click **More info** → **Run anyway**

   > **Why SmartScreen?** The binary is not code-signed with a Windows EV certificate ($200–500/yr). The source is fully public — build it yourself if you prefer (instructions below).

3. Junk launches on login and runs silently in the background.
4. Press **Ctrl+J** from any application.

**MSI (enterprise / silent deployment):**

```
msiexec /i Junk_2.8.0_x64_en-US.msi /quiet
```

---

### Linux — AppImage

```sh
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.8.0_amd64.AppImage
chmod +x Junk_2.8.0_amd64.AppImage
./Junk_2.8.0_amd64.AppImage
```

Portable — runs on any modern x86_64 Linux without installation. No sudo required.

> **Wayland note:** Global shortcuts use X11/libxdo. On a pure Wayland session, `Ctrl+J` may not register — run with `XDG_SESSION_TYPE=x11` or enable XWayland in your compositor.

---

### Linux — .deb (Debian / Ubuntu)

```sh
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.8.0_amd64.deb
sudo dpkg -i Junk_2.8.0_amd64.deb
junk
```

---

## Usage

| Action | Result |
|---|---|
| **⌘J** (macOS) / **Ctrl+J** (Win/Linux) | Toggle the window |
| **Esc** | Hide the window (or close Preferences if open) |
| **⌘,** / **Ctrl+,** | Open Preferences panel |
| Click + drag (anywhere except textarea) | Move the window |
| **⌘A** / **Ctrl+A** | Select all text |
| **⌘V** / **Ctrl+V** | Paste — works without clicking the textarea first |
| **⌘Z** / **Ctrl+Z** | Undo — history survives hide/show cycles |
| Just type | Saves automatically, 300 ms debounce |

There is no close button. The only ways to dismiss are **Esc** and **⌘J / Ctrl+J** — this prevents accidentally losing the window entirely.

---

## Preferences

Open with **⌘,** / **Ctrl+,** or the **⚙** icon in the footer. The panel slides up from the bottom as a frosted-glass sheet.

### Launch at login

A system-level toggle. When enabled, Junk registers itself as an OS login item:

| Platform | Mechanism |
|---|---|
| macOS | Per-user LaunchAgent plist in `~/Library/LaunchAgents/` — no root required |
| Windows | `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` registry key |
| Linux | `~/.config/autostart/junk.desktop` |

The toggle reads the actual OS state on every panel open — stays accurate if you toggled it from System Settings separately.

### Auto-check for updates

When enabled (default: on), Junk silently checks the GitHub Releases API ~2 seconds after first launch. If a newer version is found, the ⚙ icon turns purple. No nagging, no banners — just a quiet indicator.

The "Check now" button triggers an immediate check and shows the result inline:

- **"You're up to date (2.8.0)"** — green
- **"Update available: v2.x.x"** — purple, clickable link to releases page
- **"Could not check — are you online?"** — red if the request fails

### Credits

The Credits section explains how to truly quit Junk and links to [gojunk.app](https://gojunk.app) and the GitHub repo.

> **To truly quit Junk:** open Activity Monitor (macOS) or Task Manager (Windows), find "Junk", and force-quit. By design, ⌘Q and × hide the window — the process stays alive so the shortcut always works. This is the same architecture used by Alfred, Paste, and Magnet.

---

## The Clipboard Workflow

The no-blur-hide policy exists for one reason: **clipboard-based workflows break if the scratchpad vanishes when you click elsewhere.**

```
1. You're in Slack, a thought arrives
2. Press ⌘J → Junk appears
3. You paste a URL or start drafting something
4. Click into Safari to check a reference (Junk stays visible)
5. Copy a paragraph from Safari
6. Click back into Junk and paste
7. Press ⌘J → Junk disappears, you're back in Slack
```

If Junk hid when you clicked Safari in step 4, the whole workflow breaks. Apps like Alfred hide on blur — great for launchers, terrible for a scratchpad. Junk never hides unless you explicitly ask it to.

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

Storage paths (keyed to `com.paulfleury.junk`):

| Platform | Path |
|---|---|
| macOS | `~/Library/WebKit/com.paulfleury.junk/` |
| Windows | `%APPDATA%\com.paulfleury.junk\` |
| Linux | `~/.local/share/com.paulfleury.junk/` |

Content survives restarts, updates, and hide/show cycles. Never synced to any server.

To **clear**: open Junk, ⌘A / Ctrl+A, delete.

<details>
<summary><strong>Why localStorage and not a plain file?</strong></summary>

Three concrete reasons:

1. **No IPC for reads.** Content loads synchronously from the WebView's own storage — no async round-trip, no flash of empty content on show. File-based storage would require an async Rust IPC call before the textarea could populate, introducing a visible blank moment.

2. **The WebView is always live.** Because Junk never destroys its WebView (it only hides it), `localStorage` is always hot in memory. Writes are ~1 µs in-process operations against the live V8/JavaScriptCore storage, not filesystem syscalls.

3. **Undo history is free.** The browser's native undo stack (`⌘Z / Ctrl+Z`) operates on the textarea DOM node. That node persists across all hide/show cycles because the WebView is never recreated — you get unlimited, lossless undo for the entire session. A file-based approach would require reimplementing undo from scratch.

The trade-off: content isn't accessible at a normal file path and can't be opened in another editor. That's acceptable — Junk is a transient capture buffer, not a document editor.

</details>

---

## Comparison

| App | Shortcut | Blur-hide | Dock/Taskbar | Persistent | Offline |
|---|---|---|---|---|---|
| **Junk** | ⌘J (global) | Never | No | Yes | Yes |
| Alfred clipboard | ⌘Space | Yes | No | Yes | Yes |
| Raycast notes | ⌘Space | Yes | No | Yes | Yes |
| Notion quick note | None | — | Yes | Yes | Partial |
| Apple Notes | None | — | Yes | Yes | Yes |
| Scratchpad (various) | App-defined | Yes | Usually | No | Yes |

Key differentiators: no blur-hide, no Dock/Taskbar entry, always on top, and one keypress from anywhere — with zero interaction required once it appears.

---

## Security

**Network:** The only outbound request Junk ever makes is an optional `fetch` to `https://api.github.com/repos/paulfxyz/junk/releases/latest` for update checks — disable in Preferences. No analytics, no telemetry, no crash reporting.

**Local data:** Your text lives on your machine only. Never sent anywhere.

**Code audit:** `src-tauri/src/main.rs` is ~490 lines. The entire frontend is one HTML file (~1,200 lines). MIT-licensed, fully public — [read the source](https://github.com/paulfxyz/junk).

<details>
<summary><strong>Tauri capability permissions (minimal surface area)</strong></summary>

```json
{
  "permissions": [
    "core:default",
    "core:window:allow-show",
    "core:window:allow-hide",
    "core:window:allow-set-focus",
    "core:window:allow-is-visible",
    "core:window:allow-start-dragging",
    "core:webview:allow-set-webview-focus",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "global-shortcut:allow-is-registered",
    "autostart:allow-enable",
    "autostart:allow-disable",
    "autostart:allow-is-enabled",
    "http:default"
  ]
}
```

Notably absent: filesystem access, clipboard API (browser `paste` events need no permission), notifications, camera, microphone, arbitrary network access beyond the GitHub API.

</details>

---

## Design Philosophy

Junk is not a notes app. It solves a narrower, more specific problem: **the 30-second window when a thought arrives and you need somewhere to put it right now.**

Every design decision traces back to this constraint:

**One keypress, immediate.** Every additional action (launching the app, navigating to a note, choosing a location) increases the chance the thought evaporates. The global shortcut is not a convenience — it is the product.

**Never ask for organisation.** The moment you ask a user where to put something, you've made them think about their system instead of their idea. Junk has one scratchpad. No inbox, no archive, no folder, no tag. The act of deciding what to keep and where to put it is a separate task for a separate tool.

**Stay out of the way when not needed.** No Dock icon. No menu bar entry. No notification badge. No background network traffic (by default). Zero pixels and zero attention when you're not using it.

**Stay visible when needed.** When Junk is on screen, it floats above everything. It doesn't hide when you click away. You can look at Safari, look at Junk, copy from Safari, paste into Junk — without the window ever disappearing.

**Plain text, always.** No markdown rendering, no rich text, no formatting toolbar. Formatting is a cognitive tax on the writing process. Junk is for the thought, not its presentation.

---

## Architecture

<details>
<summary><strong>Why Rust + Tauri?</strong> (Electron → Rust migration)</summary>

Junk started as an Electron app (v1.0.0–v1.5.0). The Rust/Tauri rewrite delivers a fundamentally different performance profile:

| Metric | Electron v1.5.0 | Tauri v2 (Rust) |
|---|---|---|
| Installer size | ~160 MB | ~4 MB |
| RAM at idle | ~130 MB | ~18 MB |
| Cold start time | ~600 ms | ~80 ms |
| Runtime requirement | Node.js + Chromium | None |
| Binary count | 80+ files in bundle | 1 executable |
| Memory safety | GC + JS heap | Rust ownership model |

The binary ships as a single ~4 MB file with no external runtime. Startup time is imperceptible — the window appears before the user's finger lifts off the key.

Tauri v2 uses the OS's native WebView (WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux) rather than bundling Chromium. This is the primary reason for the size and RAM difference: we're sharing the WebView engine with the browser already running on the user's machine.

</details>

<details>
<summary><strong>The IPC bridge: withGlobalTauri and the two invoke paths (v2.8.0)</strong></summary>

This is the most important lesson from the v2.x development cycle. Understanding it prevents hours of debugging silent failures.

### The two invoke paths

Tauri v2 exposes its JavaScript IPC bridge in two ways:

**Path 1: `window.__TAURI_INTERNALS__.invoke`**

Always available. Injected directly into the WebView's JavaScript context before any user scripts run, unconditionally, at the C++/Swift/Kotlin bridge level. This is what `@tauri-apps/api` uses internally:

```js
// From @tauri-apps/api/core.js — this is the actual implementation
async function invoke(cmd, args = {}, options) {
  return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
}
```

**Path 2: `window.__TAURI__.core.invoke`**

Only available when `withGlobalTauri: true` is set in `tauri.conf.json`. This populates `window.__TAURI__` as a convenience namespace — it is literally just a re-export of the same `@tauri-apps/api` package. Without this setting, `window.__TAURI__` is `undefined`.

### What we got wrong (v2.6.0 → v2.7.1)

Every IPC call in the frontend looked like this:

```js
// This was our code — seemed reasonable, was silently broken
async function hideWindow() {
  const invokeFn = window.__TAURI__?.core?.invoke;
  if (typeof invokeFn !== 'function') return;  // ← always returned here
  await invokeFn('hide_window');
}
```

Because `withGlobalTauri` was never set, `window.__TAURI__` was `undefined`. The optional chaining silently returned `undefined`. The guard `if (typeof invokeFn !== 'function') return` silently exited every time.

The result: **every single IPC call in the app was a silent no-op** from v2.6.0 through v2.7.1. Window drag, preferences panel, launch at login toggle, update check, version display — all of them appeared to work (no errors) but did nothing. The drag showed a grab cursor (pure JS) but the window never moved. The prefs panel opened (pure CSS transition) but the toggles never read their state.

### The fix (v2.8.0)

Two changes together:

**1. `tauri.conf.json` — add `withGlobalTauri: true`**

```json
{
  "app": {
    "withGlobalTauri": true,
    ...
  }
}
```

This one line populates `window.__TAURI__` with all the standard API namespaces.

**2. JS — use `__TAURI_INTERNALS__` as primary, never rely on `__TAURI__` alone**

```js
// v2.8.0 — belt-and-suspenders, works with or without withGlobalTauri
function getInvoke() {
  return (
    window.__TAURI_INTERNALS__?.invoke ??   // primary: always available
    window.__TAURI__?.core?.invoke ??        // fallback: needs withGlobalTauri
    null
  );
}
```

Similarly for event listening:
```js
// Always use __TAURI__.event.listen (populated by withGlobalTauri)
// NOT window.__TAURI_INTERNALS__ for events — that namespace doesn't expose listen()
const listenFn = window.__TAURI__?.event?.listen ?? null;
```

### The lesson

When building a Tauri v2 app without a bundler (no `import { invoke } from '@tauri-apps/api/core'`), always set `withGlobalTauri: true`. The alternative is using `window.__TAURI_INTERNALS__.invoke` directly, but that's an internal API that Tauri could change. With `withGlobalTauri: true`, you get the stable public API at `window.__TAURI__.core.invoke`.

The reason this is non-obvious: optional chaining (`?.`) makes the failure completely silent. There are no console errors, no exceptions — just functions that appear to work but do nothing. The symptoms look like a timing issue or a capabilities problem. The actual cause is a one-line config omission.

</details>

<details>
<summary><strong>Window drag: the full story across v2.4.0 → v2.8.0</strong></summary>

Getting window drag to work on a macOS transparent frameless Tauri window took four versions and uncovered three separate layers of platform behaviour. This is the complete investigation.

### Attempt 1: CSS `-webkit-app-region: drag` (v2.4.0 → v2.6.0)

The obvious approach. WKWebView supports the `-webkit-app-region` CSS property — set it to `drag` and the OS compositor treats that region as a window drag handle.

```css
.window {
  -webkit-app-region: drag;
}
#editor, button, a, label {
  -webkit-app-region: no-drag;
}
```

**What happened:** The drag handle cursor (⠿ pill) appeared. The grab cursor appeared on hover. But dragging moved nothing.

**Why:** `-webkit-app-region: drag` works on macOS WKWebView only when the window has `decorations: true` (has a native title bar). With `decorations: false` + `transparent: true`, the WKWebView compositor receives the CSS hint but the signal is never forwarded to `NSWindow`. This is [documented in Tauri issues](https://github.com/tauri-apps/tauri/issues) but not prominently. The behaviour is silent — no error, no warning.

### Attempt 2: `invoke('plugin:window|start_dragging')` (v2.7.0)

The Tauri docs and `@tauri-apps/api` source show a `startDragging()` method on `WebviewWindow` objects. Its JS implementation is:

```js
// From @tauri-apps/api/window.js
async startDragging() {
  return invoke('plugin:window|start_dragging', { label: this.label });
}
```

We called this directly from the `mousedown` listener:

```js
invokeFn('plugin:window|start_dragging', { label: 'main' }).catch(() => {});
```

**What happened:** The window still didn't move. No error.

**Why — two problems:**

First, `invokeFn` was `window.__TAURI__?.core?.invoke` which (as described above) was `undefined` because `withGlobalTauri` wasn't set. So the call was silently dropped before it even reached the IPC bridge.

Second, even fixing the invoke path, there's a timing problem: `invoke()` is always async — it queues a message to the Rust event loop and resolves in a microtask. By the time the Rust side calls `NSWindow.performWindowDragWithEvent:`, the OS drag candidate window has closed. The OS decides "this was a click, not a drag" before the Rust command executes.

### Attempt 3: `invoke('start_dragging')` (v2.7.1 — wrong name)

We tried calling our own (non-existent) `start_dragging` command. Since it wasn't registered in `invoke_handler![]`, the IPC bridge returned an error — silently discarded by `.catch(() => {})`.

### Solution: custom Rust command + `e.preventDefault()` (v2.8.0)

The working approach requires solving both problems together:

**Fix the timing problem:**

```js
windowEl.addEventListener('mousedown', (e) => {
  if (e.button !== 0) return;
  // Walk ancestors — skip if interactive element
  let node = e.target;
  while (node && node !== windowEl) {
    if (node.matches && node.matches(INTERACTIVE_SELECTORS)) return;
    node = node.parentElement;
  }

  // KEY: preventDefault() BEFORE the async invoke.
  // This tells the OS: "I'm handling this mousedown — keep drag state open."
  // Without this, the OS commits to "click" before the Rust side executes.
  e.preventDefault();

  ipc('start_dragging').catch(() => {});
});
```

**Add the Rust command:**

```rust
/// Initiate a native OS window drag.
///
/// Called from JS via invoke('start_dragging') on mousedown.
/// Calls window.start_dragging() which invokes NSWindow.performWindowDragWithEvent:
/// on macOS — a direct OS window manager call that bypasses the WebView entirely.
///
/// The JS side must call e.preventDefault() before this invoke so the
/// OS keeps the drag candidate state open until this Rust command executes.
#[tauri::command]
fn start_dragging(window: WebviewWindow) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}
```

Register it in `invoke_handler!`:
```rust
.invoke_handler(tauri::generate_handler![
    hide_window,
    start_dragging,    // ← new in v2.8.0
    open_prefs,
    get_prefs,
    set_launch_at_login,
    check_for_update,
])
```

**Why does `e.preventDefault()` solve the timing issue?**

When the OS receives a mousedown event, it enters a "drag candidate" state: it's watching whether this will become a drag (mouse moves) or a click (mouse up quickly). This state has an implicit timeout — if no drag-initiating API call arrives within a few milliseconds, the OS commits to "click" and closes the candidate window.

`e.preventDefault()` on the WebView side tells the OS: "This event is handled by the application — do not process it as a standard click sequence." The OS doesn't close the drag candidate state because it's waiting for the application to respond. The Tauri IPC bridge, despite being async at the JS level, is processed synchronously on the Rust event loop — the native drag call executes quickly enough that the OS candidate window is still open.

Without `e.preventDefault()`, the sequence is:
1. `mousedown` → OS enters drag candidate state
2. JS `mousedown` handler fires, calls async `invoke()`
3. OS times out: "no drag-initiating API — must be a click"
4. Rust receives the IPC message, calls `window.start_dragging()`
5. Too late — OS drag candidate window is already closed, nothing happens

With `e.preventDefault()`:
1. `mousedown` → OS enters drag candidate state
2. JS `mousedown` handler fires, `e.preventDefault()` called
3. OS: "event handled by app — maintaining drag candidate state"
4. Rust receives IPC, calls `window.start_dragging()` → NSWindow picks up the drag
5. Window moves

**Summary of the three layers:**

| Layer | Problem | Fix |
|---|---|---|
| CSS `-webkit-app-region` | Silently ignored on frameless transparent WKWebView | Don't use it — use IPC instead |
| Tauri IPC availability | `window.__TAURI__` undefined, all IPC silently dropped | Add `withGlobalTauri: true` to `tauri.conf.json` |
| OS drag timing | Async IPC too slow — OS closes drag candidate window | Call `e.preventDefault()` before invoke to keep OS waiting |

</details>

<details>
<summary><strong>Process persistence</strong> (why Junk never exits)</summary>

The global shortcut (`⌘J`) is registered in the Rust process. If the process exits, the shortcut unregisters — and the user has no way to restart it without a Dock icon or menu bar entry to indicate the app is gone.

Making the process persistent means the shortcut works from first launch until the user's next reboot (or explicit force-quit from Activity Monitor). No surprises.

```rust
.run(|app, event| match event {
    // Window × button → hide, don't close.
    RunEvent::WindowEvent {
        label,
        event: tauri::WindowEvent::CloseRequested { api, .. },
        ..
    } if label == "main" => {
        api.prevent_close();
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
    // ⌘Q → actually quit (v2.6.0+).
    // Previously intercepted and hidden (v2.3.0–v2.5.0 with tray), but
    // without a tray "Quit" item, users have no other way to fully exit.
    RunEvent::ExitRequested { .. } => {
        log::info!("ExitRequested (⌘Q) — shutting down.");
        app.exit(0);
    }
    _ => {}
});
```

This is the same architectural pattern used by Alfred, Paste, Magnet, and Raycast. They all hide; they never accidentally quit.

</details>

<details>
<summary><strong>Global shortcuts</strong> (OS-level, not JS)</summary>

A JS `keydown` listener only fires when the Junk window has focus. An OS-level shortcut fires regardless of which application is in the foreground — that's the entire point of a global hotkey.

Three shortcuts are registered via [`tauri-plugin-global-shortcut`](https://v2.tauri.app/plugin/global-shortcut/):

| Shortcut | Platform | Behaviour |
|---|---|---|
| **⌘J** | macOS | Toggle window (show/hide) |
| **Ctrl+J** | Windows / Linux | Toggle window (show/hide) |
| **Escape** | All | Hide window |
| **⌘,** | macOS | Open Preferences panel |
| **Ctrl+,** | Windows / Linux | Open Preferences panel |

Platform APIs used under the hood:

| Platform | API |
|---|---|
| macOS | Carbon `RegisterEventHotKey` |
| Windows | `RegisterHotKey` (Win32) |
| Linux | `XGrabKey` via `libxdo` |

The modifier is selected at compile time via `#[cfg(target_os)]` — critical because using `SUPER | CONTROL` (the bitwise OR of both) would require the user to press both ⌘ and Ctrl simultaneously:

```rust
// macOS uses ⌘ (SUPER / Command key)
#[cfg(target_os = "macos")]
let modifiers = Modifiers::SUPER;

// Windows and Linux use Ctrl
#[cfg(not(target_os = "macos"))]
let modifiers = Modifiers::CONTROL;
```

This was a real bug in early versions — `SUPER | CONTROL` compiled fine but produced a chord shortcut that no one could trigger.

**The Esc edge case:** Registering bare `Escape` (no modifier) as a global shortcut is unusual — most apps don't do this because the macOS Carbon hotkey API doesn't natively support modifier-less keys. `tauri-plugin-global-shortcut` handles this via a different code path, which is why it works.

</details>

<details>
<summary><strong>Single-file frontend</strong> (no build step, no npm runtime)</summary>

`src/index.html` is ~1,200 lines of HTML, CSS, and JavaScript — all inline, zero build step, zero npm runtime dependencies, zero external JS libraries.

**Why a single file?** Tauri embeds the frontend directory into the binary at compile time. A single file is simpler to audit, impossible to misconfigure, and eliminates the entire category of bundler/module-resolution bugs. There is no webpack.config.js to break, no node_modules directory to corrupt, no import map to misconfigure.

**Why `<script type="module">`?** ES modules give:
- Strict mode automatically — no `"use strict"` boilerplate
- Clean module scope — variables don't leak into `window`
- Implicit `defer` semantics — the script runs after DOM parsing, no `DOMContentLoaded` guards needed

**The correct invoke pattern (v2.8.0):**

```js
// The getInvoke() helper — resolves the invoke function at call time, not module load
function getInvoke() {
  return (
    window.__TAURI_INTERNALS__?.invoke ??  // primary: always injected by Tauri
    window.__TAURI__?.core?.invoke ??       // fallback: needs withGlobalTauri: true
    null
  );
}

// All IPC goes through ipc() which logs errors consistently
async function ipc(command, args = {}) {
  const invoke = getInvoke();
  if (!invoke) {
    console.error(`[ipc] invoke not available — cannot call "${command}"`);
    return undefined;
  }
  try {
    return await invoke(command, args);
  } catch (err) {
    console.error(`[ipc] "${command}" failed:`, err);
    throw err;
  }
}
```

Why lazy resolution? `<script type="module">` defers execution until after DOM parsing. On some WKWebView builds, `window.__TAURI_INTERNALS__` is injected asynchronously — capturing it once at module load can freeze a `null` reference. Resolving at call time means every invocation gets the current, post-injection value.

</details>

<details>
<summary><strong>Tauri IPC map</strong> (full JS → Rust command surface)</summary>

```
JS Frontend (via ipc() wrapper)            Rust Backend (#[tauri::command])
─────────────────────────────────────────────────────────────────────────────
ipc('hide_window')                    →  window.hide()

ipc('start_dragging')                 →  window.start_dragging()
  [called with e.preventDefault()]       [NSWindow.performWindowDragWithEvent:]

ipc('open_prefs')                     →  window.show() + window.set_focus() (if hidden)
                                         window.emit("open-prefs", ())

ipc('get_prefs')                      →  autolaunch().is_enabled()
                                         returns { launch_at_login: bool }

ipc('set_launch_at_login',            →  autolaunch().enable() or .disable()
     { enabled: bool })

ipc('check_for_update')               →  tauri-plugin-http fetch to GitHub API
                                         returns {
                                           up_to_date: bool,
                                           current: String,   ← CARGO_PKG_VERSION
                                           latest: String,    ← tag_name from API
                                           url: String        ← release link
                                         }
─────────────────────────────────────────────────────────────────────────────

Rust → JS events (via window.emit / Tauri event system)
─────────────────────────────────────────────────────────────────────────────
"tauri://focus"   →  triggerFlyIn() + setTimeout(focusEditor, 20ms)
"open-prefs"      →  openPrefs() — reads OS state fresh on every open
```

**Why does `check_for_update` run in Rust, not JS `window.fetch()`?**

Two reasons:
1. `window.fetch()` from the WebView can be blocked by the WebView's Content Security Policy. Rust bypasses CSP entirely — it runs in the OS network stack, not in the sandboxed WebView.
2. Rust has compile-time access to `CARGO_PKG_VERSION` via `env!()`. The current version is embedded in the binary at build time — no hardcoded version strings in JS, no risk of frontend and backend drifting.

**Why does `resp.json().await` not work with `tauri-plugin-http`?**

`tauri-plugin-http` re-exports a `reqwest` client but does not enable reqwest's `json` feature flag. Calling `.json::<T>().await` triggers a compile error. The workaround is to read the body as text and parse manually:

```rust
let text = resp.text().await.map_err(|e| e.to_string())?;
let body: serde_json::Value = serde_json::from_str(&text)
    .map_err(|e| e.to_string())?;
```

This is a hidden gotcha not mentioned in the Tauri documentation.

</details>

<details>
<summary><strong>Frosted glass CSS</strong></summary>

```css
background: rgba(255, 255, 255, 0.72);
backdrop-filter: blur(40px) saturate(180%);
-webkit-backdrop-filter: blur(40px) saturate(180%);
border: 1px solid rgba(255, 255, 255, 0.60);
border-radius: 14px;
box-shadow:
  0 32px 80px rgba(0, 0, 0, 0.22),          /* ambient float — main depth */
  0 8px 24px rgba(0, 0, 0, 0.12),           /* mid-range depth */
  inset 0 -1px 0 rgba(0, 0, 0, 0.06),       /* inner bottom rim — glass edge */
  inset 0 1px 0 rgba(255, 255, 255, 0.90);  /* specular top highlight */
```

`backdrop-filter` composites the blurred background from the pixels _behind_ the window — it requires `transparent: true` in `tauri.conf.json` so the OS compositor can supply those pixels to the WebView. The specular top inset shadow creates the perception of a physical glass surface catching light from above.

On older Linux setups without compositor support, `backdrop-filter` silently degrades to the `rgba(255,255,255,0.72)` background. The window remains fully functional and readable — just not glassy.

The four-layer box-shadow is the visual trick that makes Junk look like a real floating object rather than a flat rectangle. Each layer solves a different depth problem: the wide ambient shadow provides the floating sensation, the tighter shadow defines the edges, the inner bottom rim suggests physical thickness, and the top specular highlight completes the glass illusion.

</details>

<details>
<summary><strong>Fly-in animation</strong></summary>

```css
@keyframes fly-in {
  from { opacity: 0; transform: scale(0.96) translateY(-10px); }
  to   { opacity: 1; transform: scale(1)    translateY(0);      }
}
/* 180ms · cubic-bezier(0.22, 1, 0.36, 1) — spring easing */
```

**Why scale + translateY and not just opacity?**

Pure opacity fades look like the window materialises in place — no directionality, no physicality. The slight vertical offset (`-10px`) and scale (`0.96→1.0`) give the impression of the window floating up from just below the cursor. `cubic-bezier(0.22, 1, 0.36, 1)` slightly overshoots the 1.0 scale target before settling — the same spring easing used in native macOS window animations.

**Re-triggering on each show:**

CSS animations only play once per element lifecycle. The trick:

```js
function triggerFlyIn() {
  requestAnimationFrame(() => {
    windowEl.style.animationName = 'none';  // 1. browser stops tracking animation
    void windowEl.offsetWidth;              // 2. force style recalculation flush
    windowEl.style.animationName = '';      // 3. restore → fresh animation starts
  });
}
```

`void el.offsetWidth` is the key: it forces the browser to synchronously flush all pending style changes before step 3. Without it, the browser may batch steps 1 and 3 together and never observe the "no animation" state — so step 3 produces no effect.

</details>

<details>
<summary><strong>macOS activation policy</strong></summary>

macOS has three application activation policies:

| Policy | Dock Icon | App Switcher | Used by |
|---|---|---|---|
| `Regular` | Yes | Always | Standard GUI apps |
| `Accessory` | No | Only when a window is visible | Paste, Magnet, **Junk** |
| `Prohibited` | No | Never | Login agents, background daemons |

Junk uses `Accessory`, set via `app.set_activation_policy(ActivationPolicy::Accessory)` in the Tauri `setup` hook before any window is shown. This hides the Dock icon permanently while still allowing the app to receive keyboard focus and display windows.

The call must happen before any window appears — if you set the policy after `window.show()`, the Dock icon may flash briefly. The Tauri `setup` hook runs before windows are created, making it the right place.

Windows equivalent: `skipTaskbar: true` in `tauri.conf.json` — hides the app from the taskbar and Alt+Tab while the window is hidden.

</details>

<details>
<summary><strong>Launch at login internals</strong></summary>

`tauri-plugin-autostart` wraps OS-native login item mechanisms with a uniform Rust API.

**macOS — LaunchAgent plist** (`~/Library/LaunchAgents/com.paulfleury.junk.plist`):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>Label</key>           <string>com.paulfleury.junk</string>
  <key>ProgramArguments</key><array><string>/Applications/Junk.app/Contents/MacOS/junk</string></array>
  <key>RunAtLoad</key>       <true/>
  <key>KeepAlive</key>       <false/>  <!-- don't restart on exit — Junk never exits anyway -->
</dict>
</plist>
```

This is a per-user login item — no admin privileges required. `launchd` reads it on every login and starts Junk before the user's first keypress.

**Windows** — `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`

**Linux** — `~/.config/autostart/junk.desktop` (XDG autostart spec)

**Rust API (one call per operation):**

```rust
app.autolaunch().enable().map_err(|e| e.to_string())?;
app.autolaunch().disable().map_err(|e| e.to_string())?;
let is_on = app.autolaunch().is_enabled().unwrap_or(false);
```

The toggle in Preferences reads `is_enabled()` fresh on every panel open — so it stays accurate even if the user toggled it from System Settings or Task Manager while Junk was running. We never cache the state in JS.

**One gotcha:** the plugin method is `.autolaunch()` not `.autostart_manager()`. The latter doesn't exist on `AppHandle`, but the compiler error can be confusing if you're reading older Tauri v1 examples.

</details>

<details>
<summary><strong>Dependency tree</strong></summary>

```
junk (Rust binary)
├── tauri 2.x                          ← Core framework (WebView, IPC, window mgmt)
│   ├── tauri-runtime-wry              ← Cross-platform WebView via WRY
│   │   └── wry                        ← WKWebView / WebView2 / WebKitGTK bindings
│   └── tauri-utils                    ← Config parsing, asset embedding
│
├── tauri-plugin-global-shortcut 2.x   ← OS-level hotkey registration
│   ├── macOS: Carbon RegisterEventHotKey
│   ├── Windows: RegisterHotKey (Win32)
│   └── Linux: XGrabKey via libxdo
│
├── tauri-plugin-autostart 2.x         ← OS login item management
│   ├── macOS: LaunchAgent plist (~/Library/LaunchAgents/)
│   ├── Windows: HKCU\...\CurrentVersion\Run registry key
│   └── Linux: ~/.config/autostart/*.desktop
│
├── tauri-plugin-http 2.x              ← Rust HTTP (rustls-tls feature)
│   └── reqwest (re-export, no `json` feature) ← fetch GitHub API from Rust
│
├── serde + serde_json 1.x             ← IPC serialisation (Serialize/Deserialize)
├── log 0.4                            ← Structured logging facade
└── env_logger 0.11                    ← RUST_LOG-driven log subscriber (debug builds)
```

**Frontend:** zero runtime dependencies. The only external load is [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) from Google Fonts, cached permanently after first launch.

</details>

---

## Project Structure

```
junk/
├── src/
│   └── index.html           ← Entire frontend: HTML + CSS + JS, single file, no build step
├── src-tauri/
│   ├── src/
│   │   └── main.rs          ← ~490 lines, all Rust backend logic, heavily commented
│   ├── capabilities/
│   │   └── default.json     ← Tauri permission allowlist (minimal surface area)
│   ├── Cargo.toml           ← Rust dependencies + build profile (LTO, strip, panic=abort)
│   └── tauri.conf.json      ← App config: frameless, transparent, withGlobalTauri: true
├── .github/
│   └── workflows/
│       └── build.yml        ← CI: 3-platform matrix build → GitHub Release
└── README.md
```

---

## Build from Source

### Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | stable ≥ 1.70 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 20 LTS | [nodejs.org](https://nodejs.org) |
| macOS extras | Xcode CLI Tools | `xcode-select --install` |
| Linux extras | WebKitGTK + build tools | see below |

**Linux system dependencies:**

```sh
sudo apt-get update && sudo apt-get install -y \
  build-essential libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf libxdo-dev \
  libx11-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

> **Critical:** Use `libwebkit2gtk-4.1-dev`, **not** `4.0`. Tauri v2 requires WebKitGTK 4.1. Using 4.0 produces a confusing compile error deep in the Tauri dependency graph with no obvious pointer to the root cause.

---

### Dev + production builds

```sh
git clone https://github.com/paulfxyz/junk.git
cd junk && npm install

# Dev — hot-reload dev server, opens window immediately
# Right-click the window to open WebView DevTools (console, elements, storage)
npm run dev

# Production — native architecture
npm run build

# macOS universal binary (Apple Silicon + Intel in one .dmg)
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

**Output locations:**

| Platform | Path |
|---|---|
| macOS universal | `src-tauri/target/universal-apple-darwin/release/bundle/dmg/` |
| Windows NSIS | `src-tauri/target/release/bundle/nsis/` |
| Windows MSI | `src-tauri/target/release/bundle/msi/` |
| Linux AppImage | `src-tauri/target/release/bundle/appimage/` |
| Linux .deb | `src-tauri/target/release/bundle/deb/` |

**macOS ad-hoc signing (for distribution without notarisation):**

```sh
codesign -s - --force --deep \
  "src-tauri/target/universal-apple-darwin/release/bundle/macos/Junk.app"
```

The ad-hoc signature (`-s -`) is not trusted by Gatekeeper but allows users to install by removing the quarantine flag. Free, no Apple account required.

**Useful dev environment variables:**

```sh
RUST_LOG=junk=debug npm run dev      # verbose Rust logging
RUST_LOG=tauri=debug npm run dev     # Tauri internals logging
```

---

## CI / Release Pipeline

Every push to a `v*` tag triggers the GitHub Actions workflow — a 3-platform matrix:

| Runner | Artifacts |
|---|---|
| macOS | `Junk_2.8.0_universal.dmg` |
| Windows | `Junk_2.8.0_x64-setup.exe` + `Junk_2.8.0_x64_en-US.msi` |
| Ubuntu | `Junk_2.8.0_amd64.AppImage` + `Junk_2.8.0_amd64.deb` |

All artifacts are uploaded to a GitHub Release and auto-published with a git log changelog.

The release job only runs on `v*` tag pushes — not on branch pushes or PRs. It waits for all three platform builds to complete before running, ensuring no partial releases.

<details>
<summary><strong>Hard-won CI and build lessons</strong></summary>

| Problem | Root Cause | Fix |
|---|---|---|
| `libwebkit2gtk-4.0-dev` not found on Ubuntu 22.04+ | Tauri v2 requires WebKitGTK 4.1, package name changed | Use `libwebkit2gtk-4.1-dev` |
| `npm ci` fails — cache miss | `package-lock.json` not committed | Commit `package-lock.json` — `npm ci` requires it, `npm install` generates it |
| `gh release create` fails with exit 1 | Release already exists (from manual pre-creation) | Use `gh release view` to check first; use `gh release upload --clobber` if it exists |
| Stale v1.x.x artifacts appear on new releases | GitHub Actions artifact cache preserves old Cargo build outputs | Filter upload glob by version string: `find dist/ -name "*$VERSION*"` |
| `SUPER\|CONTROL` modifier on macOS needs both keys | `Modifiers::SUPER \| Modifiers::CONTROL` is a chord | Use `#[cfg(target_os = "macos")]` to select exactly one modifier per platform |
| `resp.json().await` compile error | `tauri-plugin-http`'s reqwest doesn't enable the `json` feature | Use `.text().await` + `serde_json::from_str()` instead |
| `Image::from_bytes()` doesn't exist | Wrong API name | Use `app.default_window_icon().cloned()` |
| `.autostart_manager()` doesn't exist | Changed in tauri-plugin-autostart v2 | Use `.autolaunch()` on `AppHandle` |
| `use tauri::Emitter` missing | Required for `.emit()` on `WebviewWindow` | Add `use tauri::Emitter` to imports |
| All IPC silently fails — no errors | `withGlobalTauri` not set, `window.__TAURI__` is `undefined` | Add `"withGlobalTauri": true` to `tauri.conf.json` app section |
| Window drag appears to work (grab cursor) but moves nothing | Three separate issues: CSS ignored, wrong invoke path, async timing | Use `e.preventDefault()` + custom Rust `start_dragging` command |
| `tauri-plugin-tray = "2"` not found on crates.io | Crate doesn't exist under that name | System tray is `tauri = { features = ["tray-icon"] }` — it's a built-in feature |

</details>

---

## FAQ

**Q: Why does macOS say the app is "damaged"?**
It's not damaged — it's quarantined. macOS Gatekeeper flags every app downloaded outside the App Store that isn't notarised with a paid Apple Developer account. Run:
```sh
xattr -rd com.apple.quarantine /Applications/Junk.app && open /Applications/Junk.app
```

**Q: Why isn't Junk in the Mac App Store?**
The App Store sandbox prevents apps from registering global hotkeys that work across all applications. The entire premise of Junk is incompatible with the sandbox model.

**Q: Will Junk phone home or collect analytics?**
No telemetry, no analytics, no crash reporting. The only outbound request is the optional update check to `api.github.com`. Disable it in Preferences.

**Q: How do I truly quit Junk?**
Activity Monitor (macOS) or Task Manager (Windows) → find "Junk" → force-quit. By design, ⌘Q and × hide the window so the global shortcut stays alive. This is explained in the Credits section of the Preferences panel.

**Q: ⌘J conflicts with another app. Can I change it?**
Not yet via UI — it's hardcoded in `main.rs` (`Code::KeyJ`). Change `KeyJ` to any `Code::*` variant and rebuild. A custom shortcut preference pane is on the roadmap.

**Q: Where is my data if I uninstall?**
`~/Library/WebKit/com.paulfleury.junk/` on macOS. Delete that folder to wipe all content.

**Q: Does it work natively on Apple Silicon?**
Yes. The `.dmg` is a universal binary — native ARM64 on Apple Silicon, native x86_64 on Intel. No Rosetta translation layer needed.

**Q: Why no iOS / Android?**
The global-hotkey model doesn't translate to mobile — there's no concept of a system-wide keyboard shortcut on iOS or Android.

**Q: I'm on Wayland and the shortcut doesn't work.**
Run with `XDG_SESSION_TYPE=x11`, or enable XWayland in your compositor (KDE: Display & Monitor → Compositor; GNOME: enabled by default). This is a Tauri v2 limitation — global shortcuts on Linux use X11/libxdo.

**Q: Will you add sync / cloud backup?**
No. Sync requires a backend, account system, network requests, and a whole category of failure modes. Junk is zero-infrastructure by design. Copy your text to your sync tool of choice — that's what Junk is for.

**Q: Why Space Grotesk?**
It sits at the intersection of technical precision and warmth. The squared terminals give it a code-editor sensibility without the coldness of a true monospace. At 22 px with 1.8 line-height it feels like writing on expensive paper — wide and generous, with room for each word to breathe.

**Q: Can I use multiple Junk windows?**
No. One window, one scratchpad. This is a design constraint: multiple windows reintroduce the "which one does this go in" decision that Junk exists to eliminate.

---

## Changelog

### v2.8.0 — 2026-04-14

The "everything finally works" release. Root cause of all silent failures in v2.6.0–v2.7.1 found and fixed.

- **Root cause fix:** `withGlobalTauri: true` added to `tauri.conf.json`. Without this, `window.__TAURI__` is `undefined` — every IPC call using `window.__TAURI__?.core?.invoke` silently exited at the `undefined` guard. Drag, prefs panel, version display, update check, launch at login — all silently broken since v2.6.0. One config line fixed everything.

- **Drag fix — layer 1 (IPC availability):** The invoke path `window.__TAURI__?.core?.invoke` returned `undefined` because `withGlobalTauri` was unset. Fixed by `withGlobalTauri: true` and switching primary invoke path to `window.__TAURI_INTERNALS__.invoke` (always available, no config needed).

- **Drag fix — layer 2 (custom Rust command):** Added `#[tauri::command] fn start_dragging(window: WebviewWindow)` which calls `window.start_dragging()` → `NSWindow.performWindowDragWithEvent:` directly. Registered in `invoke_handler![]`.

- **Drag fix — layer 3 (OS timing):** `e.preventDefault()` is now called before `invoke('start_dragging')` on `mousedown`. This keeps the OS drag candidate state open until the Rust side executes. Without it, the OS commits to "click" before the async IPC resolves.

- **Prefs panel:** `openPrefs()` now correctly reads OS launch-at-login state and version via `check_for_update` on every open. Version label populated. Update check result correctly renders a clickable link when an update is available.

- **JS IPC rewrite:** `getInvoke()` uses `__TAURI_INTERNALS__.invoke` (primary) with `__TAURI__.core.invoke` (fallback). `ipc()` wrapper centralises all calls with structured error logging. Toggle reverts on IPC failure.

- **Event listeners:** `setupTauriEvents()` uses `window.__TAURI__.event.listen` for `tauri://focus` and `open-prefs` — correctly connected to fly-in animation and prefs panel open.

### v2.7.1 — 2026-04-14
- **Fix:** Correct Tauri IPC command for window drag — `invoke('plugin:window|start_dragging', { label: 'main' })`. The v2.7.0 call (`invoke('start_dragging')`) was not registered and silently ignored. (Note: this fix was correct in isolation but the IPC bridge was still non-functional — fully resolved in v2.8.0.)

### v2.7.0 — 2026-04-14
- **Fix:** Replaced `-webkit-app-region: drag` CSS (silently ignored by WKWebView on transparent frameless windows) with a JS `mousedown` listener that calls Tauri's `startDragging()` IPC. Grab cursor on draggable zones.
- **Removed:** All `-webkit-app-region` CSS.

### v2.6.0 — 2026-04-14
- **Fix:** Removed system tray / menu bar icon — intrusive and unnecessary. ⌘Q now quits for real again.
- **Removed:** `tauri-plugin-tray` dependency, `setup_tray()`, all tray menu event handlers, footer drag-handle pill.

<details>
<summary><strong>Older releases (v2.5.0 → v1.0.0)</strong></summary>

### v2.5.0 — 2026-04-14
- **Feature:** System tray / menu bar icon (later removed in v2.6.0).
- **Fix:** Update checker moved from JS `window.fetch()` to Rust `tauri-plugin-http` — the old JS fetch was blocked by the WebView CSP. Rust bypass CSP entirely.
- **Architecture:** `UpdateResult` struct — `check_for_update()` returns `{ up_to_date, current, latest, url }`.

### v2.4.0 — 2026-04-14
- **Fix:** Removed `alwaysOnTop: true` — other apps' modals now appear above Junk as expected.
- **Feature:** Footer drag handle — centered six-dot pill (⠿) as window-move region.

### v2.3.0 — 2026-04-14
- **Feature:** Persistent process — `CloseRequested` and `ExitRequested` now call `window.hide()` instead of quitting.
- **Feature:** Preferences panel — in-window frosted-glass sheet, ⌘, / Ctrl+, to open.
- **Feature:** Launch at login via `tauri-plugin-autostart` (LaunchAgent on macOS, no root required).
- **Feature:** Update checker — calls GitHub Releases API from Rust.
- **Feature:** Three OS-level global shortcuts: ⌘J/Ctrl+J, Esc, ⌘,/Ctrl+,.

### v2.2.1 — 2026-04-14
- **Fix:** `invoke()` resolved lazily on every call — fixes race between `<script type="module">` deferred execution and Tauri's `window.__TAURI__` injection.
- **Fix:** `window.focus()` before `editor.focus()` on `tauri://focus` — fixes Esc on macOS WKWebView.

### v2.2.0 — 2026-04-14
- **Fix:** Esc registered as OS-level global shortcut in Rust — bypasses WebView entirely.

### v2.1.0 — 2026-04-14
- **Fix:** ⌘J modifier is now platform-conditional (`SUPER` on macOS, `CONTROL` on Win/Linux) — fixed the chord bug that required both ⌘ and Ctrl simultaneously.
- **Feature:** Landing page supports 30+ languages with flag-based modal switcher.

### v2.0.0 — 2026-04-10
- **Feature:** Scramble/typo effect on landing page hero text.
- **Feature:** i18n in 5 languages: EN, FR, ES, DE, JA.
- **Feature:** Per-OS download modals with platform-specific install instructions.

### v1.6.0 — 2026-04-07
- **First Rust/Tauri release** — complete rewrite from Electron.
- macOS universal binary, Windows NSIS + MSI, Linux AppImage + .deb.
- ~18 MB RAM, ~4 MB installer, ~80 ms cold start.
- CI pipeline: 3-platform matrix, automated GitHub Release.

### v1.0.0–v1.5.0 — Electron era
- v1.5.0: Windows + Linux builds
- v1.4.0: Esc to dismiss, Space Grotesk, frosted glass, fly-in animation, no blur-hide
- v1.3.0: Removed Dock icon (Accessory policy)
- v1.2.0: Fixed Gatekeeper signing issues
- v1.1.0: Initial release — macOS only

</details>

---

## Contributing

Pull requests are welcome. A few conventions before opening one:

1. **Comments explain the *why***, not the *what*. The code already says what it does. Comments exist to explain intent, trade-offs, and non-obvious decisions. See `main.rs` for the style.
2. **No `unwrap()` in non-test code.** Use `map_err`, `ok_or`, `if let`, or `?`. Every `.unwrap()` is a potential panic in a running user process.
3. **No new npm runtime dependencies.** The frontend is intentionally dependency-free. Inline the relevant parts if you need a utility.
4. **No new Cargo crates without a compelling reason.** Every dependency is more compile time, more attack surface, and more supply-chain risk.
5. **Test on all three platforms before opening a PR.**
6. **Preserve the comment density in `main.rs`.** Every function should have a doc comment explaining what it does and why the approach was chosen. This is a teaching codebase as much as a working one.

---

## Roadmap

- **Custom shortcut** via Preferences — change ⌘J to any key combo
- **Window position memory** — remember where you dragged it between sessions
- **Font size preference** — currently fixed at 22px / 1.8 line-height
- **Dark mode** — automatic or manual toggle; frosted glass adapts naturally
- **Markdown rendering mode** — toggle between raw text and rendered preview (off by default)
- **Plain-text export** — one-click copy of all content to clipboard

---

## Vibe coded 🤙

This app was built entirely through vibe coding using various AI tools — Rust backend, frontend, CI pipeline, landing page, and this README included. No prior Tauri or Rust experience required. It works, it ships, and the code is thoroughly commented. But it is what it is.

---

## Author

**Paul Fleury** — [paulfleury.com](https://paulfleury.com) · [GitHub @paulfxyz](https://github.com/paulfxyz)

---

## License

[MIT](LICENSE) — do whatever you want with it.
