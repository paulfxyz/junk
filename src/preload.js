'use strict';

const { contextBridge, ipcRenderer } = require('electron');

// Expose a safe, minimal API to the renderer
contextBridge.exposeInMainWorld('junk', {
  // Called by main process when window becomes visible — focuses textarea
  onShown: (callback) => ipcRenderer.on('window-shown', callback),

  // Close / hide the window
  hide: () => ipcRenderer.send('hide-window'),

  // Platform info
  platform: process.platform,
});
