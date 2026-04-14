# 🗒️ Junk

<div align="center">

```
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║    🗒️  J U N K                                          ║
║        the flying macOS scratchpad                       ║
║                                                          ║
║    ⌘J  →  window flies in                               ║
║    type, paste, think                                    ║
║    Esc  →  gone                                          ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

[![Version](https://img.shields.io/badge/version-1.4.0-4F98A3?style=flat-square&labelColor=12110e)](https://github.com/paulfxyz/junk/releases/latest)
[![Platform](https://img.shields.io/badge/platform-macOS-000000?style=flat-square)](https://github.com/paulfxyz/junk/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-white?style=flat-square)](LICENSE)
[![Built with Electron](https://img.shields.io/badge/built_with-Electron-47848F?style=flat-square&logo=electron&logoColor=white)](https://electronjs.org)
[![Status](https://img.shields.io/badge/status-active-22c55e?style=flat-square)](https://github.com/paulfxyz/junk/releases/latest)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-4F98A3?style=flat-square)](https://github.com/paulfxyz/junk/pulls)

**A background scratchpad for macOS. One shortcut. No chrome. No cloud. Just a place to dump your brain.**

[Download](https://github.com/paulfxyz/junk/releases/latest) · [Report Bug](https://github.com/paulfxyz/junk/issues) · [Request Feature](https://github.com/paulfxyz/junk/issues)

</div>

---

## What is Junk?

Junk is a global-hotkey scratchpad that lives silently in your macOS menu bar. Press `⌘J` from *any* app, any Space, any context — a dark glass window flies in at the center of your screen. Type. Paste. Think. Press `Esc` to dismiss. Everything is auto-saved between launches.

No accounts. No sync. No settings screen. No Dock icon. No cloud. Just a fast, frictionless place to dump text.

---

## Why it exists

Every developer has the same problem: something you need to write down *right now* — an API key you just copied, a quick note from a meeting, a URL to revisit, a command you haven't run yet, a half-formed thought.

The alternatives all suck for this:
- **Notes.app** — takes too long to open, puts you in the wrong context
- **Sticky Notes** — clutter
- **The Desktop** — a TextEdit file with "untitled 47.txt"
- **Slack yourself** — you read it, you forget it
- **Terminal** — you didn't want to be in the terminal

Junk is `⌘J`. It's there in 160ms. You dump the thing. You hit `Esc`. It's gone. The text is still there when you need it again.

---

## Features

| | |
|---|---|
| **⌘J global hotkey** | Works from any app, any Space, any fullscreen window |
| **Toggle** | Same shortcut shows and hides — no separate dismiss shortcut to remember |
| **Auto-hide on blur** | Click anywhere outside → Junk disappears. Exactly like Spotlight |
| **Auto-save** | Content survives restarts. No manual save, no prompt |
| **Fly-in animation** | 160ms cubic-bezier spring — fast enough to feel instant |
| **Frameless glass UI** | macOS vibrancy, near-black warm background, no window chrome |
| **Large mono font** | 22px SF Mono — big enough to actually read while you're dumping things |
| **Menu bar icon** | Small `J` in your menu bar — right-click for toggle and quit |
| **No Dock icon** | Stays completely out of your `⌘Tab` rotation |
| **Esc to close** | Fastest exit |
| **Paste anywhere** | Paste even if the textarea isn't focused — Junk catches it |
| **Draggable** | Grab the title bar, move it wherever |

---

## Install

### Download (recommended)

1. Go to [**Releases**](https://github.com/paulfxyz/junk/releases/latest)
2. Download the right `.dmg` for your Mac:
   - **Apple Silicon (M1 / M2 / M3 / M4)** → `Junk-1.4.0-arm64.dmg`
   - **Intel Mac** → `Junk-1.4.0-x64.dmg`
3. Open the `.dmg` → drag **Junk.app** to `/Applications`
4. First launch — strip the quarantine flag:

```bash
xattr -rd com.apple.quarantine /Applications/Junk.app
open /Applications/Junk.app
```

Or: right-click **Junk.app** → **Open** → **Open**.

> Junk is not notarized with an Apple Developer certificate. The quarantine step is a one-time bypass — you only need to do it once.

See [INSTALL.md](INSTALL.md) for full instructions.

---

## Usage

| Action | How |
|---|---|
| Open / close | `⌘J` from anywhere |
| Close | `Esc` or the `×` button |
| Move the window | Drag the title bar |
| Select all text | `⌘A` |
| Clear everything | `⌘A` → `Delete` |
| Quit the app | Menu bar `J` icon → **Quit Junk** |

---

## Build from source

**Requirements:** Node.js 20+, macOS (for `iconutil`)

```bash
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# Generate the .icns icon (macOS only)
iconutil -c icns assets/icon.iconset -o assets/icon.icns

