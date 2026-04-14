━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🗒️  JUNK  v1.5.0
  The flying macOS scratchpad
  github.com/paulfxyz/junk
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT IS THIS?
─────────────
Junk is a global-hotkey scratchpad that lives
invisibly in the background. No Dock icon.
No menu bar. No accounts. No cloud.

Press ⌘J from any app — a frosted glass window
flies in. Type, paste, think. Press Esc to
dismiss. Everything auto-saves between launches.


INSTALL
───────
1. Drag Junk.app to your Applications folder

2. Open Terminal (Applications → Utilities →
   Terminal) and run these two commands:

   xattr -rd com.apple.quarantine /Applications/Junk.app
   open /Applications/Junk.app

   (This removes the macOS quarantine flag for
   apps not signed with an Apple certificate.
   You only need to do this once.)

3. Junk starts silently. Press ⌘J from anywhere.


HOW TO USE
──────────
  ⌘J       → open / close the window
  Esc      → close the window
  ⌘A       → select all text
  Drag     → move the window (grab the border
              or the footer bar)


WHY THE TERMINAL STEP?
──────────────────────
macOS Gatekeeper quarantines apps downloaded
from the internet that aren't signed with a
paid Apple Developer certificate ($99/year).

The xattr command strips that quarantine flag.
It's a standard procedure for open-source apps
distributed outside the Mac App Store.

After running it once, Junk launches normally
every time — no Terminal needed again.


WHAT'S UNDER THE HOOD
──────────────────────
Built with Electron 31. The window uses
macOS native vibrancy (under-window) behind
a frosted glass CSS layer, giving it the same
blurred-wallpaper effect as Spotlight or
Control Center.

The shortcut is registered at OS level via
globalShortcut — it works inside any app,
terminal, fullscreen window, or game.

Content is stored in localStorage at:
  ~/Library/Application Support/junk/

No network requests. No telemetry. No accounts.
Your text never leaves your machine.


SOURCE CODE
───────────
  github.com/paulfxyz/junk
  MIT License — fork it, modify it, ship it.


AUTHOR
──────
  Paul Fleury — paulfleury.com
  hello@paulfleury.com

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
