'use strict';

// ─────────────────────────────────────────────
//  Elements
// ─────────────────────────────────────────────
const pad       = document.getElementById('pad');
const closeBtn  = document.getElementById('closeBtn');

// ─────────────────────────────────────────────
//  Persistence — localStorage survives app restarts
// ─────────────────────────────────────────────
const STORAGE_KEY = 'junk:pad-content';

function load() {
  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved !== null) pad.value = saved;
}

function save() {
  localStorage.setItem(STORAGE_KEY, pad.value);
}

// Debounced save — writes 400ms after last keystroke
let saveTimer;
pad.addEventListener('input', () => {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(save, 400);
});

// ─────────────────────────────────────────────
//  Focus management
// ─────────────────────────────────────────────
function focusPad() {
  pad.focus();
  // Move cursor to end so user can keep typing
  const len = pad.value.length;
  pad.setSelectionRange(len, len);
}

// Called by main process every time the window is shown
window.junk?.onShown(() => {
  focusPad();
});

// Also focus on initial load (first open)
window.addEventListener('DOMContentLoaded', () => {
  load();
  focusPad();
});

// ─────────────────────────────────────────────
//  Keyboard shortcuts
// ─────────────────────────────────────────────
document.addEventListener('keydown', (e) => {
  // Esc → close
  if (e.key === 'Escape') {
    save();
    window.junk?.hide();
  }

  // Cmd+A → select all (textarea handles it natively, but just in case)
  if (e.key === 'a' && e.metaKey) {
    e.preventDefault();
    pad.select();
  }
});

// ─────────────────────────────────────────────
//  Close button
// ─────────────────────────────────────────────
closeBtn.addEventListener('click', () => {
  save();
  window.junk?.hide();
});

// ─────────────────────────────────────────────
//  Paste anywhere — even if textarea isn't focused
// ─────────────────────────────────────────────
document.addEventListener('paste', (e) => {
  if (document.activeElement !== pad) {
    e.preventDefault();
    const text = e.clipboardData.getData('text/plain');
    const start = pad.selectionStart;
    const end   = pad.selectionEnd;
    pad.value = pad.value.slice(0, start) + text + pad.value.slice(end);
    pad.selectionStart = pad.selectionEnd = start + text.length;
    save();
  }
});
