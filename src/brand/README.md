# Junk identity

`junk.svg` is the canonical, outlined campaign artwork: a white dotless j with exactly one peach dot on a near-black rounded tile. Do not substitute a font-rendered capital J, add another dot or use the Weekathon kitty as the app icon.

The desktop icons in `assets/icons/` derive from the matching 1024px campaign PNG. Regenerate with `npm run tauri icon -- path/to/icon-1024.png --output assets/icons`; keep the desktop PNG, ICO and ICNS outputs. The Weekathon mascot is a separate campaign component.

App accents are rust `#A0521E` in light mode and peach `#FFE5D0` in dark mode. Dark mode remains a functional user preference; it does not change the campaign's light art direction.
