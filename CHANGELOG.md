# Changelog

All notable changes to Junk are documented in this file.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

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

[1.0.0]: https://github.com/paulfxyz/junk/releases/tag/v1.0.0
