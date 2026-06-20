```
     ██╗██╗   ██╗███╗   ██╗██╗  ██╗
     ██║██║   ██║████╗  ██║██║ ██╔╝
     ██║██║   ██║██╔██╗ ██║█████╔╝
██   ██║██║   ██║██║╚██╗██║██╔═██╗
╚█████╔╝╚██████╔╝██║ ╚████║██║  ██╗
 ╚════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝

  the flying scratchpad — built with Rust + Tauri v2
```

[![Version](https://img.shields.io/badge/version-3.1.1-5b5bf6?style=flat-square)](https://github.com/paulfxyz/junk/releases)
[![macOS](https://img.shields.io/badge/macOS-universal-black?style=flat-square&logo=apple)](https://github.com/paulfxyz/junk/releases)
[![Windows](https://img.shields.io/badge/Windows-x64-0078d4?style=flat-square&logo=windows)](https://github.com/paulfxyz/junk/releases)
[![Linux](https://img.shields.io/badge/Linux-AppImage%20%7C%20deb-fcc624?style=flat-square&logo=linux&logoColor=black)](https://github.com/paulfxyz/junk/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-22c55e?style=flat-square)](LICENSE)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%20v2-ffc131?style=flat-square&logo=tauri)](https://v2.tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Website](https://img.shields.io/badge/website-thejunk.app-5b5bf6?style=flat-square)](https://thejunk.app)

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
| **Custom shortcut** | Change ⌘J to any key combo in Preferences — live re-registration, no restart |
| **Position memory** | Remembers where you dragged it between sessions via `PhysicalPosition` IPC |
| **Size memory** | Remembers window size between sessions — restored on every focus |
| **Font size** | 14–28 px slider in Preferences — drives CSS custom property, live preview |
| **Dark mode** | Light / Auto / Dark — Auto follows `prefers-color-scheme` with live listener |
| **Markdown preview** | ⌘M toggle — inline vanilla-JS parser, no library, zero bundle impact |
| **Export** | Footer copy button — copies all content to clipboard with "Copied!" feedback |
| **Rounded corners** | True OS-level rounded corners via `objc2` CALayer `masksToBounds` (macOS) |
| **Native window shadow** | macOS WindowServer drop shadow via `"shadow": true` — adapts to dark mode |

---

## Install

### macOS (Universal — Apple Silicon + Intel)

1. Download **`Junk_3.1.1_universal.dmg`** from [Releases](https://github.com/paulfxyz/junk/releases)
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

1. Download **`Junk_3.1.1_x64-setup.exe`** from [Releases](https://github.com/paulfxyz/junk/releases)
2. Run the installer. Windows SmartScreen will show a blue warning — click **More info** → **Run anyway**

   > **Why SmartScreen?** The binary is not code-signed with a Windows EV certificate ($200–500/yr). The source is fully public — build it yourself if you prefer (instructions below).

3. Junk launches on login and runs silently in the background.
4. Press **Ctrl+J** from any application.

**MSI (enterprise / silent deployment):**

```
msiexec /i Junk_3.1.1_x64_en-US.msi /quiet
```

---

### Linux — AppImage

```sh
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_3.1.1_amd64.AppImage
chmod +x Junk_3.1.1_amd64.AppImage
./Junk_3.1.1_amd64.AppImage
```

Portable — runs on any modern x86_64 Linux without installation. No sudo required.

> **Wayland note:** Global shortcuts use X11/libxdo. On a pure Wayland session, `Ctrl+J` may not register — run with `XDG_SESSION_TYPE=x11` or enable XWayland in your compositor.

---

### Linux — .deb (Debian / Ubuntu)

```sh
wget https://github.com/paulfxyz/junk/releases/latest/download/Junk_3.1.1_amd64.deb
sudo dpkg -i Junk_3.1.1_amd64.deb
junk
```

---

## Usage

| Action | Result |
|---|---|
| **⌘J** (macOS) / **Ctrl+J** (Win/Linux) | Toggle the window |
| **Esc** | Hide the window (or close Preferences if open) |
| **⌘,** / **Ctrl+,** | Open Preferences panel |
| **⌘M** / **Ctrl+M** | Toggle Markdown preview |
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

The toggle reads the actual OS state on every panel open — stays accurate if you toggled it from System Settings separately. We never cache the launch-at-login state in JavaScript; every panel open calls `get_prefs` which calls `autolaunch().is_enabled()` fresh from the OS.

The underlying mechanism on macOS is a per-user `launchd` LaunchAgent plist — no admin privileges, no system-level daemons. `launchd` reads it at login and starts Junk before the user's first keypress. On Windows, the `Run` registry key is under `HKCU` (current user) — no elevation required. On Linux the XDG autostart spec places a `.desktop` file in `~/.config/autostart/` which most login managers and desktop environments pick up automatically.

### Auto-check for updates

When enabled (default: on), Junk silently checks the GitHub Releases API ~2 seconds after first launch. If a newer version is found, the ⚙ icon turns purple. No nagging, no banners — just a quiet indicator.

The "Check now" button triggers an immediate check and shows the result inline:

- **"You're up to date (3.0.4)"** — green
- **"Update available: v3.x.x"** — purple, clickable link to releases page
- **"Could not check — are you online?"** — red if the request fails

The check is performed entirely in Rust via `tauri-plugin-http` — it never touches `window.fetch()`, so it is not blocked by the WebView's Content Security Policy. The current version is read from `CARGO_PKG_VERSION` at compile time — there are no hardcoded version strings in JS, and the frontend version label can never drift from the binary version.

### Custom shortcut

Click the shortcut field in Preferences and press any key. Junk immediately unregisters the old OS hotkey and registers the new one at runtime — no restart. The new key is stored in `localStorage` and re-applied on next launch.

Keys not available: bare modifier keys (Shift, Ctrl, Meta) and Escape (already reserved). Any letter, digit, F-key, Space, or punctuation key works.

The key is captured as `KeyboardEvent.code` (e.g. `"KeyJ"`, `"F2"`, `"Space"`) rather than `KeyboardEvent.key` (e.g. `"j"`, `"F2"`, `" "`). This is deliberate: `.code` is the physical key position on the keyboard, layout-independent — it works correctly on AZERTY, Dvorak, Colemak, and every other keyboard layout. `.key` would produce incorrect characters on non-QWERTY layouts.

### Always on top

Default: **enabled**. Keeps Junk's window floating above all other apps — the core scratchpad experience. Can be disabled from Preferences if you prefer the window to sit in normal z-order.

This is implemented as a runtime IPC call (`set_always_on_top`) rather than a static config setting because `alwaysOnTop` in `tauri.conf.json` only sets the *initial* window level. Tauri does not watch the config for runtime changes — you must call `window.set_always_on_top()` via IPC to toggle it after the app is running.

Preference is saved to `localStorage['junk-always-top']` (`'true'` or `'false'`). Default is `true` — the toggle is only `'false'` if the user has explicitly disabled it.

### Font size

A slider from 14 px to 28 px drives a CSS custom property `--font-size` on `:root`. Both the editor textarea and the Markdown preview pick it up automatically — no JS duplication. Saved to `localStorage`.

The implementation is a single `input[type="range"]` whose `input` event calls:
```js
document.documentElement.style.setProperty('--font-size', `${val}px`);
localStorage.setItem('junk-font-size', val);
```
No JS selects individual elements — the CSS `var(--font-size)` cascades to both the `<textarea>` and the `#md-preview` div automatically. On load, the saved value (default: `18`) is applied before the first paint to avoid a flash of unstyled text.

### Appearance (dark mode)

Three options: **Light**, **Auto**, **Dark**.
- **Auto** (default): reads `prefers-color-scheme` and listens for live OS changes.
- **Light** / **Dark**: hard overrides that ignore system preference.

Implemented via CSS custom properties on `:root[data-theme="dark"]`. Every colour, shadow, and blur value has a dark-mode override — the frosted glass deepens to `rgba(30,30,35,0.88)`.

Auto mode attaches a `MediaQueryList.addEventListener('change', ...)` listener that fires synchronously when the OS switches colour schemes — no restart, no page reload. When switching from Auto to Light or Dark, the listener is explicitly removed to avoid memory leaks and double-firing. The listener reference is stored in a module-scoped variable `autoThemeListener` and torn down before being reassigned.

### Markdown preview

Toggle with **⌘M** / **Ctrl+M**, the footer button, or the toggle in Preferences. When on, the textarea hides and a rendered `#md-preview` div appears. Raw text is always stored in `localStorage` — the preview is render-only.

The inline parser handles: h1/h2/h3, **bold**, _italic_, `code`, fenced code blocks, blockquotes, HR, unordered/ordered lists, `[links](url)`. No external library — ~80 lines of vanilla JS.

Switching back from preview to edit mode re-populates the textarea from `localStorage` — the content is never lost and the cursor is repositioned to the end of the text. All keyboard shortcuts (`⌘A`, `⌘Z`, `⌘V`) work normally in edit mode.

### Export

Copies all content to the clipboard via `navigator.clipboard.writeText()`. Available from the footer copy button and the Preferences "Copy all text" button.

The copy button shows a checkmark SVG icon and the label "Copied!" for 1.5 seconds, then reverts to the original clipboard icon. This feedback was added in v3.0.1 — prior versions used a static clipboard SVG with no state change, making it unclear whether the copy had succeeded. The feedback is implemented as a simple `setTimeout` that swaps two inline SVG paths and reverts.

Export always copies raw text — even when Markdown preview is active, the raw Markdown source is exported, not the rendered HTML.

### Credits

The Credits section explains how to truly quit Junk and links to [thejunk.app](https://thejunk.app) and the GitHub repo.

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

**Code audit:** `src-tauri/src/main.rs` is ~700 lines. The entire frontend is one HTML file (~1,500 lines). MIT-licensed, fully public — [read the source](https://github.com/paulfxyz/junk).

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
    "core:window:allow-outer-position",
    "core:window:allow-set-position",
    "core:window:allow-inner-size",
    "core:window:allow-set-size",
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

The Tauri capability system is additive — every permission listed here is explicitly required. The `core:default` bundle grants only basic IPC bootstrap functionality. Every window, webview, and plugin permission above it is individually declared. This means adding a new feature requires a conscious permission addition — there is no ambient over-permissioning.

**Why `core:window:allow-inner-size` and `core:window:allow-set-size`?** Added in v3.0.1 for window size memory. Reads the current window dimensions before saving position, and restores them on next focus. Both permissions were absent in v3.0.0 and caused silent IPC failures when size memory was first implemented.

</details>

---

## Design Philosophy

Junk is not a notes app. It solves a narrower, more specific problem: **the 30-second window when a thought arrives and you need somewhere to put it right now.**

Every design decision traces back to this constraint:

**One keypress, immediate.** Every additional action (launching the app, navigating to a note, choosing a location) increases the chance the thought evaporates. The global shortcut is not a convenience — it is the product.

**Never ask for organisation.** The moment you ask a user where to put something, you've made them think about their system instead of their idea. Junk has one scratchpad. No inbox, no archive, no folder, no tag. The act of deciding what to keep and where to put it is a separate task for a separate tool.

**Stay out of the way when not needed.** No Dock icon. No menu bar entry. No notification badge. No background network traffic (by default). Zero pixels and zero attention when you're not using it.

**Stay visible when needed.** When Junk is on screen, it floats above everything. It doesn't hide when you click away. You can look at Safari, look at Junk, copy from Safari, paste into Junk — without the window ever disappearing.

**Plain text, always.** No rich text, no formatting toolbar. Formatting is a cognitive tax on the writing process. Junk is for the thought, not its presentation. Markdown preview exists for those who want to render their notes — but raw text is always the source of truth.

**Native quality.** Frosted glass, rounded corners clipped at the OS compositor level, native window shadows, native drag behaviour — these aren't decoration. They signal to the user "this belongs here, on this OS". A tool you use dozens of times a day should feel native.

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

**Why not Electron still?** Three reasons beyond size and performance:

1. **No runtime requirement.** Electron bundles Node.js. Every user needs a compatible Node environment embedded in the binary. Tauri's Rust binary has zero external dependencies — it runs on any supported OS with no prerequisites.

2. **System WebView.** On macOS, WKWebView is the same engine as Safari — a first-class citizen of the OS. It gets security patches automatically via macOS updates. An Electron app bundles a specific Chromium version and requires manual updates to get security fixes.

3. **Capability sandboxing.** Tauri v2's capability system requires explicit permission declarations for every OS API surface the app uses. There is no ambient "full OS access" like Node.js's `require('fs')`. The attack surface is minimal and auditable.

**Why not a pure native Swift/ObjC app?** Cross-platform. A Swift app would need a complete Windows and Linux rewrite. A Rust + Tauri app shares the backend logic across all three platforms. The platform-specific code in `main.rs` is less than 20 lines of `#[cfg(target_os)]` — the rest is identical.

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
<summary><strong>macOS rounded corners: the full investigation (v3.0.2 → v3.0.3)</strong></summary>

Getting true OS-level rounded corners on a transparent frameless Tauri window on macOS was a three-attempt investigation that exposed the compositor layer hierarchy of WKWebView.

### Attempt 1: CSS `border-radius` on `html, body` — FAILED

The most obvious approach: apply `border-radius: 14px` to the root HTML element.

```css
html, body {
  border-radius: 14px;
  overflow: hidden;
}
```

**What happened:** The CSS rounded the painted content in the WebView, but the WKWebView itself still occupied a full rectangular frame. On a transparent window, you see the rounded CSS content — but the WKWebView renders in a separate compositor layer managed by the OS. The corner pixels of the WebView were transparent (correctly, because the CSS `background` was clipped), but the `backdrop-filter: blur()` still operated on the full rectangle, producing an unclipped blur with square corners.

**Root cause:** CSS cannot clip the WKWebView at the OS compositor level. WebKit renders the WKWebView in a separate CALayer (Core Animation layer) subtree that sits above the OS compositor. CSS `border-radius` clips the rendered CSS content but does not instruct the CALayer to mask its compositing bounds. The blur samples pixels from the layers below — the blur layer itself is square even if its CSS content appears rounded.

**Result:** The frosted glass had square corners. The visual result was worse than no rounding at all, because the sharp square blur looked intentional.

### Attempt 2: `window-vibrancy` crate with `NSVisualEffectView.cornerRadius` — PARTIALLY WORKED

The `window-vibrancy` crate provides a `apply_vibrancy()` function for macOS frosted glass. Reading the crate's source on crates.io reveals the implementation: it creates an `NSVisualEffectView` as a subview of the window's `contentView` and calls `setCornerRadius()` on that subview.

```rust
// From window-vibrancy source — simplified
let visual_effect_view = NSVisualEffectView::new(mtm);
visual_effect_view.setMaterial(NSVisualEffectMaterial::HudWindow);
visual_effect_view.setCornerRadius(14.0);  // rounds the blur subview
content_view.addSubview(&visual_effect_view);
```

**What happened:** The frosted glass blur appeared. The blur subview's corners were rounded. But the WKWebView still rendered as a full square above the blur layer.

**Why partial:** `setCornerRadius()` on `NSVisualEffectView` rounds the blur *subview's* rendering — the subview's own CALayer clips its blur effect. But `NSVisualEffectView` is a subview added to the `contentView`; the WKWebView is a *sibling* (or child) of that `contentView`. The NSWindow frame itself is still rectangular. The WKWebView renders above the blur layer in its own full-size square layer, un-clipped.

**Visual result:** A frosted glass window with rounded blur corners *visible in the corners where the WKWebView was transparent* — but the WKWebView's background (even if `rgba(0,0,0,0)`) still occupied a rectangle that blocked the OS compositor from seeing through to the corners properly. The result was slightly rounded blur but square content.

### Root cause discovered: reading `window-vibrancy` source

The key insight came from reading the `window-vibrancy` source directly. `apply_vibrancy()` operates on an `NSVisualEffectView` subview. It never touches the `contentView`'s own CALayer. The `contentView` CALayer is the root layer — it clips everything, including WKWebView — but its `cornerRadius` and `masksToBounds` are left at their defaults (`0` and `NO`).

`masksToBounds = NO` on the root contentView CALayer means the compositing layer tree is not clipped to the window's frame shape. Every subview (including WKWebView) renders its full extent, unclipped.

The solution must operate on the `contentView`'s CALayer directly.

### Solution (v3.0.3): Two-step approach

**Step 1:** Call `apply_vibrancy()` for frosted-glass background material.

**Step 2:** Call a custom `set_macos_window_corner_radius()` function that walks up to the `contentView`'s CALayer and sets both `cornerRadius` and `masksToBounds`:

```rust
use objc2::msg_send;
use objc2::runtime::AnyObject;
use objc2_app_kit::{NSView, NSWindow};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

/// Set rounded corners on the macOS window at the OS compositor level.
///
/// This works by setting cornerRadius + masksToBounds = YES on the root
/// contentView CALayer. masksToBounds clips the ENTIRE compositor subtree —
/// including WKWebView — at the OS level. CSS border-radius cannot do this.
///
/// Must be called AFTER apply_vibrancy() — vibrancy sets up the NSVisualEffectView
/// subview hierarchy that we want to clip along with everything else.
fn set_macos_window_corner_radius(window: &WebviewWindow, radius: f64) -> Result<(), String> {
    let handle = window
        .window_handle()
        .map_err(|e| e.to_string())?;

    let ns_view = match handle.as_raw() {
        RawWindowHandle::AppKit(h) => h.ns_view.as_ptr() as *mut AnyObject,
        _ => return Err("not an AppKit window".into()),
    };

    unsafe {
        // ns_view → NSWindow → contentView → CALayer
        let ns_window: *mut AnyObject = msg_send![ns_view, window];
        let content_view: *mut AnyObject = msg_send![ns_window, contentView];
        let layer: *mut AnyObject = msg_send![content_view, layer];

        // cornerRadius: clips the layer's rendered output to a rounded rect
        let _: () = msg_send![layer, setCornerRadius: radius];

        // masksToBounds = YES: the key line.
        // Forces the CALayer to clip ALL child layers (including WKWebView)
        // to the layer's own bounds + cornerRadius shape.
        // Without this, cornerRadius only rounds the layer's own drawing —
        // child layers render unconstrained and appear above the rounded clip.
        let _: () = msg_send![layer, setMasksToBounds: true];
    }

    Ok(())
}
```

**Why `masksToBounds = YES` is the key:**

CALayer's `cornerRadius` property rounds the layer's *own* drawing (its background colour, border, etc.). But child layers are not clipped unless `masksToBounds` is `YES`. With `masksToBounds = YES`, the CALayer creates a mask in the shape of its bounds + cornerRadius and clips the *entire compositor subtree* — including all child layers (WKWebView and NSVisualEffectView) — to that shape.

This is an OS compositor operation: it happens below the WebKit rendering pipeline. No CSS, no JS, and no WebKit drawing code can see past it. The result is a true hardware-clipped rounded window.

### Dependencies

All three crates were already transitive dependencies of `window-vibrancy` — declared explicitly in `Cargo.toml` for direct use:

```toml
[dependencies]
window-vibrancy = "0.5"
objc2 = "0.6.0"
objc2-app-kit = { version = "0.3.0", features = ["NSView", "NSWindow"] }
raw-window-handle = "0.6"
```

No new transitive dependencies were added. The total binary size impact of the explicit declarations is zero — these crates were already compiled as part of `window-vibrancy`.

### Setup call sequence in `main.rs`

```rust
// In the Tauri setup hook, after the window is created:
#[cfg(target_os = "macos")]
{
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("apply_vibrancy failed");

    // Must come AFTER apply_vibrancy — rounds the entire compositor stack
    set_macos_window_corner_radius(&window, 14.0)
        .expect("set_macos_window_corner_radius failed");
}
```

The `#[cfg(target_os = "macos")]` guard is critical — `objc2` and `objc2-app-kit` are macOS-only. The function must not be compiled or called on Windows or Linux.

</details>

<details>
<summary><strong>macOS window shadow (v3.0.4)</strong></summary>

### The problem: `box-shadow` doesn't work on masked windows

After v3.0.3 added `masksToBounds = YES` to the contentView's CALayer, the existing CSS `box-shadow` stopped casting a shadow. The cause: `masksToBounds = YES` clips the *entire compositor subtree* to the layer's bounds — including any shadows that CSS tried to draw outside those bounds.

```css
/* This works on unmasked windows, but is clipped by masksToBounds = YES */
box-shadow:
  0 32px 80px rgba(0, 0, 0, 0.22),
  0 8px 24px rgba(0, 0, 0, 0.12);
```

The `box-shadow` is rendered as part of the WebKit compositor layer. With `masksToBounds = YES` on the parent contentView CALayer, any drawing that extends outside the layer's bounding rect — including drop shadows — is clipped. The result is a crisp, rounded window with no shadow.

### Attempt: CSS `box-shadow` via a wrapper element

One approach would be to create a separate DOM element positioned behind the window content but outside the masked layer. This is complex, fragile, and doesn't integrate with macOS's window stacking and depth system.

### Solution: `"shadow": true` in `tauri.conf.json` (v3.0.4)

macOS's WindowServer draws native drop shadows *outside* the window frame — below the compositor layer where `masksToBounds` operates. The WindowServer shadow is not a WebKit drawing; it's a system-level effect applied by the compositor to the entire window frame.

```json
{
  "app": {
    "windows": [
      {
        "shadow": true,
        ...
      }
    ]
  }
}
```

One line. Zero Rust code. The result:

- WindowServer draws a native drop shadow below and around the window frame
- The shadow adapts automatically to window depth (how high the window is above others)
- The shadow adapts to dark mode — darker in light mode, subtler in dark mode
- The shadow updates as the window moves, resizes, or changes z-order
- The shadow is drawn *below* the `masksToBounds` clip — it is never clipped

This is exactly the same shadow mechanism used by every macOS native app. It looks indistinguishable from a native window shadow because it *is* the native window shadow.

### What CSS `box-shadow` does now

With the native shadow providing depth, the CSS `box-shadow` is now reduced to a single crisp edge ring:

```css
box-shadow:
  inset 0 -1px 0 rgba(0, 0, 0, 0.06),       /* inner bottom rim — glass edge */
  inset 0 1px 0 rgba(255, 255, 255, 0.90),  /* specular top highlight */
  0 0 0 0.5px rgba(0, 0, 0, 0.15);          /* crisp edge ring — visible on white bg */
```

The `0.5px` edge ring is subpixel — on Retina displays it renders as a half-pixel border. This is only visible when Junk is positioned over a white or light background; on dark wallpapers the edge is invisible. Its purpose is to define the window boundary precisely when the backdrop blur blends into a similarly-coloured background.

The two inset shadows (bottom rim and top highlight) simulate physical glass catching light. They are not clipped by `masksToBounds` because they are *inset* — they draw inside the layer's bounds, not outside.

### Shadow in dark mode

The native WindowServer shadow automatically reduces its opacity in dark mode — macOS reduces shadow intensity system-wide in dark mode to preserve contrast. The CSS edge ring has a dark-mode override:

```css
:root[data-theme="dark"] .window {
  box-shadow:
    inset 0 -1px 0 rgba(0, 0, 0, 0.20),
    inset 0 1px 0 rgba(255, 255, 255, 0.08),
    0 0 0 0.5px rgba(255, 255, 255, 0.08);  /* light edge ring on dark bg */
}
```

</details>

<details>
<summary><strong>The deep-comment bug: JavaScriptCore vs V8 (v3.0.4 → v3.0.8)</strong></summary>

### The deep-comment bug

**What happened:**
In v3.0.4, an educational documentation pass rewrote `index.html` from 1567 to 2067 lines by adding extensive inline comments. The file passed all static checks:
- `node --check src/index.html` → no syntax errors
- Manual brace-depth analysis → correctly balanced
- AST walk → all variables declared before use (at the module level)

But when running in Tauri's WKWebView (JavaScriptCore), the app produced:

```
ReferenceError: Can't find variable: windowEl
  at registerDragListener — localhost:1847
  at Module Code — localhost:1890
```

`windowEl` is declared as `const windowEl = document.getElementById('window')` near the top of the module. `registerDragListener` is an IIFE at the module's closing brace that references it.

**Why it only failed in WebKit:**
V8 (Node.js, Chrome DevTools) processes large ES modules without issue. JavaScriptCore (WebKit, used in WKWebView) appears to have a different compilation path for modules above a certain size threshold. The exact mechanism is unconfirmed — it may be related to lazy compilation, a JIT tier change, or a variable-resolution difference in how large module scopes are compiled. Regardless, the symptom is consistent: the file works below ~1700 lines and fails above ~2000.

**Diagnosis process:**
1. User reported all buttons non-functional after installing v3.0.7
2. Console error: `ReferenceError: Can't find variable: windowEl`
3. Searched git log for when `windowEl` was last changed → commit `3b7bef4` (deep-comment pass)
4. `wc -l` comparison: `3b7bef4` = 2067 lines, working commit `4a18c56` = 1567 lines
5. `git checkout 4a18c56 -- src/index.html` → confirmed crash gone

**Fix:** `git checkout 4a18c56 -- src/index.html` (pre-deep-comment version, 2026-06-06, released as v3.0.8).

**Rule going forward:** Keep `index.html` under ~1700 lines. Add documentation to `.md` files, not to the HTML module itself.

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

**Why not intercept ⌘Q?** Junk v2.3.0 through v2.5.0 (when there was a system tray) intercepted `ExitRequested` and hid instead of quitting. After the tray was removed in v2.6.0, intercepting ⌘Q created a trap: the user typed ⌘Q, nothing happened, and there was no UI to indicate the app was still running. From v2.6.0 onwards, ⌘Q quits for real — the only way to keep the process alive is to not press ⌘Q. Activity Monitor is the escape hatch for users who want to truly kill it without quitting.

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

**Custom shortcut registration flow (v3.0.0+):**

When the user changes the shortcut in Preferences:

```
JS: keydown → e.code (e.g. "KeyK") → ipc('set_hotkey', { key: 'KeyK' })
Rust: parse_key_code("KeyK") → Code::KeyK
      gs.unregister(current_shortcut)  // explicit — no auto-deregistration on drop
      gs.on_shortcut(new_shortcut, toggle_fn)
      *state.lock() = new_shortcut
```

`Shortcut` objects are not automatically deregistered when dropped — the OS hotkey registration is a side effect that persists until explicitly cancelled. This means if you register a new shortcut without unregistering the old one, both fire simultaneously. The `CurrentShortcut(Mutex<Shortcut>)` managed state holds the currently-registered shortcut so the old registration can be found and cancelled before the new one is created.

</details>

<details>
<summary><strong>Single-file frontend</strong> (no build step, no npm runtime)</summary>

`src/index.html` is ~1,500 lines of HTML, CSS, and JavaScript — all inline, zero build step, zero npm runtime dependencies, zero external JS libraries.

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

**The website frontend is a separate codebase.** The app frontend (`src/index.html`) is the window that appears when you press ⌘J. The landing page at [thejunk.app](https://thejunk.app) is a separate HTML/CSS/JS file deployed via FTP to SiteGround. They share no code. The landing page uses cache-busted filenames (`style.v304.css`, `app.v304.js`) so that CDN-cached HTML doesn't serve stale assets. See the SiteGround CDN section below for the full explanation.

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

─── v3.0.0 additions ────────────────────────────────────────────────────────────
ipc('set_hotkey', { key: String })    →  parse_key_code(key) → Code
                                         global_shortcut.unregister(old)
                                         global_shortcut.register(new Code + modifier)
                                         *state.lock() = new_shortcut

ipc('get_window_position')            →  window.outer_position()
                                         returns WindowPosition { x: i32, y: i32 }

ipc('set_window_position', {x, y})    →  window.set_position(
                                           PhysicalPosition::new(x, y)
                                         )

─── v3.0.1 additions ────────────────────────────────────────────────────────────
ipc('get_window_size')                →  window.inner_size()
                                         returns WindowSize { width: u32, height: u32 }

ipc('set_window_size',                →  window.set_size(
     { width: u32, height: u32 })          PhysicalSize::new(width, height)
                                         )
─────────────────────────────────────────────────────────────────────────────

Rust → JS events (via window.emit / Tauri event system)
─────────────────────────────────────────────────────────────────────────────
"tauri://focus"   →  triggerFlyIn() + setTimeout(focusEditor, 20ms)
                     + restoreWindowPosition()  ← new in v3.0.0
                     + restoreWindowSize()      ← new in v3.0.1
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
<summary><strong>v3.0.0 feature internals: runtime hotkey, position memory, dark mode, markdown parser</strong></summary>

### Runtime hotkey re-registration

The challenge: `tauri-plugin-global-shortcut` registers a `Shortcut` value. To change the hotkey, you must unregister the old `Shortcut` first — otherwise both fire. But the `Shortcut` object is consumed by the registration call, so you can't hold a reference after registering.

Solution: `CurrentShortcut(Mutex<Shortcut>)` managed state. The `set_hotkey` command locks the mutex, clones the old shortcut for unregistration, registers the new one, and replaces the mutex value:

```rust
#[tauri::command]
fn set_hotkey(
    app: AppHandle,
    key: String,
    state: State<'_, CurrentShortcut>,
) -> Result<(), String> {
    let new_code = parse_key_code(&key)?;
    let modifier = platform_modifier();
    let new_shortcut = Shortcut::new(Some(modifier), new_code);

    let gs = app.global_shortcut();
    let mut current = state.0.lock().unwrap();

    // Unregister old — explicit call required, Shortcut does not impl Drop
    gs.unregister(current.clone()).map_err(|e| e.to_string())?;

    // Register new
    let toggle_fn = /* same toggle closure as setup */ ...;
    gs.on_shortcut(new_shortcut.clone(), toggle_fn)
        .map_err(|e| e.to_string())?;

    *current = new_shortcut;
    Ok(())
}
```

Key detail: `KeyboardEvent.code` (e.g. `"KeyJ"`, `"F2"`, `"Space"`) is used, not `.key` (e.g. `"j"`, `"F2"`, `" "`). `.code` is layout-independent — it represents physical key position, so it works identically on AZERTY, Dvorak, and QWERTY.

### Window position memory

Two IPC commands: `get_window_position` (reads `window.outer_position()` → `PhysicalPosition`) and `set_window_position` (calls `window.set_position(PhysicalPosition::new(x,y))`). Two new capability permissions required: `core:window:allow-outer-position` and `core:window:allow-set-position`.

JS save flow: on every `mouseup` after a drag, wait 80 ms (OS commits the final position asynchronously), call `get_window_position`, store `x`/`y` in `localStorage`. The 80 ms delay is critical — without it, `outer_position()` may return the pre-drag position on macOS.

JS restore flow: on `tauri://focus` event (every time the window appears), call `get_window_position` from localStorage and `set_window_position`. Not on startup — the `tauri://focus` path guarantees the WKWebView is visible and the OS compositor has committed the window frame before we try to move it.

Off-screen guard: `if (x < -100 || y < -100 || x > screen.width + 200 || y > screen.height + 200)` — silently discard. Protects against position memory becoming stale after a monitor is unplugged or a display resolution changes.

### Automatic dark mode with live listener

`window.matchMedia('(prefers-color-scheme: dark)')` provides both the current state and a live change event. The key subtlety is setting up the listener only when `Auto` mode is active, and tearing it down when the user switches to Light or Dark:

```js
function applyTheme(theme) {
    // Remove any existing auto-listener
    if (autoThemeListener) {
        darkMq.removeEventListener('change', autoThemeListener);
        autoThemeListener = null;
    }
    if (theme === 'auto') {
        autoThemeListener = (e) => setDarkClass(e.matches);
        darkMq.addEventListener('change', autoThemeListener);
        setDarkClass(darkMq.matches);
    } else {
        setDarkClass(theme === 'dark');
    }
}
```

CSS dark-mode variables are defined on `:root[data-theme="dark"]` — a single attribute flip applied to `:root` drives the entire visual tree via CSS custom properties. No JS property iteration, no class list management on individual elements.

### Vanilla Markdown parser

No external library. ~80 lines of vanilla JS process line-by-line:

```
Inline pass (per line):
  **bold**   → <strong>
  _italic_   → <em>
  `code`     → <code>
  [text](url)→ <a href>

Block pass (per line):
  ^# → <h1>, ^## → <h2>, ^### → <h3>
  ^> → <blockquote>
  ^--- → <hr>
  ^- / ^* → <ul><li>
  ^\d+\. → <ol><li>
  ^``` → fenced code block (buffered until closing ```)
  else → <p>
```

Lists accumulate in a buffer and flush when a non-list line is encountered. Fenced code blocks capture everything until the matching ` ``` ` close, then wrap in `<pre><code>`. The result is set as `innerHTML` on a preview `<div>` — the `<textarea>` is hidden, raw text continues to be stored in `localStorage` unchanged.

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
  inset 0 -1px 0 rgba(0, 0, 0, 0.06),       /* inner bottom rim — glass edge */
  inset 0 1px 0 rgba(255, 255, 255, 0.90),  /* specular top highlight */
  0 0 0 0.5px rgba(0, 0, 0, 0.15);          /* crisp edge ring */
```

`backdrop-filter` composites the blurred background from the pixels _behind_ the window — it requires `transparent: true` in `tauri.conf.json` so the OS compositor can supply those pixels to the WebView. The specular top inset shadow creates the perception of a physical glass surface catching light from above.

On older Linux setups without compositor support, `backdrop-filter` silently degrades to the `rgba(255,255,255,0.72)` background. The window remains fully functional and readable — just not glassy.

**Dark mode frosted glass:**

```css
:root[data-theme="dark"] .window {
  background: rgba(30, 30, 35, 0.88);
  backdrop-filter: blur(40px) saturate(160%);
  -webkit-backdrop-filter: blur(40px) saturate(160%);
  border: 1px solid rgba(255, 255, 255, 0.10);
  box-shadow:
    inset 0 -1px 0 rgba(0, 0, 0, 0.20),
    inset 0 1px 0 rgba(255, 255, 255, 0.08),
    0 0 0 0.5px rgba(255, 255, 255, 0.08);
}
```

The dark glass uses a higher opacity (`0.88` vs `0.72`) because dark backgrounds need more contrast to remain legible. The saturation is slightly reduced (`160%` vs `180%`) because over-saturated dark backgrounds look neon. The border changes from a white glow to a subtle white outline at low opacity — enough to define the edge without being glary.

**Note on `box-shadow` vs native shadow (v3.0.4):**

The original `box-shadow` stack included large ambient drop shadows (`0 32px 80px rgba(0,0,0,0.22)`). These were removed in v3.0.3 when `masksToBounds = YES` began clipping them. In v3.0.4, drop shadow is provided by the native WindowServer shadow (`"shadow": true` in `tauri.conf.json`) which operates below the clip. The CSS `box-shadow` is now only the two inset glass highlights and the `0.5px` edge ring.

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
<summary><strong>Window position + size memory (v3.0.1)</strong></summary>

### Two new IPC commands

```rust
#[tauri::command]
fn get_window_size(window: WebviewWindow) -> Result<WindowSize, String> {
    let size = window.inner_size().map_err(|e| e.to_string())?;
    Ok(WindowSize { width: size.width, height: size.height })
}

#[tauri::command]
fn set_window_size(
    window: WebviewWindow,
    width: u32,
    height: u32,
) -> Result<(), String> {
    window
        .set_size(tauri::PhysicalSize::new(width, height))
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WindowSize {
    width: u32,
    height: u32,
}
```

Two new capability permissions added to `capabilities/default.json`:
- `core:window:allow-inner-size`
- `core:window:allow-set-size`

### JS save flow

```js
// On mouseup after any drag — save both position AND size
windowEl.addEventListener('mouseup', async () => {
  // 80ms delay: OS commits final window position asynchronously.
  // Without this, outer_position() returns the pre-drag position on macOS.
  await new Promise(r => setTimeout(r, 80));

  const pos = await ipc('get_window_position');
  const size = await ipc('get_window_size');

  if (pos) {
    localStorage.setItem('junk-win-x', pos.x);
    localStorage.setItem('junk-win-y', pos.y);
  }
  if (size) {
    localStorage.setItem('junk-win-w', size.width);
    localStorage.setItem('junk-win-h', size.height);
  }
});

// Also save on window resize (resize handle drag)
window.addEventListener('resize', debounce(async () => {
  const size = await ipc('get_window_size');
  if (size) {
    localStorage.setItem('junk-win-w', size.width);
    localStorage.setItem('junk-win-h', size.height);
  }
}, 200));
```

### JS restore flow

```js
// On tauri://focus — restore position and size on every show
async function restoreWindowGeometry() {
  const x = parseInt(localStorage.getItem('junk-win-x'));
  const y = parseInt(localStorage.getItem('junk-win-y'));
  const w = parseInt(localStorage.getItem('junk-win-w'));
  const h = parseInt(localStorage.getItem('junk-win-h'));

  // Off-screen guard: discard positions that are clearly off-monitor
  // Handles: monitor unplugged, resolution change, multi-monitor rearrangement
  const isOffScreen = (
    isNaN(x) || isNaN(y) ||
    x < -100 || y < -100 ||
    x > screen.width + 200 ||
    y > screen.height + 200
  );

  if (!isOffScreen) {
    await ipc('set_window_position', { x, y });
  }

  // Size restore is separate — never guard on size (no "off-screen" equivalent)
  if (!isNaN(w) && !isNaN(h) && w > 100 && h > 60) {
    await ipc('set_window_size', { width: w, height: h });
  }
}
```

### Why `tauri://focus` and not startup?

Restoring position on startup fails on macOS because `outer_position()` and `set_position()` are not reliable before the window has been shown and the OS compositor has committed the window frame. The `tauri://focus` event fires after `window.show()` — the window is visible and the OS has committed its frame. Position and size changes at this point are atomic from the user's perspective (before the window appears on screen) because the fly-in animation runs simultaneously.

### The 80ms delay explained

macOS's Quartz Compositor commits window frame changes asynchronously. When a drag ends (`mouseup`), the OS WindowServer has not yet finalised the window's position in its own data structures — it's still processing the drag end event. Calling `outer_position()` immediately on `mouseup` often returns the window's position from a few frames before the drag ended.

The 80ms delay (measured empirically on multiple machines and macOS versions) reliably gives the OS enough time to commit the final position before we read it back. A longer delay (200ms) also works but produces a perceptible lag in position-saving that can cause the window to not save its final position if the user quickly hides it after dragging.

</details>

<details>
<summary><strong>The SiteGround CDN cache problem (thejunk.app deployment)</strong></summary>

The landing page at [thejunk.app](https://thejunk.app) is deployed to SiteGround shared hosting via FTP. SiteGround runs a CDN in front of the shared hosting — it caches responses to reduce server load. This CDN layer caused two classes of deployment problems.

### Problem 1: Uploading `index.html` doesn't update the site

SiteGround's CDN caches `https://thejunk.app/` (the root URL — the directory index) separately from `https://thejunk.app/index.html` (the explicit file URL). When you upload a new `index.html` via FTP:

- `https://thejunk.app/index.html` — immediately serves the new file (CDN has a cache miss, fetches from origin)
- `https://thejunk.app/` — **still serves the old cached response** until the CDN TTL expires (typically 1–24 hours)

Since virtually all visitors hit the root URL, a deployed update is invisible for hours.

### Workaround 1: `.htaccess` 302 redirect

```apache
# .htaccess — redirect root to explicit index.html
RewriteEngine On
RewriteRule ^$ /index.html [R=302,L]
```

This forces the CDN to issue a separate cache entry for `/index.html` (which busts on upload) rather than caching the root response. Visitors hitting `https://thejunk.app/` get a `302 → /index.html` — the redirect itself is cacheable but the destination file gets the fresh upload immediately.

A `302` (temporary redirect) rather than `301` (permanent redirect) is deliberate — browsers cache `301` responses indefinitely, meaning a user who visited once would follow the stale cached `301` forever. `302` instructs the browser to re-check each time.

### Problem 2: Stale JS/CSS even with fresh HTML

Even after solving the root URL cache problem, a second issue emerged: the browser had cached `style.css` and `app.js` from a previous visit. The HTML was fresh but referenced the same filenames — the browser served the old cached assets from its local cache.

### Workaround 2: Cache-busted asset filenames

Instead of `style.css` and `app.js`, version-specific names are used:

```html
<!-- Old — cached forever by browsers -->
<link rel="stylesheet" href="style.css">
<script src="app.js"></script>

<!-- New — new filename = new cache entry -->
<link rel="stylesheet" href="style.v304.css">
<script src="app.v304.js"></script>
```

Every release bumps the version suffix in the filename. Since browsers have never requested `style.v304.css` before, there is no cached entry — the asset is fetched fresh. The old `style.v303.css` remains on the server (no cleanup needed — it's 10 KB).

This technique works even if the old `index.html` is still being served from the CDN — the old HTML references `style.v303.css` (still on the server), and the new HTML references `style.v304.css` (also on the server). Both serve correctly. The CDN transitions are seamless.

### Why not use SiteGround's cache purge tool?

SiteGround's cPanel provides a "Purge Cache" button for the CDN. We tried this — it clears the CDN cache for cached URLs, but in practice the CDN would re-cache the root URL from the origin within seconds of the next request (before we could verify the update was live). The race condition made it unreliable as a deployment step.

The `.htaccess` + version-suffix approach is deterministic and requires no manual steps after FTP upload.

### Lesson

CDN caching for directory indexes (root URLs) is a silent failure mode that doesn't affect development (localhost has no CDN) and doesn't affect testing via explicit file paths (`/index.html`). It only manifests in production, under the root URL, with a delay. The combination of `.htaccess` redirect + versioned filenames provides complete CDN-cache independence for deployments.

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
├── window-vibrancy 0.5                ← macOS NSVisualEffectView frosted glass
│   └── (brings objc2, objc2-app-kit, raw-window-handle as transitives)
│
├── objc2 0.6.0                        ← ObjC runtime bindings (explicit — for CALayer)
├── objc2-app-kit 0.3.0                ← NSView + NSWindow Rust bindings
│   features: ["NSView", "NSWindow"]
├── raw-window-handle 0.6              ← Platform window handle extraction
│
├── serde + serde_json 1.x             ← IPC serialisation (Serialize/Deserialize)
├── log 0.4                            ← Structured logging facade
└── env_logger 0.11                    ← RUST_LOG-driven log subscriber (debug builds)
```

**Frontend:** zero runtime dependencies. The only external load is [Space Grotesk](https://fonts.google.com/specimen/Space+Grotesk) from Google Fonts, cached permanently after first launch.

**Why `objc2` and not `objc` (the older crate)?** `objc` is the original Objective-C runtime binding crate for Rust. `objc2` is the community-maintained successor with safer bindings, better type checking, and active maintenance. `window-vibrancy` already uses `objc2` — declaring it explicitly aligns our direct usage with the version already in the dependency tree, avoiding version conflicts.

</details>

---

## Project Structure

```
junk/
├── src/
│   └── index.html           ← Entire frontend: HTML + CSS + JS, single file, no build step
├── src-tauri/
│   ├── src/
│   │   └── main.rs          ← ~700 lines, all Rust backend logic, heavily commented
│   ├── capabilities/
│   │   └── default.json     ← Tauri permission allowlist (minimal surface area)
│   ├── Cargo.toml           ← Rust dependencies + build profile (LTO, strip, panic=abort)
│   └── tauri.conf.json      ← App config: frameless, transparent, shadow, withGlobalTauri: true
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
| macOS | `Junk_3.1.1_universal.dmg` |
| Windows | `Junk_3.1.1_x64-setup.exe` + `Junk_3.1.1_x64_en-US.msi` |
| Ubuntu | `Junk_3.1.1_amd64.AppImage` + `Junk_3.1.1_amd64.deb` |

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
| Runtime hotkey re-registration silently fires both old and new shortcut | Registered `Shortcut` objects are not auto-deregistered on drop | Hold old `Shortcut` in `CurrentShortcut(Mutex<Shortcut>)` state; call `unregister` explicitly before registering new one |
| `KeyboardEvent.key` ("j", "J") fails on non-QWERTY layouts for hotkey capture | `.key` is layout-dependent — produces wrong characters on AZERTY etc. | Use `KeyboardEvent.code` ("KeyJ") — physical key position, layout-independent |
| `window.outer_position()` returns wrong coordinates after first show | WKWebView reports position before the OS compositor has committed the final frame | Restore position on `tauri://focus` event (not on startup) after 80 ms mouseup delay for save |
| CSS `border-radius` doesn't clip WKWebView blur | WebKit renders WKWebView in a separate compositor layer above CSS | Set `masksToBounds = YES` on contentView CALayer via `objc2::msg_send!` |
| `apply_vibrancy()` with `setCornerRadius` — corners still square on content | `NSVisualEffectView.cornerRadius` only rounds blur subview, not the window frame | Additionally call `set_macos_window_corner_radius()` to set CALayer masksToBounds |
| CSS `box-shadow` clipped after adding `masksToBounds = YES` | `masksToBounds` clips everything drawn outside the layer bounds, including shadows | Use `"shadow": true` in `tauri.conf.json` — native WindowServer shadow operates below the clip |
| SiteGround CDN caches root URL separately from index.html | CDN treats `/` and `/index.html` as distinct cache entries | Use `.htaccess` 302 redirect from `/` to `/index.html` + versioned asset filenames |
| `objc2::msg_send!` macro syntax differs between objc2 versions | Breaking API change in objc2 0.5 → 0.6 | Match the version used by `window-vibrancy` (currently 0.6) — check transitive dep tree |

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
Yes — open Preferences (⌘,), click the shortcut field, and press any key combo. Junk re-registers the OS hotkey immediately, no restart needed. The new key persists in `localStorage`.

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

**Q: The window appears in a different position than where I left it.**
This can happen if you've unplugged an external monitor or changed your display resolution since the last session. The off-screen guard detects positions outside the current screen bounds and discards them, which means the window reverts to its default position. Drag it back to where you want it and it will remember the new position.

**Q: How does the rounded-corner macOS implementation work?**
See the "macOS rounded corners: the full investigation" section in Architecture above. The short version: CSS `border-radius` can't clip WKWebView at the compositor level. The solution is setting `cornerRadius` + `masksToBounds = YES` directly on the `NSWindow contentView`'s `CALayer` via Objective-C runtime calls (`objc2::msg_send!`). This clips the entire compositor subtree — including WKWebView and NSVisualEffectView — at the OS level.

---

## Changelog

### v3.1.1 — 2026-06-06

Always on top restored; preferences toggleable; version label fix; confirmed position/font/theme memory.

- **Always on top restored** — The window's floating-above-all-windows behaviour was inadvertently disabled in v3.0.2 when `alwaysOnTop` was set to `false` in `tauri.conf.json` during the visual rework. Restored in v3.1.1 as a runtime feature: new `set_always_on_top(always_on_top: bool)` Rust IPC command, a toggle in Preferences ("Always on top — Keep Junk above all other windows"), default `ON`, persisted to `localStorage['junk-always-top']`, applied on every startup via `loadAlwaysOnTop()`. New capability permission: `core:window:allow-set-always-on-top`.
- **Version label fix** — The version display in the Preferences panel footer was hardcoded to `v3.0.4`. Now set to the correct version; `loadVersionDisplay()` overwrites it with the live version from `check_for_update()` IPC.
- **Window position memory** — confirmed fully wired: `saveWindowGeometry()` called after every drag (`mouseup`, 80 ms delay) and after every resize (300 ms debounce). `restoreWindowGeometry()` called on startup and on every `tauri://focus` event.
- **Font size memory** — confirmed: `loadFontSize()` on startup reads `localStorage['junk-font-size']`, falls back to 22 px. Slider `input` event saves immediately.
- **Dark mode persistence** — confirmed: `loadTheme()` on startup reads `localStorage['junk-theme']`, defaults to `'auto'`. Buttons save on click.

---

### v3.0.8 — 2026-06-06

Critical fix: restore pre-deep-comment `index.html` — resolves `windowEl` ReferenceError.

- **Root cause** — The educational documentation pass in v3.0.4 rewrote `index.html` from 1567 lines to 2067 lines by adding extensive inline comments. The resulting file passes Node.js syntax checks (`node --check`) and AST brace-depth analysis without issue, but causes `ReferenceError: Can't find variable: windowEl` in WebKit/WKWebView (JavaScriptCore) at runtime. All JavaScript stops executing at the crash point — clipboard copy, markdown toggle, settings panel, and window drag become completely non-functional.
- **Diagnosis** — Console error pointed to `registerDragListener` at the module's closing IIFE, which references `windowEl` (a `const` declared at the module's top level). The crash only occurs in JavaScriptCore; V8 (Node.js, Chrome DevTools) parses the file cleanly. Hypothesis: JavaScriptCore has a different code-gen path for large ES modules and doesn't hoist module-scope `const` declarations in the same way V8 does when the script exceeds a certain size.
- **Fix** — `git checkout 4a18c56 -- src/index.html` restores the 1567-line pre-comment version, which is confirmed working. The deep-commented version is preserved in git history at commit `3b7bef4` for reference.
- **Rule learned** — Keep `index.html` under ~1700 lines. WebKit ES module parsing is not identical to V8's.

---

### v3.0.7 — 2026-06-06

Revert attempt — replaced v3.0.5/v3.0.6 footer overlay with v3.0.4 `index.html`. Still broken because v3.0.4 already contained the deep-comment bug (introduced in the same release). Fixed definitively in v3.0.8.

- **Attempted fix** — Reverted `src/index.html` to the v3.0.4 state, removing the `footer-drag-handle` div and `pointer-events:none` additions from v3.0.5/v3.0.6.
- **Still broken** — v3.0.4's `index.html` was the 2067-line deep-commented version. The `windowEl` crash persisted. Root cause identified in v3.0.8.

---

### v3.0.6 — 2026-06-06

Fix attempt: `pointer-events: none` on footer drag handle — partially fixed.

- **Problem** — Footer buttons (markdown toggle, clipboard, settings) were still unresponsive after v3.0.5. The `footer-drag-handle` div (`position: absolute`) was blocking clicks even though it covered the full footer.
- **Fix** — Added `pointer-events: none` to `.footer-drag-handle`. Clicks now pass through to buttons. Drag via the empty centre zone still works because `mousedown` bubbles up through `<footer>` → `#window` regardless.
- **Still broken** — The `windowEl` ReferenceError from the deep-comment bug in `index.html` was already present in this version. The footer drag/buttons appeared fixed in the diff but were never actually testable.

---

### v3.0.5 — 2026-06-06

Footer drag handle — dragging from bottom bar stopped working after v3.0.4.

- **Problem** — After v3.0.4's visual polish (shadow, glass refinements), dragging the window from the footer's empty centre zone stopped working. Investigation showed the empty zone was no longer propagating `mousedown` events to the `#window` listener.
- **Attempted fix** — Added a dedicated `div.footer-drag-handle` (`position: absolute`, full footer width/height) to the footer HTML, styled with `grab` cursor. The div's `mousedown` was expected to bubble up to `#window` and trigger `start_dragging()`.
- **Regression introduced** — The overlay div covered the markdown, clipboard, and settings buttons, making them unresponsive. This was not caught before release. Fixed in v3.0.6 (`pointer-events: none`) and permanently resolved by reverting to the clean `index.html` in v3.0.8.

---

### v3.0.4 — 2026-06-04

Native macOS window shadow.

- **macOS window shadow** — `"shadow": true` added to the `windows` array in `tauri.conf.json`. This instructs the macOS WindowServer to draw a native drop shadow around the window frame. The shadow is drawn *below* the compositor layer where `masksToBounds = YES` clips, so it is never clipped. It adapts automatically to: window depth (z-order relative to other windows), dark mode (dimmer in dark mode), window state (focused/unfocused), and movement.
- **CSS `box-shadow` revised** — the large ambient drop shadows (`0 32px 80px rgba(0,0,0,0.22)`) removed (they were clipped by `masksToBounds = YES` since v3.0.3). The `box-shadow` now provides only: two inset glass highlights (top specular + bottom rim) and a `0.5px` edge ring for window boundary definition on white backgrounds.
- **Dark mode shadow** — the CSS edge ring inverts in dark mode (becomes a subtle white glow at 8% opacity) to define the window boundary on dark wallpapers.

---

### v3.0.3 — 2026-06-04

True macOS rounded corners via `objc2` CALayer `masksToBounds`.

- **macOS rounded corners** — resolved after three attempts spanning v3.0.2. Final solution: `set_macos_window_corner_radius()` Rust function uses `objc2::msg_send!` to walk `nsView → [nsView window] → [window contentView] → CALayer`, then sets `layer.cornerRadius = 14.0` and `layer.masksToBounds = YES`. The `masksToBounds = YES` clips the entire compositor subtree (WKWebView, NSVisualEffectView, everything) at the OS level — not just CSS content. Called in the Tauri `setup` hook immediately after `apply_vibrancy()`.
- **Dependencies** — `objc2 = "0.6.0"`, `objc2-app-kit = { version = "0.3.0", features = ["NSView", "NSWindow"] }`, `raw-window-handle = "0.6"` added to `Cargo.toml` as explicit direct dependencies (were already transitive via `window-vibrancy`).
- **Bug** — Discovered that `apply_vibrancy()` alone (v3.0.2) only rounded the `NSVisualEffectView` subview's corner — not the window frame. WKWebView still rendered as a square above the blur. This version fixes the root issue.

---

### v3.0.2 — 2026-06-04

`window-vibrancy` for macOS frosted glass (partial — corners still square at WKWebView level).

- **macOS frosted glass** — `window-vibrancy = "0.5"` added. `apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)` called in setup hook. The `NSVisualEffectView` provides the system-native frosted glass blur behind the window. The `backdrop-filter` CSS approach continues to function in parallel (provides the blur sampling from behind-window pixels); the `NSVisualEffectView` adds depth and native vibrancy integration.
- **Partial rounded corners** — `NSVisualEffectView.setCornerRadius(14.0)` was called but only rounded the blur subview. The WKWebView continued rendering as a full rectangle above it. True rounded corners were deferred to v3.0.3.
- **Tauri config** — `"decorations": false` and `"transparent": true` confirmed required for `apply_vibrancy()` to work — it needs to be able to add the `NSVisualEffectView` as a subview behind the window's existing content.

---

### v3.0.1 — 2026-06-04

Clipboard feedback, window size memory, preferences scroll fix.

- **Clipboard feedback** — the footer copy button now shows a checkmark SVG icon and "Copied!" label for 1.5 s after a successful copy, then reverts to the clipboard icon. Previous behaviour was a silent copy with no visual confirmation.
- **Window size memory** — two new IPC commands: `get_window_size` (reads `window.inner_size()`) and `set_window_size` (calls `window.set_size(PhysicalSize)`). JS saves both position and size on `mouseup` (80 ms delay) and on `resize` (200 ms debounce). Restored on `tauri://focus` alongside position. New capability permissions: `core:window:allow-inner-size` and `core:window:allow-set-size`.
- **Preferences scroll fix** — the Preferences panel now has `overflow-y: auto` on the content area. On small screens or when font size is large, preferences content was clipping. The panel now scrolls independently of the window.
- **IPC map updated** — `get_window_size` and `set_window_size` added to `invoke_handler![]`. Total IPC commands: 11.

---

### v3.0.0 — 2026-06-04

The "power user" release — six new features shipping in a single tag. Every one is toggleable, persisted to `localStorage`, and zero-restart.

- **Custom shortcut** — click the shortcut field in Preferences and press any key. Junk unregisters the old OS hotkey and registers the new one at runtime. No restart. Saved as `localStorage['junk-hotkey']`. Key is passed as `KeyboardEvent.code` (layout-independent) → `parse_key_code()` in Rust maps it to `global-shortcut`'s `Code` enum. A `CurrentShortcut(Mutex<Shortcut>)` Tauri managed state holds the active `Shortcut` object so the previous registration can be explicitly unregistered before the new one is created — otherwise both would fire simultaneously.

- **Window position memory** — `get_window_position` / `set_window_position` IPC commands using Tauri's `PhysicalPosition`. JS saves on `mouseup` after any drag (80 ms delay to let OS settle). Restored on `tauri://focus` event so position is correct from the first show. Off-screen guard: positions with `x < -100`, `y < -100`, `x > screen.width + 200`, or `y > screen.height + 200` are silently discarded (protects against position memory across monitor layout changes). New capability permissions `core:window:allow-outer-position` and `core:window:allow-set-position` added to `capabilities/default.json`.

- **Font size** — slider 14–28 px. Drives `--font-size` CSS custom property on `:root`. Both the `<textarea>` and the Markdown preview `<div>` consume `var(--font-size)` directly — no JS duplication. Saved as `localStorage['junk-font-size']`.

- **Dark mode** — three buttons: Light / Auto / Dark. Auto reads `window.matchMedia('(prefers-color-scheme: dark)')` and attaches a live `.addEventListener('change', ...)` — dark mode follows the OS without a restart or page reload. Applied via `data-theme="dark"` attribute on `:root`. Full CSS variable set for dark theme: `--bg-glass: rgba(30,30,35,0.88)`, `--text: #e8e8f0`, `--border: rgba(255,255,255,0.10)`, adjusted box-shadows. Saved as `localStorage['junk-theme']`.

- **Markdown preview** — inline ~80-line parser (no external library, zero bytes added to the bundle). Handles: h1/h2/h3, **bold**, _italic_, `inline code`, fenced code blocks, blockquotes, `---` HR, `- ` unordered lists, `1. ` ordered lists, `[text](url)` links. Toggle: `⌘M`/`Ctrl+M`, the footer markdown button, or the Preferences toggle. Raw text is always stored; the preview is render-only. Saved as `localStorage['junk-md-mode']`.

- **Export** — `navigator.clipboard.writeText(textarea.value)` from the footer copy button and the Preferences "Copy all text" button. Shows "Copied!" for 1.5 s then reverts. Works in both raw and Markdown-preview mode (always exports raw text).

**Rust additions (main.rs):** `CurrentShortcut(Mutex<Shortcut>)` managed state; `set_hotkey(app, key, state)` command; `get_window_position(app)` → `WindowPosition { x: i32, y: i32 }`; `set_window_position(app, x, y)` using `PhysicalPosition::new(x, y)`; `parse_key_code(key: &str) → Result<Code, String>`. All 9 IPC commands registered in `invoke_handler![]`.

---

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

All v3.0.4 features are shipped. Possible future directions:

- **Multi-scratchpad** — named scratch buffers, switchable via shortcut
- **Image paste** — capture and store a screenshot clip inside Junk
- **Encrypted local storage** — optional passphrase-protected content
- **Cross-platform sync** — local-only (no server), LAN-only sync option
- **Tauri v2 iOS/Android** — read-only clipboard viewer on mobile

---

## Vibe coded 🤙

This app was built entirely through vibe coding using various AI tools — Rust backend, frontend, CI pipeline, landing page, and this README included. No prior Tauri or Rust experience required. It works, it ships, and the code is thoroughly commented. But it is what it is.

---

## Author

**Paul Fleury** — [LinkedIn](https://www.linkedin.com/in/paulfxyz/) · [GitHub @paulfxyz](https://github.com/paulfxyz)

---

## License

[MIT](LICENSE) — do whatever you want with it.
