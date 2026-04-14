```
     ██╗██╗   ██╗███╗   ██╗██╗  ██╗
     ██║██║   ██║████╗  ██║██║ ██╔╝
     ██║██║   ██║██╔██╗ ██║█████╔╝
██   ██║██║   ██║██║╚██╗██║██╔═██╗
╚█████╔╝╚██████╔╝██║ ╚████║██║  ██╗
 ╚════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝

  the flying scratchpad — built with Rust + Tauri v2
```

[![Version](https://img.shields.io/badge/version-2.2.1-5b5bf6?style=flat-square)](https://github.com/paulfxyz/junk/releases)
[![macOS](https://img.shields.io/badge/macOS-universal-black?style=flat-square&logo=apple)](https://github.com/paulfxyz/junk/releases)
[![Windows](https://img.shields.io/badge/Windows-x64-0078d4?style=flat-square&logo=windows)](https://github.com/paulfxyz/junk/releases)
[![Linux](https://img.shields.io/badge/Linux-AppImage%20%7C%20deb-fcc624?style=flat-square&logo=linux&logoColor=black)](https://github.com/paulfxyz/junk/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-22c55e?style=flat-square)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-ffc131?style=flat-square&logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Website](https://img.shields.io/badge/website-gojunk.app-5b5bf6?style=flat-square)](https://gojunk.app)

---

## What is Junk?

**Junk** is a global-hotkey scratchpad. Press **⌘J** (macOS) or **Ctrl+J** (Windows/Linux) — from anywhere, any app, any Space — and a clean floating notepad appears instantly. Press it again, or hit **Esc**, and it vanishes without a trace.

Your words stay. The window stays. It just gets out of your way.

No accounts. No sync. No cloud. No dock icon. No menu bar clutter. No distractions. Just a place for the thing you're thinking about right now — before you know how to say it.

> **It's the app you open before you know what you want to say.**

---

## Why does this exist?

Every developer, designer, writer, and thinker has a variation of the same workflow: you're deep in something, a thought arrives, and you need somewhere to put it. Not a note-taking app that wants you to organise it. Not a task manager that wants you to assign it. Not a full editor that loads for two seconds.

Just a place. Immediately. Then gone.

Junk is that place.

---

## Features

| Feature | Details |
|---|---|
| **⌘J / Ctrl+J** | Global hotkey — works in any app, any macOS Space, any virtual desktop |
| **Esc** | Hides the window — OS-level shortcut (v2.2.0), works even before JS loads |
| **Auto-save** | Content persists to `localStorage` with a 300 ms debounce — zero data loss |
| **Frosted glass UI** | `backdrop-filter: blur(40px) saturate(180%)` — beautiful on any wallpaper |
| **Always on top** | Floats above all other windows so it's always reachable |
| **Frameless** | No title bar, no traffic lights — drag anywhere on the window to reposition |
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

1. Download **`Junk_2.2.1_universal.dmg`** from [Releases](https://github.com/paulfxyz/junk/releases)
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

1. Download **`Junk_2.2.1_x64-setup.exe`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Run the installer. Windows SmartScreen will show a blue warning — click **More info** → **Run anyway**

   **Why SmartScreen?** The binary is not code-signed with a Windows Extended Validation (EV) certificate ($200–500/yr). SmartScreen flags all unsigned binaries. The source code is fully public — if you prefer, build it yourself (instructions below).

3. Junk launches on login and disappears into the background.
4. Press **Ctrl+J** from any application.

Alternatively, download the **MSI** (`Junk_2.2.1_x64_en-US.msi`) for enterprise/silent deployment:

```
msiexec /i Junk_2.2.1_x64_en-US.msi /quiet
```

---

### Linux — AppImage

```sh
# Download
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.2.1_amd64.AppImage

# Make executable
chmod +x Junk_2.2.1_amd64.AppImage

# Run
./Junk_2.2.1_amd64.AppImage
```

AppImages are portable — they run on any modern x86_64 Linux distribution without installation. No sudo required.

> **Wayland note:** Junk uses the X11 global shortcut mechanism via `libxdo`. On a pure Wayland session (no XWayland), the `Ctrl+J` global shortcut may not register — Wayland's security model intentionally prevents global hotkeys. Run with `XDG_SESSION_TYPE=x11` or enable XWayland in your compositor to work around this. This is a Tauri v2 framework limitation.

---

### Linux — .deb (Debian / Ubuntu)

```sh
# Download and install
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_2.2.1_amd64.deb
sudo dpkg -i Junk_2.2.1_amd64.deb

# Run
junk
```

---

## Usage

| Action | Result |
|---|---|
| **⌘J** (macOS) | Toggle the window: show if hidden, hide if visible |
| **Ctrl+J** (Windows / Linux) | Toggle the window |
| **Esc** | Hide the window |
| Click and drag anywhere | Move the window — the entire surface is a drag handle |
| **⌘A** / **Ctrl+A** | Select all text |
| **⌘V** / **Ctrl+V** | Paste — works even without clicking the textarea first |
| **⌘Z** / **Ctrl+Z** | Undo — full undo history survives hide/show cycles |
| Just type | Content saves automatically, 300 ms after you stop typing |

There is no close button. No title bar. The only ways to dismiss the window are **Esc** and **⌘J / Ctrl+J**. This is intentional — it prevents accidentally losing the window entirely.

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

> **Important:** Use `libwebkit2gtk-4.1-dev`, not `4.0`. Tauri v2 requires WebKitGTK 4.1.

---

### Development build

```sh
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# Hot-reload dev server — opens a window immediately
npm run dev
```

In dev mode, right-click anywhere in the window to open the WebView inspector.

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

The Rust/Tauri rewrite ships as a single ~4 MB binary with no external runtime. The startup time is imperceptible.

---

### Global Shortcuts

Two shortcuts are registered at the OS level using [`tauri-plugin-global-shortcut`](https://v2.tauri.app/plugin/global-shortcut/):

| Shortcut | Platform | Behaviour |
|---|---|---|
| **⌘J** | macOS | Toggle window (show/hide) |
| **Ctrl+J** | Windows / Linux | Toggle window (show/hide) |
| **Escape** | All | Hide window (v2.2.0 fix) |

Both shortcuts use platform-native APIs:

| Platform | API |
|---|---|
| macOS | Carbon `RegisterEventHotKey` |
| Windows | `RegisterHotKey` (Win32) |
| Linux | `XGrabKey` via `libxdo` |

The modifier is selected at compile time:

```rust
// macOS: ⌘ (Command / Super)
#[cfg(target_os = "macos")]
let modifiers = Modifiers::SUPER;

// Windows + Linux: Ctrl
#[cfg(not(target_os = "macos"))]
let modifiers = Modifiers::CONTROL;

let shortcut = Shortcut::new(Some(modifiers), Code::KeyJ);
```

When the shortcut fires, the Rust callback reads `window.is_visible()` and calls either `window.hide()` or `window.show()` + `window.set_focus()`. No state machine, no flags — just two OS calls.

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
    │                      ▼
    │               User types (content auto-saves)
    │                      │
    │                      ▼
    ├─ ⌘J / Esc → window.hide()
    │
    └─ (window stays alive in background — WebView state preserved)
```

**Why never destroy?**

1. **Speed.** Showing an existing hidden window: ~5 ms. Creating a new window + parsing HTML: ~200–400 ms.
2. **State.** Cursor position, scroll position, selection, and full undo history survive every hide/show cycle.
3. **Storage.** Because it's the same WebView instance, `localStorage` is always intact — no re-read needed.

---

### Tauri IPC

The frontend invokes exactly one Rust command:

```
JS frontend
    │
    └─ await window.__TAURI__.core.invoke('hide_window')
                                            │
                                            ▼
                                  fn hide_window(app: AppHandle)
                                            │
                                            └─ app.get_webview_window("main")
                                                         │
                                                         └─ .hide()
```

`AppHandle` is dependency-injected by Tauri's `invoke_handler` — no arguments flow from JS. The command is registered in `tauri::generate_handler![hide_window]` and permitted in `capabilities/default.json`.

Two JS code paths call `hide_window` as a belt-and-suspenders fallback:
1. The **Esc** key listener (JS fallback — the primary Esc handler is the OS-level shortcut registered in Rust since v2.2.0)
2. The **in-window ⌘J / Ctrl+J** listener (handles the edge case where the shortcut fires while the window already has focus)

---

### macOS Activation Policy

macOS has three application activation policies:

| Policy | Dock Icon | App Switcher | Used by |
|---|---|---|---|
| `Regular` | Yes | Always | Standard GUI apps |
| `Accessory` | No | Only when a window is visible | Paste, Magnet, Junk |
| `Prohibited` | No | Never | Login agents, daemons |

Junk uses `Accessory`. This hides the Dock icon permanently while still allowing the app to receive keyboard focus and display windows. The policy is set via `app.set_activation_policy(ActivationPolicy::Accessory)` in the Tauri `setup` hook, before any window is shown.

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

CSS animations only run once per element lifecycle. To replay on each show:

```js
// 1. Remove the animation name
el.style.animationName = 'none';
// 2. Force style recalculation (browser must flush before restoring)
void el.offsetWidth;
// 3. Restore — browser sees a new animation and plays it
el.style.animationName = '';
```

The `tauri://focus` event (emitted by the Tauri WebView when the window gains focus) is the hook for triggering the replay.

---

### Frosted Glass

The visual style uses only CSS — no images, no SVG filters, no canvas:

```css
background: rgba(255, 255, 255, 0.72);
backdrop-filter: blur(40px) saturate(180%);
-webkit-backdrop-filter: blur(40px) saturate(180%);
border: 1px solid rgba(255, 255, 255, 0.55);
border-radius: 18px;
box-shadow:
  0 32px 64px rgba(0, 0, 0, 0.18),
  0 8px 24px rgba(0, 0, 0, 0.10),
  inset 0 1px 0 rgba(255, 255, 255, 0.85);
```

`backdrop-filter` composites the blurred background from the pixels behind the window — it requires a transparent window (`transparent: true` in `tauri.conf.json`) so the OS compositor can supply those pixels. On older Linux setups without compositor support, the blur silently degrades to a semi-transparent white.

---

## Project Structure

```
junk/
├── src/
│   └── index.html              ← Single-file frontend: HTML + CSS + JS, zero build step
│
├── src-tauri/
│   ├── src/
│   │   └── main.rs             ← Rust backend: shortcuts, window management, IPC
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

---

## CI / Release Pipeline

Every push to a `v*` tag triggers the GitHub Actions workflow:

```
Tag pushed (e.g. v2.2.0)
        │
        ├─ [macOS runner]    cargo tauri build --target universal-apple-darwin
        │                    ad-hoc codesign (xattr-removable)
        │                    → Junk_2.2.1_universal.dmg
        │
        ├─ [Windows runner]  cargo tauri build
        │                    → Junk_2.2.1_x64-setup.exe
        │                    → Junk_2.2.1_x64_en-US.msi
        │
        └─ [Ubuntu runner]   cargo tauri build
                             → Junk_2.2.1_amd64.AppImage
                             → Junk_2.2.1_amd64.deb
                                      │
                                      ▼
                             All artifacts uploaded to GitHub Release
                             Release auto-published with changelog
```

The release job uses `find dist/ -type f` (not glob patterns) to enumerate artifacts — this was a hard-won fix; GitHub Actions glob expansion is inconsistent across runner environments.

---

## Changelog

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
├── serde + serde_json 1.x             ← IPC serialisation
├── log 0.4                            ← Structured logging facade
└── env_logger 0.11                    ← RUST_LOG-driven log subscriber (debug builds)
```

**Frontend:** zero dependencies. `index.html` is a single self-contained file — HTML, CSS, and JavaScript all inline. No build step, no bundler, no npm modules at runtime.

---

## FAQ

**Q: Why does macOS say the app is "damaged"?**  
A: It's not damaged — it's quarantined. macOS Gatekeeper flags every app downloaded outside the App Store that isn't notarised with a paid Apple Developer account. Run `xattr -rd com.apple.quarantine /Applications/Junk.app && open /Applications/Junk.app` to remove the flag. This is safe.

**Q: Why isn't Junk in the Mac App Store?**  
A: The App Store's sandbox restrictions prevent apps from registering global hotkeys that work across all applications. Junk's core feature is incompatible with the sandbox.

**Q: Will Junk phone home, collect analytics, or check for updates?**  
A: No. Junk has no network code. There is no update daemon, no telemetry, no crash reporting. The only data it writes is your text, locally.

**Q: ⌘J conflicts with another app on my Mac. Can I change it?**  
A: Not yet via UI — it's hardcoded. If you need to change it, edit `main.rs` (change `Code::KeyJ` to any other `Code::*` variant), rebuild, and replace your app bundle. A preference pane for the shortcut is on the roadmap.

**Q: Where is my data if I uninstall?**  
A: Your content lives in the Tauri WebView profile. On macOS: `~/Library/WebKit/com.paulfleury.junk/`. Delete that folder to wipe everything.

**Q: Does Junk work on macOS without Rosetta on Apple Silicon?**  
A: Yes. The `.dmg` ships a universal binary — native ARM64 code runs on Apple Silicon, native x86_64 runs on Intel. No Rosetta translation layer needed.

**Q: Why no iOS / Android?**  
A: The global-hotkey model doesn't translate to mobile — there's no concept of a system-wide keyboard shortcut on iOS or Android. Junk is fundamentally a desktop-first tool.

---

## Contributing

Pull requests are welcome. A few things to know:

1. **All comments must explain the *why***, not the *what*. The code already says what it does. Comments exist to explain intent, trade-offs, and non-obvious decisions.
2. **No unwrap() in non-test code.** Use `map_err`, `ok_or`, `if let`, or `?` instead.
3. **No new npm dependencies.** The frontend is intentionally dependency-free. If you need a library, inline the relevant parts.
4. **Test on all three platforms before opening a PR.** The CI matrix will catch platform regressions, but it's faster if you do too.

---

## Built with Perplexity Computer

This app — from the Rust rewrite to the landing page to this README — was built using [Perplexity Computer](https://www.perplexity.ai/computer), an AI agent that writes production-quality code, reasons about architecture, and explains every decision inline.

Every comment in `main.rs` is part of the output. The architecture diagrams, the trade-off tables, the FAQ — all generated and iterated through conversation.

---

## Author

**Paul Fleury** — [paulfleury.com](https://paulfleury.com) · [GitHub @paulfxyz](https://github.com/paulfxyz)

---

## License

[MIT](LICENSE) — do whatever you want with it.
