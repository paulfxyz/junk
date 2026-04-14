'use strict';

const { app, BrowserWindow, globalShortcut, screen, Tray, Menu, nativeImage, ipcMain } = require('electron');
const path = require('path');

// Keep references alive so GC doesn't kill them
let tray = null;
let win = null;
let isVisible = false;

// ─────────────────────────────────────────────
//  App settings
// ─────────────────────────────────────────────
const WINDOW_WIDTH  = 720;
const WINDOW_HEIGHT = 480;
const SHORTCUT      = 'CommandOrControl+J';

// Hide from dock — Junk is a background utility
app.dock?.hide();

// ─────────────────────────────────────────────
//  Window factory
// ─────────────────────────────────────────────
function createWindow() {
  const { workAreaSize, bounds } = screen.getPrimaryDisplay();

  // Center on screen
  const x = Math.round(bounds.x + (workAreaSize.width  - WINDOW_WIDTH)  / 2);
  const y = Math.round(bounds.y + (workAreaSize.height - WINDOW_HEIGHT) / 2);

  win = new BrowserWindow({
    width:  WINDOW_WIDTH,
    height: WINDOW_HEIGHT,
    x,
    y,
    frame:               false,      // frameless — we draw our own chrome
    transparent:         true,       // vibrancy needs transparent host
    vibrancy:            'under-window', // macOS glass effect
    visualEffectState:   'active',
    alwaysOnTop:         true,
    skipTaskbar:         true,
    resizable:           false,
    movable:             true,
    show:                false,       // show() is called after ready-to-show
    hasShadow:           true,
    titleBarStyle:       'hidden',
    webPreferences: {
      nodeIntegration:  false,
      contextIsolation: true,
      preload:          path.join(__dirname, 'preload.js'),
    },
  });

  win.loadFile(path.join(__dirname, 'index.html'));

  // Hide instead of close when user presses Cmd+W or the × button
  win.on('close', (e) => {
    e.preventDefault();
    hideWindow();
  });

  // Lose focus → auto-hide (like Spotlight)
  win.on('blur', () => {
    hideWindow();
  });
}

// ─────────────────────────────────────────────
//  Show / hide helpers
// ─────────────────────────────────────────────
function showWindow() {
  if (!win) createWindow();

  // Re-center on whichever display the cursor is on
  const cursor  = screen.getCursorScreenPoint();
  const display = screen.getDisplayNearestPoint(cursor);
  const { bounds, workAreaSize } = display;

  const x = Math.round(bounds.x + (workAreaSize.width  - WINDOW_WIDTH)  / 2);
  const y = Math.round(bounds.y + (workAreaSize.height - WINDOW_HEIGHT) / 2);

  win.setPosition(x, y, false);
  win.show();
  win.focus();
  win.webContents.send('window-shown');
  isVisible = true;
}

function hideWindow() {
  if (win && isVisible) {
    win.hide();
    isVisible = false;
  }
}

function toggleWindow() {
  if (isVisible) {
    hideWindow();
  } else {
    showWindow();
  }
}

// ─────────────────────────────────────────────
//  Tray icon (small menu bar icon)
// ─────────────────────────────────────────────
function createTray() {
  // 16×16 or 22×22 menu bar icon — we use a tiny inline PNG for portability
  const icon = nativeImage.createFromDataURL(getTrayIconDataURL());
  icon.setTemplateImage(true); // adapts to light/dark menu bar automatically

  tray = new Tray(icon);
  tray.setToolTip('Junk — CMD+J to open');

  const menu = Menu.buildFromTemplate([
    {
      label: 'Open Junk',
      accelerator: 'CommandOrControl+J',
      click: toggleWindow,
    },
    { type: 'separator' },
    {
      label: 'Quit Junk',
      accelerator: 'CommandOrControl+Q',
      click: () => {
        app.exit(0);
      },
    },
  ]);

  tray.setContextMenu(menu);
  tray.on('click', toggleWindow);
}

// ─────────────────────────────────────────────
//  App lifecycle
// ─────────────────────────────────────────────
app.whenReady().then(() => {
  createTray();
  createWindow(); // pre-warm window for snappy first open

  // Register global shortcut
  const ok = globalShortcut.register(SHORTCUT, toggleWindow);
  if (!ok) {
    console.error(`[junk] Could not register global shortcut ${SHORTCUT}`);
  }

  // IPC: renderer asks to hide
  ipcMain.on('hide-window', hideWindow);
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
});

// Prevent default quit behavior — we live in the tray
app.on('window-all-closed', (e) => {
  // Do nothing — keep alive
});

// ─────────────────────────────────────────────
//  Tray icon data (16×16 white "J" on transparent)
//  Generated as an inline base64 PNG so we need
//  zero external assets at runtime.
// ─────────────────────────────────────────────
function getTrayIconDataURL() {
  // A minimal 22×22 template icon: white rounded square with "J"
  // (template image → macOS inverts for dark/light mode automatically)
  return 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABYAAAAWCAYAAADEtGw7AAAACXBIWXMAAAsTAAALEwEAmpwYAAABIElEQVQ4jb3UMUoDQRTG8f+bXWMhFkIKC8FLeAAvIHgBL+AFxMpKEMErWFh5AQ9gZWFhYSEiIiIiIiIiIiIiIiIiIiIiIiIiIoJgICTZzc4m+2C3sBR2n9/MvHkzQxJJJJFEEkkkkcT/8Q5cAx/ANbAHbAFbQA1cAmdJ0r0Bx8B5kuwBKXAB3ANPwApwBaRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRSSimllFJKKaWUUkoppZRS6g+/AV8DrsGvAAAAAElFTkSuQmCC';
}
