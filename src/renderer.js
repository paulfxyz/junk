'use strict';

const pad = document.getElementById('pad');

// ─────────────────────────────────────────────
//  Persistence
// ─────────────────────────────────────────────
const KEY = 'junk:content';

function load() {
  const saved = localStorage.getItem(KEY);
  if (saved !== null) pad.value = saved;
}

let saveTimer;
function scheduleSave() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => localStorage.setItem(KEY, pad.value), 300);
}

pad.addEventListener('input', scheduleSave);

// ─────────────────────────────────────────────
//  Focus
// ─────────────────────────────────────────────
function focusPad() {
  pad.focus();
  const len = pad.value.length;
  pad.setSelectionRange(len, len);
}

// Called by main when window is shown via ⌘J
window.junk?.onShown(() => focusPad());

window.addEventListener('DOMContentLoaded', () => {
  load();
  focusPad();
});

// ─────────────────────────────────────────────
//  Keyboard
// ─────────────────────────────────────────────
document.addEventListener('keydown', (e) => {
  // ⌘J — hide (main also handles this via globalShortcut, belt-and-suspenders)
  if ((e.metaKey || e.ctrlKey) && e.key === 'j') {
    e.preventDefault();
    scheduleSave();
    window.junk?.hide();
  }

  // ⌘A — select all
  if ((e.metaKey || e.ctrlKey) && e.key === 'a') {
    e.preventDefault();
    pad.select();
  }
});

// ─────────────────────────────────────────────
//  Paste anywhere — even if focus drifted
// ─────────────────────────────────────────────
document.addEventListener('paste', (e) => {
  if (document.activeElement !== pad) {
    e.preventDefault();
    const text = e.clipboardData.getData('text/plain');
    const s = pad.selectionStart;
    const end = pad.selectionEnd;
    pad.value = pad.value.slice(0, s) + text + pad.value.slice(end);
    pad.selectionStart = pad.selectionEnd = s + text.length;
    scheduleSave();
  }
});