# Run in development
npm start

# Build DMG
npm run build
# → dist/Junk-1.4.0-arm64.dmg
# → dist/Junk-1.4.0-x64.dmg
```

---

## Under the Hood

> A technical breakdown of how Junk works — for developers who want to understand or fork it.

### Global shortcut registration

```js
globalShortcut.register('CommandOrControl+J', toggleWindow)
```

Registered after `app.whenReady()`, unregistered on `will-quit`. This intercepts `⌘J` at the OS level *before* any focused app sees it — so it works inside VS Code, browsers, terminals, fullscreen apps, everything.

### Frameless + transparent window

`BrowserWindow` is created with `frame: false` and `transparent: true`. This gives macOS's compositor full control over the window shape. The `vibrancy: 'under-window'` setting activates the native blur material — the same glass effect used by Spotlight and Control Center — which blurs whatever is behind Junk and adapts to the wallpaper.

### Auto-hide on blur

```js
win.on('blur', hideWindow)
```

When focus leaves the window for any reason — clicking another app, switching Space, pressing `⌘Tab` — Junk hides instantly. Same model as Spotlight or Alfred. No close button required.

### Content persistence

The renderer uses `localStorage`. Electron stores it in:

```
~/Library/Application Support/junk/
```

Saves are debounced 400ms after each keystroke — never blocks the UI, survives crashes.

### Fly-in animation

```css
@keyframes flyIn {
  from { opacity: 0; transform: translateY(-12px) scale(0.97); }
  to   { opacity: 1; transform: translateY(0)     scale(1);    }
}
animation: flyIn 160ms cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
```

`cubic-bezier(0.34, 1.56, 0.64, 1)` is a spring curve with a slight scale overshoot — it gives the window a physical "pop" without being distracting. 160ms is below the threshold of feeling slow.

### Dock hiding

```js
app.dock?.hide()
```

Called synchronously before `app.whenReady()`. The `?.` optional chain handles non-macOS platforms. This is why Junk doesn't appear in `⌘Tab`, the Dock, or Mission Control — it's a pure menu bar utility.

### Context isolation

The preload script uses `contextBridge.exposeInMainWorld` to expose a minimal `window.junk` API to the renderer. `nodeIntegration: false` means the renderer has zero Node.js access — only the explicit surface we hand it.

```js
contextBridge.exposeInMainWorld('junk', {
  onShown: (cb) => ipcRenderer.on('window-shown', cb),
  hide: () => ipcRenderer.send('hide-window'),
  platform: process.platform,
})
```

### Window pre-warming

The window is created on app ready — before any shortcut is pressed — so the first `⌘J` is instant. `win.show()` on a pre-created hidden window is much faster than creating a new one on demand.

---

## Project Structure

```
junk/
├── src/
│   ├── main.js        # Electron main process — window, tray, shortcuts, IPC
│   ├── preload.js     # Context bridge — safe IPC surface for the renderer
│   ├── index.html     # Window markup
│   ├── style.css      # UI — dark glass, large mono type, fly-in animation
│   └── renderer.js    # UI logic — persistence, keyboard shortcuts, focus
├── assets/
│   ├── icon.iconset/  # All macOS icon sizes (16px → 1024px)
│   ├── icon.icns      # Compiled icon (generated at build time by CI)
│   ├── icon.png       # Flat PNG for README / GitHub social preview
│   └── dmg-background.png
├── .github/
│   └── workflows/
│       └── build.yml  # CI: ad-hoc signs + packages DMG on every tag push
├── package.json
├── README.md
├── INSTALL.md
├── CHANGELOG.md
└── LICENSE
```

---

## Built with Perplexity Computer

Junk was designed and built in a single session with **[Perplexity Computer](https://www.perplexity.ai)** — from idea to shipped `.dmg`. Architecture, code, signing, CI pipeline, README, and GitHub release — all generated and iterated in one thread.

---

## Contributing

PRs welcome. Keep the philosophy: one thing, fast, no settings.

---

## Author

**Paul Fleury** — French internet entrepreneur based in Lisbon.

[paulfleury.com](https://paulfleury.com) · [GitHub](https://github.com/paulfxyz) · [LinkedIn](https://www.linkedin.com/in/paulfxyz/)

---

## License

MIT © [Paul Fleury](https://paulfleury.com)
