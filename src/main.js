'use strict';

const { app, BrowserWindow, globalShortcut, screen, ipcMain } = require('electron');
const path = require('path');

let win = null;
let isVisible = false;

const WINDOW_WIDTH  = 720;
const WINDOW_HEIGHT = 460;
const SHORTCUT      = 'CommandOrControl+J';

app.dock?.hide();

function createWindow() {
  const display = screen.getPrimaryDisplay();
  const { bounds, workAreaSize } = display;
  const x = Math.round(bounds.x + (workAreaSize.width  - WINDOW_WIDTH)  / 2);
  const y = Math.round(bounds.y + (workAreaSize.height - WINDOW_HEIGHT) / 2);

  win = new BrowserWindow({
    width:  WINDOW_WIDTH,
    height: WINDOW_HEIGHT,
    x, y,
    // titleBarStyle hiddenInset gives a native macOS window with proper
    // shadow + vibrancy, but pushes traffic lights off-screen
    titleBarStyle:        'hiddenInset',
    trafficLightPosition: { x: -100, y: -100 },
    // Transparent so backdrop-filter: blur() in CSS sees through to the desktop
    transparent:          true,
    backgroundColor:      '#00000000',
    vibrancy:             'under-window',
    visualEffectState:    'active',
    alwaysOnTop:          true,
    skipTaskbar:          true,
    resizable:            true,
    movable:              true,
    show:                 false,
    hasShadow:            true,
    roundedCorners:       true,
    webPreferences: {
      nodeIntegration:  false,
      contextIsolation: true,
      preload:          path.join(__dirname, 'preload.js'),
    },
  });

  win.loadFile(path.join(__dirname, 'index.html'));

  win.on('close', (e) => {
    e.preventDefault();
    hideWindow();
  });

  // NO blur handler — stays open when you switch apps
}

function showWindow() {
  if (!win) createWindow();

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
  isVisible ? hideWindow() : showWindow();
}

app.whenReady().then(() => {
  createWindow();
  const ok = globalShortcut.register(SHORTCUT, toggleWindow);
  if (!ok) console.error(`[junk] Could not register ${SHORTCUT}`);
  ipcMain.on('hide-window', hideWindow);
});

app.on('will-quit', () => globalShortcut.unregisterAll());
app.on('window-all-closed', () => {});
