'use strict';

const { app, BrowserWindow, globalShortcut, screen, ipcMain } = require('electron');
const path = require('path');

let win = null;
let isVisible = false;

const WINDOW_WIDTH  = 720;
const WINDOW_HEIGHT = 480;
const SHORTCUT      = 'CommandOrControl+J';

// No dock icon — pure background utility
app.dock?.hide();

// ─────────────────────────────────────────────
//  Window factory
// ─────────────────────────────────────────────
function createWindow() {
  const display = screen.getPrimaryDisplay();
  const { bounds, workAreaSize } = display;

  const x = Math.round(bounds.x + (workAreaSize.width  - WINDOW_WIDTH)  / 2);
  const y = Math.round(bounds.y + (workAreaSize.height - WINDOW_HEIGHT) / 2);

  win = new BrowserWindow({
    width:  WINDOW_WIDTH,
    height: WINDOW_HEIGHT,
    x,
    y,
    frame:             false,
    transparent:       true,
    vibrancy:          'under-window',
    visualEffectState: 'active',
    alwaysOnTop:       true,
    skipTaskbar:       true,
    resizable:         true,
    movable:           true,
    show:              false,
    hasShadow:         true,
    // No titleBarStyle — fully frameless, no traffic lights
    webPreferences: {
      nodeIntegration:  false,
      contextIsolation: true,
      preload:          path.join(__dirname, 'preload.js'),
    },
  });

  win.loadFile(path.join(__dirname, 'index.html'));

  // Intercept close — just hide instead
  win.on('close', (e) => {
    e.preventDefault();
    hideWindow();
  });

  // ⚠️  NO blur handler — window stays open when user switches apps
  //     Only ⌘J toggles it
}

// ─────────────────────────────────────────────
//  Show / hide
// ─────────────────────────────────────────────
function showWindow() {
  if (!win) createWindow();

  // Re-center on the display where the cursor currently is
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
//  App lifecycle
// ─────────────────────────────────────────────
app.whenReady().then(() => {
  createWindow();

  const ok = globalShortcut.register(SHORTCUT, toggleWindow);
  if (!ok) console.error(`[junk] Could not register ${SHORTCUT}`);

  ipcMain.on('hide-window', hideWindow);
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
});

app.on('window-all-closed', () => {
  // Keep alive — no tray, no dock, no window needed to stay running
});
