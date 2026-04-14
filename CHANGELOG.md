# Changelog

All notable changes to Junk are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [1.4.0] — 2026-04-14

### Added
- **Esc to close** — dismiss the window instantly without touching the keyboard shortcut
- **Space Grotesk font** — loaded via Google Fonts, replaces system monospace

### Changed
- **Frosted glass UI** — `backdrop-filter: blur(40px) saturate(180%)` over a transparent window; white glass with layered box-shadow bevel
- **Native window** — `titleBarStyle: hiddenInset` + traffic lights pushed off-screen (`-100,-100`); proper macOS shadow and rounded corners, no fake border-radius hack
- **Transparent Electron window** — `transparent: true` + `vibrancy: under-window` so the blur actually sees through to the desktop
- **Larger line-height** — 1.80, 22px Space Grotesk, `letter-spacing: -0.015em`
- **Cleaner footer** — glass-tinted, thinner divider

---

## [1.3.0] — 2026-04-14

### Changed
- **No tray icon** — removed entirely. Junk is now a pure invisible background process. No menu bar presence whatsoever
- **Window no longer hides on blur** — switching to another app, copying text, running a clipboard action and coming back no longer dismisses the window. Only `⌘J` toggles it
- **No title bar, no fake traffic lights** — completely frameless floating modal. No close button, no drag handle header. The entire window border is the drag region; the textarea is not
- **Cleaner UI** — removed title bar and `×` button entirely. Just the textarea + a faint footer hint
- **Smaller, tighter footer** — single line: `⌘J toggle · content auto-saved`
- **Rounder corners** — 18px radius for a softer floating feel

---

## [1.2.0] — 2026-04-14

### Fixed
- Complete signing overhaul — split CI into two separate jobs (arm64 on `macos-14`, x64 on `macos-13`)
- Sign every dylib, `.so`, framework and binary inside the `.app` individually (leaves → root) before DMG packaging
- Ad-hoc sign the `.app` root with `--options runtime --timestamp=none` — this is the exact combination macOS Gatekeeper requires to show **Open Anyway** in Privacy & Security instead of rejecting the app as damaged
- x64 build now runs on a real Intel runner — eliminates cross-arch signing issues

---

## [1.1.0] — 2026-04-14

### Fixed
- Ad-hoc code signing of `.app` bundle before DMG packaging — prevents macOS "app is damaged" Gatekeeper error
- `identity: null` in electron-builder config to fully suppress Apple certificate discovery

---

## [1.0.0] — 2026-04-14

### Added
- Global `⌘J` hotkey — works from any app, any Space
- Frameless, transparent floating window with macOS vibrancy
- Fly-in animation on open (`cubic-bezier` spring, 160ms)
- Large monospaced scratchpad textarea (22px SF Mono)
- Auto-hide when window loses focus
- `Esc` key to close
- `×` button in title bar to close
- Auto-save via `localStorage` — content survives app restarts
- Draggable title bar to reposition window
- Menu bar tray icon with toggle and quit options
- Paste-anywhere handler (even when textarea isn't focused)
- `⌘A` select-all support
- Dark glass UI — near-black warm background, teal accent
- Thin custom scrollbar
- GitHub Actions CI — builds `arm64` + `x64` `.dmg` on tag push
- MIT license

[1.4.0]: https://github.com/paulfxyz/junk/releases/tag/v1.4.0
[1.3.0]: https://github.com/paulfxyz/junk/releases/tag/v1.3.0
[1.2.0]: https://github.com/paulfxyz/junk/releases/tag/v1.2.0
[1.1.0]: https://github.com/paulfxyz/junk/releases/tag/v1.1.0
[1.0.0]: https://github.com/paulfxyz/junk/releases/tag/v1.0.0
