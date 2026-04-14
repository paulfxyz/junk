# Changelog

All notable changes to Junk are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

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

[1.2.0]: https://github.com/paulfxyz/junk/releases/tag/v1.2.0
[1.1.0]: https://github.com/paulfxyz/junk/releases/tag/v1.1.0
[1.0.0]: https://github.com/paulfxyz/junk/releases/tag/v1.0.0
