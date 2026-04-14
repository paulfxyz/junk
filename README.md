# 🗒️ Junk

> A flying macOS scratchpad. Hit `⌘J`, type or paste, hit `Esc`. That's it.

![Version](https://img.shields.io/github/v/release/paulfxyz/junk?style=flat-square&color=4F98A3)
![Platform](https://img.shields.io/badge/platform-macOS-black?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-white?style=flat-square)
![Built with Electron](https://img.shields.io/badge/built_with-Electron-47848F?style=flat-square&logo=electron)

---

## What is Junk?

Junk is a global keyboard shortcut scratchpad for macOS. It lives silently in your menu bar, invisible until you need it. Press `⌘J` from any app — a clean, dark, floating window appears at the center of your screen. Type, paste, think. Press `Esc` (or `⌘J` again) to make it vanish. Everything you type is auto-saved locally between launches.

No accounts. No cloud. No noise.

---

## Screenshot

```
╔══════════════════════════════════════════════════╗
║  junk                                          × ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  api key: sk-...                                 ║
║  meeting notes from 4pm                          ║
║  remember to call back                           ║
║  https://example.com/some-long-link              ║
║                                                  ║
╠══════════════════════════════════════════════════╣
║  ⌘J toggle  ·  Esc close  ·  content auto-saved ║
╚══════════════════════════════════════════════════╝
```

---

## Features

| Feature | Detail |
|---|---|
| **Global hotkey** | `⌘J` from any app, any space, anytime |
| **Toggle** | Same shortcut shows and hides — fast |
| **Auto-hide on blur** | Click anywhere outside → Junk disappears |
| **Auto-save** | Content persists across restarts via `localStorage` |
| **Fly-in animation** | 160ms cubic-bezier — snappy, not flashy |
| **Menu bar icon** | Tiny `J` tray icon with right-click menu |
| **Dark glass UI** | macOS vibrancy, near-black warm background |
| **Large monospaced font** | 22px SF Mono — readable dump-thoughts size |
| **No Dock icon** | Stays out of your way |
| **`Esc` to close** | Fastest way out |
| **`⌘A` select all** | Works natively in the textarea |
| **Paste anywhere** | Even if textarea isn't focused |

---

## Install

### From GitHub Releases (recommended)

1. Go to [Releases](https://github.com/paulfxyz/junk/releases/latest)
2. Download the `.dmg` for your Mac:
   - **Apple Silicon (M1/M2/M3/M4):** `Junk-*-arm64.dmg`
   - **Intel Mac:** `Junk-*-x64.dmg`
3. Open the `.dmg` → drag **Junk.app** to `/Applications`
4. First launch — bypass Gatekeeper (app is unsigned):
   ```bash
   xattr -rd com.apple.quarantine /Applications/Junk.app
   ```
   Or: right-click the app → **Open** → **Open**

See [INSTALL.md](INSTALL.md) for full instructions.

---

## Usage

| Action | How |
|---|---|
| Open / close | `⌘J` from any app |
| Close | `Esc` key or `×` button |
| Move window | Drag the title bar |
| Select all | `⌘A` |
| Clear all | `⌘A` then `Delete` |
| Quit entirely | Menu bar `J` icon → **Quit Junk** |

---

## Build from source

**Requirements:** Node.js 20+, macOS (for `.icns` generation)

```bash
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# Run in development
npm start

# Build DMG
iconutil -c icns assets/icon.iconset -o assets/icon.icns
npm run build
```

The built `.dmg` files appear in `dist/`.

---

## Under the Hood

Junk is a minimal [Electron](https://electronjs.org) app. Here's how the key mechanics work:

### Global shortcut registration
`globalShortcut.register('CommandOrControl+J', toggleWindow)` intercepts `⌘J` at the OS level before any app sees it. This is registered after `app.whenReady()` and unregistered on `will-quit` to be a good citizen.

### Frameless + transparent window
The `BrowserWindow` is created with `frame: false` and `transparent: true`, which lets macOS's compositor handle the window shape. The vibrancy effect (`vibrancy: 'under-window'`) blurs the content behind Junk — a native macOS material that adapts to the wallpaper.

### Auto-hide on blur
`win.on('blur', hideWindow)` — when focus leaves the window for any reason (clicking another app, switching spaces), Junk hides instantly. Same behavior as Spotlight or Alfred.

### Content persistence
The renderer uses `localStorage` to store the textarea content. Electron persists `localStorage` in `~/Library/Application Support/junk/`. The save is debounced 400ms after each keystroke — never blocks the UI.

### Fly-in animation
CSS `@keyframes flyIn` with `cubic-bezier(0.34, 1.56, 0.64, 1)` — a slight spring overshoot on the scale axis. Runs once per show (the animation is reset by toggling the window's `display` via class). Duration is 160ms — below the threshold of feeling slow.

### Dock hiding
`app.dock?.hide()` is called synchronously before `app.whenReady()`. The `?.` optional chain handles Linux/Windows where `.dock` doesn't exist. This is why Junk doesn't show in `⌘Tab` or the Dock — it's a menu bar utility.

### Context isolation
The preload script uses `contextBridge.exposeInMainWorld` to pass a minimal `window.junk` API to the renderer. `nodeIntegration: false` ensures the renderer has zero Node.js access — only the safe API surface we explicitly expose.

---

## Project structure

```
junk/
├── src/
│   ├── main.js        # Electron main process — window, tray, shortcuts
│   ├── preload.js     # Context bridge — safe IPC between main & renderer
│   ├── index.html     # Window markup
│   ├── style.css      # UI styles — dark glass, large type
│   └── renderer.js    # UI logic — persistence, keyboard shortcuts, focus
├── assets/
│   ├── icon.iconset/  # macOS icon set (all sizes)
│   ├── icon.icns      # Compiled icon (generated at build time)
│   ├── icon.png       # Flat PNG for README/GitHub
│   └── dmg-background.png
├── .github/
│   └── workflows/
│       └── build.yml  # CI: builds + publishes DMG on tag push
├── package.json
├── README.md
├── INSTALL.md
├── CHANGELOG.md
└── LICENSE
```

---

## Contributing

PRs welcome. Keep it simple — Junk is intentionally a single-purpose tool.

---

## License

MIT © [Paul Fleury](https://paulfleury.com)
