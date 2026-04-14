# Installing Junk

## Quick install (recommended)

1. Go to the [latest release](https://github.com/paulfxyz/junk/releases/latest)
2. Download the correct `.dmg`:
   - **Apple Silicon (M1, M2, M3, M4)** → `Junk-1.0.0-arm64.dmg`
   - **Intel Mac** → `Junk-1.0.0-x64.dmg`
3. Open the `.dmg` file
4. Drag **Junk.app** to the `/Applications` folder shortcut
5. Eject the disk image

## First launch (Gatekeeper bypass)

Junk is not code-signed with an Apple Developer certificate. macOS will block it on first open.

**Option A — Right-click method:**
1. Open `/Applications`
2. Right-click **Junk.app** → **Open**
3. Click **Open** in the dialog

**Option B — Terminal method:**
```bash
xattr -rd com.apple.quarantine /Applications/Junk.app
open /Applications/Junk.app
```

You only need to do this once.

## Verify it's running

- You should see a small `J` icon in your menu bar
- Press `⌘J` from anywhere — the Junk window should fly in
- Press `Esc` to dismiss

## Uninstall

```bash
# Remove the app
rm -rf /Applications/Junk.app

# Remove saved content and preferences
rm -rf ~/Library/Application\ Support/junk
```

## Build from source

```bash
git clone https://github.com/paulfxyz/junk.git
cd junk
npm install

# Generate ICNS icon (macOS only)
iconutil -c icns assets/icon.iconset -o assets/icon.icns

# Development run
npm start

# Build DMG
npm run build
# → dist/Junk-1.0.0-arm64.dmg
# → dist/Junk-1.0.0-x64.dmg
```

**Requirements:** Node.js 20+, macOS (for `iconutil` and `.icns` generation)
