# 🗒️ Junk

<div align="center">

```
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║    🗒️  J U N K                                          ║
║        the flying scratchpad                             ║
║                                                          ║
║    ⌘J  /  Ctrl+J  →  window flies in                   ║
║    type, paste, think                                    ║
║    Esc  →  gone                                          ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

[![Version](https://img.shields.io/badge/version-1.5.0-4F98A3?style=flat-square&labelColor=12110e)](https://github.com/paulfxyz/junk/releases/latest)
[![macOS](https://img.shields.io/badge/macOS-supported-000000?style=flat-square&logo=apple&logoColor=white)](https://github.com/paulfxyz/junk/releases/latest)
[![Windows](https://img.shields.io/badge/Windows-supported-0078D4?style=flat-square&logo=windows&logoColor=white)](https://github.com/paulfxyz/junk/releases/latest)
[![Linux](https://img.shields.io/badge/Linux-supported-FCC624?style=flat-square&logo=linux&logoColor=black)](https://github.com/paulfxyz/junk/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-white?style=flat-square)](LICENSE)
[![Built with Electron](https://img.shields.io/badge/built_with-Electron-47848F?style=flat-square&logo=electron&logoColor=white)](https://electronjs.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-4F98A3?style=flat-square)](https://github.com/paulfxyz/junk/pulls)

**A global-hotkey scratchpad for macOS, Windows, and Linux. One shortcut. No chrome. No cloud. Just a place to dump your brain.**

[Download](https://github.com/paulfxyz/junk/releases/latest) · [Report Bug](https://github.com/paulfxyz/junk/issues) · [Request Feature](https://github.com/paulfxyz/junk/issues)

</div>

---

## What is Junk?

Junk is a background scratchpad app. It sits invisible — no Dock icon, no tray icon, no menu bar presence — until you need it.

Press `⌘J` (macOS) or `Ctrl+J` (Windows / Linux) from *any* app, any context. A frosted glass window flies in at the center of your screen. Type. Paste. Think. Press `Esc` or the shortcut again to dismiss. Everything is auto-saved between launches.

No accounts. No sync. No settings screen. No cloud. Just a fast, frictionless place to dump text.

---

## Why it exists

Every developer and knowledge worker hits the same wall: something you need to write down *right now*.

An API key you just copied. A quick note from a call. A URL to revisit. A half-formed thought before it evaporates. A command you haven't run yet. A snippet to transform before pasting somewhere.

The usual suspects all fail here:
- **Notes.app / Notepad** — takes too long, wrong context
- **Slack yourself** — you'll never find it again
- **Desktop text files** — `untitled 47.txt`
- **Browser address bar** — don't
- **Terminal** — you didn't want to be there

Junk is `⌘J` / `Ctrl+J`. It's there in under 200ms. You dump the thing. You hit `Esc`. It's gone. The text is still there when you come back.

---

## Features

| | |
|---|---|
| **Global hotkey** | `⌘J` on macOS · `Ctrl+J` on Windows and Linux · works from any app, any fullscreen window |
| **Toggle** | Same shortcut shows and hides |
| **Esc to close** | Fastest way out |
| **Persistent window** | Switching apps does not close Junk — copy text, run a clipboard tool, come back, it's still there |
| **Auto-save** | Content survives restarts — stored locally via `localStorage` |
| **Frosted glass UI** | macOS: native vibrancy (`under-window`) · Windows/Linux: backdrop-filter blur |
| **Space Grotesk font** | 22px, line-height 1.80 — comfortable for dumping thoughts fast |
| **Fully frameless** | No title bar, no traffic lights, no window chrome |
| **Drag anywhere** | Grab any non-text area to reposition |
| **No Dock icon** | Stays completely out of `⌘Tab` / `Alt+Tab` |
| **No tray icon** | Truly invisible — just the hotkey |
| **Paste anywhere** | Even if the textarea isn't focused, paste works |
| **Cross-platform** | macOS (arm64 + x64) · Windows (installer + portable) · Linux (AppImage + deb) |

---

## Install

### macOS

> **Apple Silicon (M1 / M2 / M3 / M4)** → `Junk-1.5.0-arm64.dmg`
> **Intel Mac** → `Junk-1.5.0-x64.dmg`

1. Download from [Releases](https://github.com/paulfxyz/junk/releases/latest)
2. Open the `.dmg` → drag **Junk.app** to `/Applications`
3. **Required first-launch step** — Junk is not notarized with an Apple Developer certificate. Run this in Terminal:

```bash
xattr -rd com.apple.quarantine /Applications/Junk.app
open /Applications/Junk.app
```

That's it. Junk starts silently in the background. Press **`⌘J`** from anywhere.

> **Why this step?** macOS Gatekeeper quarantines apps downloaded from the internet that aren't signed with a paid Apple Developer certificate ($99/year). The `xattr` command removes that quarantine flag. You only need to do this once — after that Junk launches normally on login.

---

### Windows

> `Junk-Setup-1.5.0.exe` — one-click installer
> `Junk-1.5.0.exe` — portable (no install needed)

1. Download the installer or portable from [Releases](https://github.com/paulfxyz/junk/releases/latest)
2. Run it — Windows SmartScreen may warn about an unknown publisher (click **More info → Run anyway**)
3. Press **`Ctrl+J`** from anywhere

To launch on startup: add a shortcut to `Junk.exe` in `shell:startup`.

---

### Linux

> `Junk-1.5.0.AppImage` — universal, no install
> `junk_1.5.0_amd64.deb` — Debian / Ubuntu

**AppImage:**
```bash
chmod +x Junk-1.5.0.AppImage
./Junk-1.5.0.AppImage
```

**Debian / Ubuntu:**
```bash
sudo dpkg -i junk_1.5.0_amd64.deb
junk
```

Then press **`Ctrl+J`** from anywhere.

To autostart on login, add Junk to your desktop environment's startup applications.

---

## Usage

| Action | macOS | Windows / Linux |
|---|---|---|
| Open / close | `⌘J` | `Ctrl+J` |
| Close | `Esc` | `Esc` |
| Move window | Drag footer or border | Drag footer or border |
| Select all | `⌘A` | `Ctrl+A` |
| Clear all | `⌘A` → `Delete` | `Ctrl+A` → `Delete` |

---

## Build from source

**Requirements:** Node.js 20+, macOS (for `.icns` generation)

```bash
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# macOS only — generate ICNS icon
iconutil -c icns assets/icon.iconset -o assets/icon.icns

