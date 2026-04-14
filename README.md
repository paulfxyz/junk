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
| **Frameless** | No title bar, no traffic lights — drag anywhere (except textarea) to reposition |
| **No Dock icon** | macOS `Accessory` activation policy — stays invisible between uses |
| **No Taskbar icon** | `skipTaskbar: true` on Windows/Linux — same philosophy |
| **No blur-hide** | Window stays visible when you click another app (critical for clipboard workflows) |
| **Draggable** | Click and hold anywhere outside the textarea to move the window |
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

There is no close button. The only ways to dismiss are **Esc** and **⌘J / Ctrl+J** — prevents accidentally losing the window.

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

The toggle reads the actual OS state on every panel open — stays accurate if you toggle it from System Settings separately.

### Auto-check for updates

When enabled (default: on), Junk silently checks the GitHub Releases API ~2 seconds after first launch. If a newer version is found, the ⚙ icon turns purple. No nagging, no banners — just a quiet indicator.

The "Check now" button triggers an immediate check and shows the result inline:

- **"You're up to date (2.7.1)"** — green
- **"Update available: v2.x.x"** — purple, clickable link to releases page
- **"Could not check — are you online?"** — red if the request fails

### Credits

The Credits section explains how to truly quit Junk, links to [gojunk.app](https://gojunk.app) and the GitHub repo.

> **To truly quit Junk:** open Activity Monitor (macOS) or Task Manager (Windows), find "Junk", and force-quit. By design, ⌘Q and × hide the window — the process stays alive so the shortcut always works. This is the same architecture used by Alfred, Paste, and Magnet.

---

## The Clipboard Workflow

The no-blur-hide policy exists for one reason: **clipboard-based workflows break if the scratchpad vanishes when you click elsewhere.**

```
1. You're in Slack, a thought arrives
2. Press ⌘J → Junk appears
3. You paste a URL or start drafting something
4. Click into Safari to check something (Junk stays visible)
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
    ▼  (300 ms debounce)
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

1. **No IPC for reads.** Content loads synchronously from the WebView's own storage — no async round-trip, no flash of empty content.
2. **The WebView is always live.** Because Junk never destroys its WebView, `localStorage` is always hot in memory. Writes are ~1 µs in-process operations, not syscalls.
3. **Undo history is free.** The browser's native undo stack operates on the textarea DOM node, which persists across hide/show cycles. A file-based approach would destroy this.

The trade-off: content isn't accessible at a normal file path and can't be opened in another editor. That's fine — Junk is a transient buffer, not a document editor.

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

Key differentiators: no blur-hide, no Dock/Taskbar entry, always on top, and one keypress from anywhere.

---

## Security

**Network:** The only outbound request Junk ever makes is an optional `fetch` to `https://api.github.com/repos/paulfxyz/junk/releases/latest` for update checks — disable in Preferences. No analytics, no telemetry, no crash reporting.

**Local data:** Your text lives on your machine only. Never sent anywhere.

**Code audit:** `src-tauri/src/main.rs` is ~460 lines. The entire frontend is one HTML file. MIT-licensed, fully public — [read the source](https://github.com/paulfxyz/junk).

<details>
<summary><strong>Tauri capability permissions (minimal surface area)</strong></summary>

```json
{
  "permissions": [
    "core:window:allow-hide",
    "core:window:allow-show",
    "core:window:allow-is-visible",
    "core:window:allow-set-focus",
    "core:window:allow-start-dragging",
    "core:event:allow-emit-to",
    "core:event:allow-listen",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister",
    "autostart:allow-enable",
    "autostart:allow-disable",
    "autostart:allow-is-enabled",
    "http:default"
  ]
}
```

Notably absent: filesystem access, clipboard API (Paste events need no permission), notifications, camera, microphone.

</details>

---

## Design Philosophy

Junk is not a notes app. It solves a narrower, more specific problem: **the 30-second window when a thought arrives and you need somewhere to put it right now.**

Every design decision traces back to this constraint:

**One keypress, immediate.** Every additional action (launching the app, navigating to a note, choosing a location) increases the chance the thought evaporates. The global shortcut is not a convenience — it is the product.

**Never ask for organisation.** The moment you ask a user where to put something, you've made them think about their system instead of their idea. Junk has one scratchpad. No inbox, no archive, no folder, no tag.

**Stay out of the way when not needed.** No Dock icon. No menu bar entry. No notification badge. Zero pixels and zero attention when you're not using it.

**Stay visible when needed.** When Junk is on screen, it floats above everything. Doesn't hide when you click away. You can look at Safari, look at Junk, copy from Safari, paste into Junk — without the window ever disappearing.

**Plain text, always.** No markdown rendering, no rich text, no formatting toolbar. Formatting is a cognitive tax. Junk is for the thought, not its presentation.

---

## Architecture

<details>
<summary><strong>Why Rust + Tauri?</strong> (Electron → Rust migration)</summary>

Junk started as an Electron app (v1.0.0–v1.5.0). The Rust/Tauri rewrite delivers a fundamentally different profile:

| Metric | Electron v1.5.0 | Tauri v2 (Rust) |
|---|---|---|
| Installer size | ~160 MB | ~4 MB |
| RAM at idle | ~130 MB | ~18 MB |
| Cold start time | ~600 ms | ~80 ms |
| Runtime requirement | Node.js + Chromium | None |
| Binary count | 80+ files in bundle | 1 executable |
| Memory safety | GC + JS heap | Rust ownership model |

The binary ships as a single ~4 MB file with no external runtime. Startup is imperceptible.

</details>

<details>
<summary><strong>Window drag implementation</strong> (why <code>startDragging()</code> and not CSS)</summary>

`-webkit-app-region: drag` is silently ignored by WKWebView on macOS transparent frameless windows (`decorations: false` + `transparent: true`). The CSS property reaches the WebView compositor but the signal is not forwarded to the OS window manager under this configuration.

The correct approach: a `mousedown` listener on `.window` walks the DOM from the click target upward. If no interactive ancestor is found (`textarea`, `input`, `button`, `select`, `a`, `label`, `.prefs-panel`, `.editor-dim`), it calls:

```js
invoke('plugin:window|start_dragging', { label: 'main' })
```

This invokes the Tauri window plugin's native `startDragging()` — the OS window manager takes over the drag directly, bypassing the WebView layer. A `grab` cursor appears via `mousemove` to signal draggable zones.

</details>

<details>
<summary><strong>Process persistence</strong> (why Junk never exits)</summary>

The global shortcut (`⌘J`) is registered in the Rust process. If the process exits, the shortcut unregisters — and the user has no way to restart it without a Dock icon or menu bar entry.

Making the process persistent means the shortcut works from first launch until the user's next reboot (or explicit force-quit). No surprises.

```rust
.run(|app, event| match event {
    RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
        if let Some(w) = app.get_webview_window("main") {
            let _ = w.hide();
        }
    }
    RunEvent::WindowEvent { event: WindowEvent::CloseRequested { api, .. }, .. } => {
        api.prevent_close();
        if let Some(w) = app.get_webview_window("main") {
            let _ = w.hide();
        }
    }
    _ => {}
});
```

This is the same pattern used by Alfred, Paste, Magnet, and Raycast.

</details>

<details>
<summary><strong>Global shortcuts</strong> (OS-level, not JS)</summary>

A JS `keydown` listener only fires when the Junk window has focus. An OS-level shortcut fires regardless of which application is in the foreground — that's the entire point.

Three shortcuts are registered via [`tauri-plugin-global-shortcut`](https://v2.tauri.app/plugin/global-shortcut/):

| Shortcut | Platform | Behaviour |
|---|---|---|
| **⌘J** | macOS | Toggle window |
| **Ctrl+J** | Windows / Linux | Toggle window |
| **Escape** | All | Hide window |
| **⌘,** | macOS | Open Preferences |
| **Ctrl+,** | Windows / Linux | Open Preferences |

Platform API used:

| Platform | API |
|---|---|
| macOS | Carbon `RegisterEventHotKey` |
| Windows | `RegisterHotKey` (Win32) |
| Linux | `XGrabKey` via `libxdo` |

The modifier is selected at compile time via `#[cfg(target_os)]`:

```rust
#[cfg(target_os = "macos")]
let modifiers = Modifiers::SUPER;

#[cfg(not(target_os = "macos"))]
let modifiers = Modifiers::CONTROL;
```

</details>

<details>
<summary><strong>Single-file frontend</strong> (no build step, no npm runtime)</summary>

`src/index.html` is ~1,200 lines of HTML, CSS, and JavaScript — all inline, zero build step, zero npm runtime dependencies, zero external JS libraries.

**Why a single file?** Tauri embeds the frontend directory into the binary at compile time. A single file is simpler to audit, impossible to misconfigure, and eliminates the entire category of bundler/module-resolution bugs.

**Why `<script type="module">`?** ES modules give strict mode automatically, clean module scope (variables don't leak into `window`), and implicit `defer` semantics without `DOMContentLoaded` guards.

**The `invoke()` laziness rule:** `<script type="module">` defers execution until after DOM parsing, but Tauri's WebView injects `window.__TAURI__` asynchronously. All IPC calls resolve `window.__TAURI__?.core?.invoke` lazily at call time, never captured at module load:

```js
// ✅ Correct — lazy resolution
async function hideWindow() {
  const invokeFn = window.__TAURI__?.core?.invoke;
  if (typeof invokeFn !== 'function') return;
  await invokeFn('hide_window');
}
```

</details>

<details>
<summary><strong>Tauri IPC map</strong> (full JS → Rust surface)</summary>

```
JS Frontend
  invoke('hide_window')            → window.hide()
  invoke('open_prefs')             → emits 'open-prefs' event to JS
  invoke('get_prefs')              → returns { launch_at_login: bool }
  invoke('set_launch_at_login',    → autolaunch().enable() / .disable()
          { enabled: bool })
  invoke('check_for_update')       → returns { current_version, releases_url }
  invoke('plugin:window|          → OS window manager takes over drag
          start_dragging',
          { label: 'main' })
```

Rust → JS events:

| Event | When | Handler |
|---|---|---|
| `tauri://focus` | Window gains OS focus | Trigger fly-in, focus editor |
| `open-prefs` | ⌘, shortcut fires in Rust | Open preferences panel |

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
  0 32px 80px rgba(0, 0, 0, 0.22),   /* ambient float */
  0 8px 24px rgba(0, 0, 0, 0.12),    /* mid-range depth */
  inset 0 -1px 0 rgba(0, 0, 0, 0.06), /* inner bottom rim */
  inset 0 1px 0 rgba(255, 255, 255, 0.90); /* specular top highlight */
```

`backdrop-filter` composites the blurred background from the pixels behind the window — requires `transparent: true` in `tauri.conf.json` so the OS compositor can supply those pixels. On Linux without compositor support it silently degrades to semi-transparent white.

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

CSS animations only play once per element lifecycle. To replay on each show:

```js
el.style.animationName = 'none';  // 1. stop tracking
void el.offsetWidth;               // 2. force style flush
el.style.animationName = '';       // 3. restart
```

`void el.offsetWidth` forces the browser to flush pending style changes before step 3 — without it, the browser may batch both changes and never "see" step 1.

</details>

<details>
<summary><strong>macOS activation policy</strong></summary>

| Policy | Dock Icon | App Switcher | Used by |
|---|---|---|---|
| `Regular` | Yes | Always | Standard GUI apps |
| `Accessory` | No | Only with visible window | Paste, Magnet, **Junk** |
| `Prohibited` | No | Never | Login agents, daemons |

Set via `app.set_activation_policy(ActivationPolicy::Accessory)` in the Tauri `setup` hook, before any window is shown. Windows equivalent: `skipTaskbar: true` in `tauri.conf.json`.

</details>

<details>
<summary><strong>Launch at login internals</strong></summary>

`tauri-plugin-autostart` wraps OS-native login item mechanisms:

**macOS — LaunchAgent plist** (`~/Library/LaunchAgents/com.paulfleury.junk.plist`):
```xml
<key>RunAtLoad</key><true/>
<key>KeepAlive</key><false/>  <!-- don't restart if it exits -->
```

**Windows:** `HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`

**Linux:** `~/.config/autostart/junk.desktop`

Rust API:
```rust
app.autolaunch().enable()?;
app.autolaunch().disable()?;
let is_on = app.autolaunch().is_enabled().unwrap_or(false);
```

</details>

<details>
<summary><strong>Dependency tree</strong></summary>

```
junk (Rust binary)
├── tauri 2.x                          ← Core (WebView, IPC, window management)
│   ├── tauri-runtime-wry              ← Cross-platform WebView
│   │   └── wry                        ← WebKitGTK / WebView2 / WKWebView
│   └── tauri-utils
│
├── tauri-plugin-global-shortcut 2.x   ← OS-level hotkey registration
├── tauri-plugin-autostart 2.x         ← OS login item management
├── tauri-plugin-http 2.x              ← Rust HTTP for update check (rustls-tls)
├── serde + serde_json 1.x             ← IPC serialisation
├── log 0.4                            ← Logging facade
└── env_logger 0.11                    ← RUST_LOG subscriber (debug builds)
```

**Frontend:** zero runtime dependencies. The only external load is [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) from Google Fonts, cached after first launch.

</details>

---

## Project Structure

```
junk/
├── src/
│   └── index.html          ← Entire frontend: HTML + CSS + JS, single file, no build step
├── src-tauri/
│   ├── src/
│   │   └── main.rs         ← ~460 lines, all Rust backend logic, heavily commented
│   ├── capabilities/
│   │   └── default.json    ← Tauri permission allowlist (minimal surface area)
│   ├── Cargo.toml          ← Rust dependencies
│   └── tauri.conf.json     ← App config: frameless, transparent, no decorations
├── .github/
│   └── workflows/
│       └── build.yml       ← CI: 3-platform matrix → GitHub Release
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
  librsvg2-dev patchelf libxdo-dev
```

> **Critical:** Use `libwebkit2gtk-4.1-dev`, **not** `4.0`. Tauri v2 requires WebKitGTK 4.1 — using 4.0 produces a confusing error deep in the dependency graph.

---

### Dev + production builds

```sh
git clone https://github.com/paulfxyz/junk.git
cd junk && npm install

# Dev (hot-reload, opens window immediately, right-click for DevTools)
npm run dev

# Production — native architecture
npm run build

# macOS universal binary (Apple Silicon + Intel)
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

**macOS ad-hoc signing:**

```sh
codesign -s - --force --deep \
  "src-tauri/target/universal-apple-darwin/release/bundle/macos/Junk.app"
```

The ad-hoc signature is not trusted by Gatekeeper but allows users to install by removing the quarantine flag. Free, no Apple account required.

---

## CI / Release Pipeline

Every push to a `v*` tag triggers the GitHub Actions workflow — a 3-platform matrix:

| Runner | Artifacts |
|---|---|
| macOS | `Junk_2.8.0_universal.dmg` |
| Windows | `Junk_2.8.0_x64-setup.exe` + `Junk_2.8.0_x64_en-US.msi` |
| Ubuntu | `Junk_2.8.0_amd64.AppImage` + `Junk_2.8.0_amd64.deb` |

All artifacts are uploaded to a GitHub Release and auto-published with changelog.

<details>
<summary><strong>Hard-won CI lessons</strong></summary>

| Problem | Fix |
|---|---|
| `libwebkit2gtk-4.0-dev` not found on Ubuntu 22.04+ | Use `libwebkit2gtk-4.1-dev` |
| `actions/setup-node@v4` cache fails | Commit `package-lock.json` — `npm ci` requires it |
| `gh release create` fails if release exists | Use `gh release view` + `gh release upload --clobber` |
| Stale artifacts from old versions reappear | Filter by version string; clean with `gh api DELETE /releases/assets/{id}` |
| `SUPER|CONTROL` modifier requires both keys on macOS | Use `#[cfg(target_os)]` to select SUPER vs CONTROL |
| `resp.json().await` fails with tauri-plugin-http's reqwest | Use `.text().await` + `serde_json::from_str()` |

</details>

---

## FAQ

**Q: Why does macOS say the app is "damaged"?**
It's not damaged — it's quarantined. Run `xattr -rd com.apple.quarantine /Applications/Junk.app && open /Applications/Junk.app` to remove the flag.

**Q: Why isn't Junk in the Mac App Store?**
The App Store sandbox prevents apps from registering global hotkeys that work across all applications. Junk's core feature is incompatible with the sandbox.

**Q: Will Junk phone home or collect analytics?**
No telemetry, no analytics, no crash reporting. The only outbound request is the optional update check to `api.github.com`. Disable it in Preferences.

**Q: How do I truly quit Junk?**
Activity Monitor (macOS) or Task Manager (Windows) → find "Junk" → force-quit. By design, ⌘Q and × hide the window so the global shortcut stays alive.

**Q: ⌘J conflicts with another app. Can I change it?**
Not yet via UI — it's hardcoded in `main.rs` (`Code::KeyJ`). Change `KeyJ` to any `Code::*` variant and rebuild. A custom shortcut preference pane is on the roadmap.

**Q: Where is my data if I uninstall?**
`~/Library/WebKit/com.paulfleury.junk/` on macOS. Delete that folder to wipe everything.

**Q: Does it work natively on Apple Silicon?**
Yes. The `.dmg` is a universal binary — native ARM64 on Apple Silicon, native x86_64 on Intel. No Rosetta needed.

**Q: Why no iOS / Android?**
The global-hotkey model doesn't translate to mobile — there's no system-wide keyboard shortcut on iOS or Android.

**Q: I'm on Wayland and the shortcut doesn't work.**
Run `XDG_SESSION_TYPE=x11` before starting Junk, or enable XWayland in your compositor. This is a Tauri v2 limitation.

**Q: Will you add sync / cloud backup?**
No. Sync means a backend, account system, network requests, and failure modes. Junk is zero-infrastructure by design.

**Q: Why Space Grotesk?**
It sits at the intersection of technical precision and warmth. At 22 px with 1.8 line-height, it feels like writing on expensive paper — wide and generous, with room for each word to breathe.

---

## Changelog

### v2.8.0 — 2026-04-14
- **Fix:** Added `withGlobalTauri: true` to `tauri.conf.json` — this was missing, causing `window.__TAURI__?.core?.invoke` to be `undefined` on all IPC calls (drag, prefs, update check, launch at login all silently failed).
- **Fix:** Added dedicated `start_dragging` Rust IPC command — calls `window.start_dragging()` directly. JS now calls `e.preventDefault()` before the async invoke, keeping the OS drag candidate state open long enough for the native NSWindow drag API to activate.
- **Fix:** Rewritten JS IPC layer — uses `window.__TAURI_INTERNALS__.invoke` as primary (always available) with `window.__TAURI__.core.invoke` as fallback. All prefs panel functions (version display, update check, launch at login, auto-update) now work correctly.
- **Fix:** Prefs panel event listeners rewired — `open-prefs` Tauri event now correctly opens the panel; version number loads on every prefs open.

### v2.7.1 — 2026-04-14
- **Fix:** Correct Tauri IPC command for window drag — `invoke('plugin:window|start_dragging', { label: 'main' })`. The v2.7.0 call (`invoke('start_dragging')`) was silently ignored; the correct path includes the plugin namespace.

### v2.7.0 — 2026-04-14
- **Fix:** Window dragging now works on macOS. `-webkit-app-region: drag` is silently ignored by WKWebView on transparent frameless windows — replaced with a native `startDragging()` IPC call on `mousedown`. The OS window manager handles the drag directly, bypassing the WebView layer.
- **Feature:** `grab` cursor appears on draggable zones (anywhere except textarea, buttons, prefs panel) — makes the affordance discoverable.
- **Removed:** All `-webkit-app-region` CSS.

### v2.6.0 — 2026-04-14
- **Fix:** Removed system tray / menu bar icon — intrusive and unnecessary. ⌘Q now quits for real again.
- **Feature:** Full-window drag via `-webkit-app-region: drag` on `.window` (later replaced in v2.7.0).
- **Removed:** Footer drag-handle pill (⠿), `tauri-plugin-tray` dependency, all tray handlers.

<details>
<summary><strong>Older releases (v2.5.0 → v1.1.0)</strong></summary>

### v2.5.0 — 2026-04-14
- **Feature:** System tray / menu bar icon (later removed in v2.6.0).
- **Fix:** Update checker moved from JS `window.fetch()` to Rust `tauri-plugin-http` — the old JS fetch was blocked by WebView CSP.
- **Architecture:** `UpdateResult` struct — `check_for_update()` returns `{ up_to_date, current, latest, url }`.

### v2.4.0 — 2026-04-14
- **Fix:** Removed `alwaysOnTop: true` — other apps' modals now appear above Junk as expected.
- **Feature:** Footer drag handle — centered six-dot pill (⠿) as window-move region.

### v2.3.0 — 2026-04-14
- **Feature:** Persistent process — `CloseRequested` and `ExitRequested` now call `window.hide()` instead of quitting.
- **Feature:** Preferences panel — in-window frosted-glass sheet, ⌘, / Ctrl+, to open. Launch at login, auto-update, credits.
- **Feature:** Launch at login via `tauri-plugin-autostart` (LaunchAgent on macOS, no root required).
- **Feature:** Update checker — calls GitHub Releases API from JS.
- **Feature:** Three OS-level global shortcuts: ⌘J/Ctrl+J, Esc, ⌘,/Ctrl+,.

### v2.2.1 — 2026-04-14
- **Fix:** `invoke()` now resolved lazily on every call — fixes a race between `<script type="module">` deferred execution and Tauri's async `window.__TAURI__` injection.
- **Fix:** `window.focus()` before `editor.focus()` on `tauri://focus` — fixes Esc on macOS WKWebView.

### v2.2.0 — 2026-04-14
- **Fix:** Esc registered as OS-level global shortcut in Rust — bypasses WebView entirely. Previously unreliable.

### v2.1.0 — 2026-04-14
- **Fix:** ⌘J now works correctly on macOS — modifier is now platform-conditional (`SUPER` on macOS, `CONTROL` on Win/Linux) instead of combined `SUPER | CONTROL`.
- **Feature:** Landing page supports 30+ languages with flag-based modal switcher.

### v2.0.0 — 2026-04-10
- **Feature:** Scramble/typo effect on landing page hero text.
- **Feature:** i18n in 5 languages: EN, FR, ES, DE, JA.
- **Feature:** Per-OS download modals with platform-specific install instructions.
- **Improvement:** Full a11y pass — focus rings, ARIA labels, keyboard navigation, `prefers-reduced-motion`.

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

1. **Comments explain the *why***, not the *what*. See `main.rs` for the style.
2. **No `unwrap()` in non-test code.** Use `map_err`, `ok_or`, `if let`, or `?`.
3. **No new npm runtime dependencies.** The frontend is intentionally dependency-free. Inline the relevant parts if you need a library.
4. **No new Cargo crates without a compelling reason.** Every dependency is more compile time and attack surface.
5. **Test on all three platforms before opening a PR.**
6. **Preserve the comment density in `main.rs`.** Every function should have a block comment explaining what it does and why.

---

## Roadmap

- **Custom shortcut** via Preferences — change ⌘J to any key combo
- **Window position memory** — remember where you dragged it between sessions
- **Font size preference** — currently fixed at 22px / 1.8 line-height
- **Dark mode** — automatic or manual toggle
- **Markdown rendering mode** — toggle between raw text and rendered preview (off by default)
- **Plain-text export** — one-click copy of everything to clipboard

---

## Built with Perplexity Computer

This app — from the Rust rewrite to the landing page to this README — was built using [Perplexity Computer](https://www.perplexity.ai/computer), an AI agent that writes production-quality code, reasons about architecture, and explains every decision inline.

---

## Author

**Paul Fleury** — [paulfleury.com](https://paulfleury.com) · [GitHub @paulfxyz](https://github.com/paulfxyz)

---

## License

[MIT](LICENSE) — do whatever you want with it.
