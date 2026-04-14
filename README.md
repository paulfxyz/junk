```
     ██╗██╗   ██╗███╗   ██╗██╗  ██╗
     ██║██║   ██║████╗  ██║██║ ██╔╝
     ██║██║   ██║██╔██╗ ██║█████╔╝
██   ██║██║   ██║██║╚██╗██║██╔═██╗
╚█████╔╝╚██████╔╝██║ ╚████║██║  ██╗
 ╚════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝

  a global-hotkey scratchpad — built with Rust + Tauri v2
```

[![Version](https://img.shields.io/badge/version-2.0.0-5b5bf6?style=flat-square)](https://github.com/paulfleury/junk/releases)
[![macOS](https://img.shields.io/badge/macOS-10.15%2B-black?style=flat-square&logo=apple)](https://github.com/paulfleury/junk/releases)
[![Windows](https://img.shields.io/badge/Windows-10%2B-0078d4?style=flat-square&logo=windows)](https://github.com/paulfleury/junk/releases)
[![Linux](https://img.shields.io/badge/Linux-AppImage%20%7C%20deb-fcc624?style=flat-square&logo=linux&logoColor=black)](https://github.com/paulfleury/junk/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-22c55e?style=flat-square)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-ffc131?style=flat-square&logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)

---

## What is Junk?

Junk is a **global-hotkey scratchpad**. Press **⌘J** (macOS) or **Ctrl+J** (Windows/Linux) — anywhere, any app — and a clean floating notepad appears. Press it again, or hit **Esc**, and it vanishes. Your words are always there when you come back.

No accounts. No sync. No cloud. Just a place to put the thing you're thinking about right now.

It's the app you open before you know what you want to say.

---

## Why Rust / Tauri?

Junk started as an [Electron](https://github.com/paulfleury/junk) app. Electron works, but for something this simple it carried a cost:

| Concern | Electron | Tauri v2 + Rust |
|---|---|---|
| Installer size | ~160 MB | ~4 MB |
| RAM at rest | ~130 MB | ~18 MB |
| Cold start | ~600 ms | ~80 ms |
| Node.js required | Yes | No |
| Native system APIs | Via bindings | Direct (Rust) |
| Memory safety | GC + JS heap | Rust ownership model |
| Shortcut registration | `electron-globalShortcut` | `tauri-plugin-global-shortcut` |

The Rust/Tauri version is a **drop-in replacement** — same keyboard shortcut, same glass aesthetic, same localStorage persistence — but ships as a ~4 MB binary with no runtime dependency.

---

## Features

| Feature | Description |
|---|---|
| ⌘J / Ctrl+J | Global hotkey — works in any app, any Space |
| Esc | Hides the window from inside the app |
| Auto-save | Content persists to `localStorage` with 300 ms debounce |
| Frosted glass | `backdrop-filter: blur(40px) saturate(180%)` — beautiful on any wallpaper |
| Always on top | Floats above all other windows |
| Frameless | No title bar — drag anywhere on the window to move it |
| No Dock icon | Stays out of your way (macOS: Accessory activation policy) |
| No Taskbar icon | `skipTaskbar: true` on Windows/Linux |
| Fly-in animation | 180 ms spring easing — feels alive, not jarring |
| Paste anywhere | Paste (⌘V/Ctrl+V) even without clicking into the textarea first |
| Universal binary | macOS: native on both Apple Silicon and Intel in one file |
| Zero dependencies | No Node.js, no runtime, no update daemon |

---

## Install

### macOS

1. Download `Junk_1.6.0_universal.dmg` from [Releases](https://github.com/paulfleury/junk/releases)
2. Open the DMG, drag **Junk** to **Applications**
3. Remove the quarantine flag (required because the app is not notarised with an Apple Developer ID):
   ```sh
   xattr -dr com.apple.quarantine /Applications/Junk.app
   ```
   **Why is this necessary?** macOS Gatekeeper quarantines apps downloaded from the internet that aren't signed by an Apple Developer account ($99/yr). The `xattr` command removes the quarantine attribute so Gatekeeper lets the app open. It is safe — you're telling macOS "I trust this file I downloaded intentionally."

4. Launch Junk from Spotlight or Applications. It will not appear in the Dock — that's intentional.
5. Press **⌘J** from any app.

### Windows

1. Download `Junk_1.6.0_x64-setup.exe` from [Releases](https://github.com/paulfleury/junk/releases)
2. Run the installer. Windows SmartScreen may show a warning: click **More info** → **Run anyway**

   **Why SmartScreen?** The binary is not code-signed with an Extended Validation certificate ($200–500/yr). SmartScreen warns for any unsigned installer. The source code is fully open — you can build it yourself (see below) if you prefer.

3. Press **Ctrl+J** from any app.

### Linux (AppImage)

```sh
# Download
wget https://github.com/paulfleury/junk/releases/latest/download/junk_1.6.0_amd64.AppImage

# Make executable
chmod +x junk_1.6.0_amd64.AppImage

# Run
./junk_1.6.0_amd64.AppImage
```

> **Note for Wayland users:** Junk uses the X11 global shortcut mechanism via `libxdo`. If you're on a pure Wayland session (no XWayland), the global shortcut may not register. Run with `XDG_SESSION_TYPE=x11` or enable XWayland in your compositor. This is a Tauri v2 limitation — Wayland's security model intentionally prevents global shortcuts.

### Linux (deb)

```sh
sudo dpkg -i junk_1.6.0_amd64.deb
```

---

## Usage

| Action | Result |
|---|---|
| **⌘J** (macOS) / **Ctrl+J** (Win/Linux) | Toggle the window (show if hidden, hide if visible) |
| **Esc** | Hide the window |
| Click + drag anywhere on the window | Move the window |
| **⌘A** / Ctrl+A | Select all text in the textarea |
| **⌘V** / Ctrl+V | Paste — works even without clicking the textarea first |
| Type | Your words are saved automatically (300 ms after you stop) |
| Close the window (✕) | Not possible — Junk has no title bar. Use Esc or ⌘J instead. |

Content is stored in the WebView's `localStorage` under the key `junk-content`. It persists across restarts as long as you don't clear the app's local data.

---

## Build from Source

### Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | stable (≥ 1.70) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Node.js | 20 LTS | [nodejs.org](https://nodejs.org) |
| macOS additional | Xcode Command Line Tools | `xcode-select --install` |
| Linux additional | WebKitGTK + build tools | See below |

**Linux system dependencies:**
```sh
sudo apt-get update && sudo apt-get install -y \
  build-essential libssl-dev libgtk-3-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf libxdo-dev
```

### Steps

```sh
# Clone the repo
git clone https://github.com/paulfleury/junk.git
cd junk

# Install JS tooling (just @tauri-apps/cli)
npm install

# Development build — opens a hot-reload WebView window
npm run dev

# Production build — outputs to src-tauri/target/release/bundle/
npm run build
```

**macOS universal binary:**
```sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin
```

The release binary is at:
- macOS:   `src-tauri/target/universal-apple-darwin/release/bundle/dmg/`
- Windows: `src-tauri/target/release/bundle/nsis/` and `/msi/`
- Linux:   `src-tauri/target/release/bundle/appimage/` and `/deb/`

---

## Under the Hood

### Global Shortcut

The **⌘J / Ctrl+J** shortcut is registered at the OS level using [`tauri-plugin-global-shortcut`](https://v2.tauri.app/plugin/global-shortcut/), which wraps platform-native APIs:

- **macOS**: Carbon's `RegisterEventHotKey` (same as every Mac menu-bar app)
- **Windows**: `RegisterHotKey` Win32 API
- **Linux**: `XGrabKey` via `libxdo`

The shortcut descriptor uses combined modifiers (`SUPER | CONTROL`) so a single registration works cross-platform — the OS only activates when the platform-correct modifier is pressed.

When the shortcut fires, the Rust callback calls `window.is_visible()` to decide whether to show or hide, then calls `window.show()` + `window.set_focus()` or `window.hide()`. No state machine — just two OS calls.

### Window Persistence

The window is **never destroyed** — only shown and hidden. This is a deliberate architectural choice:

1. **Speed**: Showing an existing hidden window takes ~5 ms. Creating a new window + loading HTML takes ~200–400 ms.
2. **State preservation**: The textarea cursor position, scroll position, and undo history survive hide/show cycles.
3. **localStorage survives**: Because it's the same WebView instance, all stored data is intact.

The window starts with `visible: false` in `tauri.conf.json` and is never automatically made visible — only the global shortcut or `window.show()` from Rust can surface it.

### No-Blur-Hide Policy

Junk intentionally does **not** hide when it loses focus (`blur`). This enables clipboard workflows:

1. Press ⌘J → Junk appears
2. Copy something from Junk
3. Click into another app (Junk loses focus)
4. Paste into the other app
5. Cmd+Tab back to Junk — it's still there

If Junk hid on blur (like Alfred, for example), step 3 would make it vanish and break the workflow.

### Content Storage

```
textarea input
     │
     ▼ (300 ms debounce)
localStorage.setItem('junk-content', value)
     │
     ▼ (on startup)
localStorage.getItem('junk-content') → textarea.value
```

The debounce batches rapid typing into single writes. `localStorage` is synchronous and survives app restarts because the Tauri WebView maintains its own profile directory per app identifier (`com.paulfleury.junk`).

### Fly-in Animation

Every time the window appears, it plays a scale + translateY entry:

```css
@keyframes fly-in {
  from { opacity: 0; transform: scale(0.96) translateY(-10px); }
  to   { opacity: 1; transform: scale(1)    translateY(0);      }
}
/* Duration: 180ms  Easing: cubic-bezier(0.22, 1, 0.36, 1) */
```

CSS animations only play once per element lifecycle. To replay it on each show, we:
1. Remove `animationName` from the element's style
2. Force a style recalculation (`void el.offsetWidth`)
3. Restore `animationName` — the browser sees this as a new animation and plays it

The `tauri://focus` event from Tauri's WebView signals that the window was just shown — that's the hook for triggering the re-play.

### Tauri IPC

The frontend invokes one Rust command:

```
JS: invoke('hide_window')
        │
        ▼
Rust: fn hide_window(app: AppHandle) → Result<(), String>
        │
        └─ app.get_webview_window("main")?.hide()
```

`AppHandle` is dependency-injected by Tauri — the JS caller passes no arguments. The command is registered in `main.rs` via `tauri::generate_handler![hide_window]` and permitted in `capabilities/default.json`.

---

## Project Structure

```
junk/
├── src/
│   └── index.html              ← Single-file frontend (HTML + CSS + JS)
├── src-tauri/
│   ├── src/
│   │   └── main.rs             ← Rust backend: shortcuts, window, IPC
│   ├── capabilities/
│   │   └── default.json        ← Tauri v2 permission grants
│   ├── Cargo.toml              ← Rust dependencies
│   ├── build.rs                ← Tauri build script (code generation)
│   └── tauri.conf.json         ← Window config, bundle settings
├── assets/
│   └── icons/                  ← App icons (icns, ico, png) — see assets/README.txt
├── .github/
│   └── workflows/
│       └── build.yml           ← CI: build + release on all 3 platforms
├── package.json                ← npm scripts, @tauri-apps/cli
└── README.md
```

---

## Built with Perplexity Computer

This Rust/Tauri rewrite was generated using [Perplexity Computer](https://www.perplexity.ai/computer) — an AI agent that writes production-quality code, reasons about architecture, and explains decisions inline. Every comment in the source is part of the output.

If you want to understand how this app works, read the source — it's designed to be a reference implementation.

---

## Author

**Paul Fleury** — [paulfleury.com](https://paulfleury.com) · [GitHub](https://github.com/paulfleury)

---

## License

[MIT](LICENSE) — do whatever you want with it.