# Run in development
npm start

# Build for your platform
npm run build:mac    # → dist/*.dmg
npm run build:win    # → dist/*.exe
npm run build:linux  # → dist/*.AppImage + *.deb
```

---

## Under the Hood

> How Junk works — for developers who want to understand or fork it.

### Global shortcut

```js
globalShortcut.register('CommandOrControl+J', toggleWindow)
```

`CommandOrControl` maps to `⌘` on macOS and `Ctrl` on Windows/Linux automatically. Registered after `app.whenReady()`, unregistered on `will-quit`. Intercepts the key at OS level — works inside any focused app, fullscreen game, or terminal.

### Native window, no chrome

```js
{
  titleBarStyle:        'hiddenInset',
  trafficLightPosition: { x: -100, y: -100 },  // pushed off-screen
  transparent:          true,
  vibrancy:             'under-window',          // macOS glass
}
```

`titleBarStyle: 'hiddenInset'` gives a real macOS native window — proper shadow, proper compositor rounding, proper vibrancy material — but hides the title bar. The traffic light buttons are pushed to `{x: -100, y: -100}` so they're invisible and unreachable. No fake CSS border-radius, no frameless-window hacks.

On Windows and Linux, `transparent: true` + CSS `backdrop-filter: blur(40px)` gives a similar frosted effect without native vibrancy.

### Persistent window

The blur event is intentionally **not** handled. Most scratchpads close when you click away — Junk does not. This is the key design decision: you can copy text from Junk, switch to another app, run a clipboard processing tool, and come back. The window is exactly where you left it.

### Content persistence

```js
localStorage.setItem('junk:content', pad.value)
```

Electron stores `localStorage` at:
- macOS: `~/Library/Application Support/junk/`
- Windows: `%APPDATA%\junk\`
- Linux: `~/.config/junk/`

Saves are debounced 300ms after each keystroke — never blocks the UI.

### Fly-in animation

```css
@keyframes in {
  from { opacity: 0; transform: scale(0.96) translateY(-10px); }
  to   { opacity: 1; transform: scale(1)    translateY(0);     }
}
animation: in 180ms cubic-bezier(0.22, 1, 0.36, 1) both;
```

`cubic-bezier(0.22, 1, 0.36, 1)` is a fast ease-out — snappy entry with no spring overshoot. 180ms is imperceptible as delay but makes the appearance feel intentional rather than jarring.

### Dock / taskbar hiding

```js
app.dock?.hide()          // macOS — removes from Dock and ⌘Tab
// skipTaskbar: true      // Windows/Linux — removes from taskbar
```

The `?.` optional chain handles non-macOS platforms gracefully. Combined with `skipTaskbar: true` in the `BrowserWindow` config, Junk is invisible in every app switcher on every platform.

---

## Project Structure

```
junk/
├── src/
│   ├── main.js        # Electron main process — window, shortcuts, IPC
│   ├── preload.js     # Context bridge — safe IPC API for the renderer
│   ├── index.html     # Window markup
│   ├── style.css      # Frosted glass UI, Space Grotesk, animation
│   └── renderer.js    # Persistence, keyboard shortcuts, focus, paste
├── assets/
│   ├── icon.iconset/  # macOS icon set (16–1024px)
│   ├── icon.icns      # Compiled macOS icon (generated at build time)
│   ├── icon.png       # Flat PNG — Windows, Linux, README
│   └── dmg-background.png
├── .github/
│   └── workflows/
│       └── build.yml  # CI: 4 parallel jobs → macOS arm64, macOS x64, Windows, Linux
├── package.json
├── README.md
├── INSTALL.md
├── CHANGELOG.md
└── LICENSE
```

---

## CI / Build pipeline

Four parallel GitHub Actions jobs — each on a native runner for its platform:

| Job | Runner | Output |
|---|---|---|
| macOS arm64 | `macos-14` (Apple Silicon) | `Junk-*.arm64.dmg` |
| macOS x64 | `macos-14` (cross-compile) | `Junk-*.dmg` |
| Windows | `windows-latest` | `Junk-Setup-*.exe` + `Junk-*.exe` (portable) |
| Linux | `ubuntu-latest` | `Junk-*.AppImage` + `junk_*.deb` |

All macOS builds are ad-hoc signed (leaves → root, `--options runtime --timestamp=none`) before DMG packaging. The release job collects all artifacts and publishes them to the GitHub Release.

---

## Built with Perplexity Computer

Designed and shipped in a single session with **[Perplexity Computer](https://www.perplexity.ai)** — from idea to cross-platform `.dmg` / `.exe` / `.AppImage`. All code, CI pipeline, signing, and documentation generated and iterated in one thread.

---

## Contributing

PRs welcome. Keep the philosophy: one thing, fast, no settings, no accounts.

---

## Author

**Paul Fleury** — French internet entrepreneur based in Lisbon.

[paulfleury.com](https://paulfleury.com) · [GitHub](https://github.com/paulfxyz) · [LinkedIn](https://www.linkedin.com/in/paulfxyz/)

---

## License

MIT © [Paul Fleury](https://paulfleury.com)
