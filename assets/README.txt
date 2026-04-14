Junk — App Icons
================

This directory contains the icon assets for the Junk scratchpad app
(Tauri v2 / Rust edition).

Required files
--------------
Tauri v2 reads these paths from the `bundle.icon` array in tauri.conf.json:

  icons/32x32.png          — 32×32 px,  PNG, used for Linux taskbar / .deb
  icons/128x128.png        — 128×128 px, PNG, used for Linux AppImage / .deb
  icons/128x128@2x.png     — 256×256 px, PNG, used for macOS Retina display
  icons/icon.icns          — macOS icon bundle (contains 16–1024 px sizes)
  icons/icon.ico           — Windows icon (contains 16, 32, 48, 256 px sizes)

Optional (for Windows Store / modern installers):

  icons/Square30x30Logo.png
  icons/Square44x44Logo.png
  icons/Square71x71Logo.png
  icons/Square89x89Logo.png
  icons/Square107x107Logo.png
  icons/Square142x142Logo.png
  icons/Square150x150Logo.png
  icons/Square284x284Logo.png
  icons/Square310x310Logo.png
  icons/StoreLogo.png

How to generate
---------------
1. Start with a single 1024×1024 px master PNG (e.g. junk-icon-master.png).

2. Use the Tauri CLI's built-in icon generator:
     npx @tauri-apps/cli icon assets/icons/junk-icon-master.png
   This creates all required sizes in src-tauri/icons/ automatically.
   Copy or symlink that output to assets/icons/.

3. Alternatively, use a tool like:
   - https://www.npmjs.com/package/icon-gen  (npm)
   - https://github.com/nicowillis/icon-maker (macOS .icns)
   - https://www.icoconverter.com/            (Windows .ico)

Icon design notes (Junk Rust edition)
---------------------------------------
The original Junk icon is a simple "J" letterform with a torn-paper or
scratch aesthetic — rough edges, slight tilt, one-colour mark. It works
at 16 px (favicon-size) and at 512 px (Retina display) because it avoids
fine detail that disappears at small sizes.

For this Rust edition, consider updating the icon to include a subtle
ferris-the-crab (🦀) or Tauri logo element to signal the technology stack
to technical users — but keep the primary "J" mark dominant so the brand
is recognisable to existing users.

Notes for developers
--------------------
- The icons/ directory is intentionally empty in the Git repository.
  Binary assets inflate repo size and cause merge conflicts. Generate them
  locally from the master PNG using the CLI command above.

- Tauri will produce a build error if any listed icon file is missing.
  The error message will name the missing file explicitly.

- On macOS, the .icns file must contain at minimum: 16, 32, 128, 256, 512 px
  sizes. The Tauri CLI generates all of these from a 1024 px source.

- On Windows, the .ico file should contain 16, 32, 48, and 256 px variants.
  256 px is required for the modern Windows installer UI.

Version
-------
This README applies to Junk v1.6.0 (Tauri v2 / Rust rewrite).
The Electron version's icon assets are compatible — same filenames, same sizes.
