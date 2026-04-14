'use strict';

const pad = document.getElementById('pad');
const KEY = 'junk:content';

// ── Persistence ──
function load() {
  const s = localStorage.getItem(KEY);
  if (s !== null) pad.value = s;
}

let saveTimer;
function scheduleSave() {
  clearTimeout(saveTimer);
  saveTimer = setTimeout(() => localStorage.setItem(KEY, pad.value), 300);
}
pad.addEventListener('input', scheduleSave);

// ── Focus ──
function focusPad() {
  pad.focus();
  const len = pad.value.length;
  pad.setSelectionRange(len, len);
}

window.junk?.onShown(() => focusPad());

window.addEventListener('DOMContentLoaded', () => {
  load();
  focusPad();
});

// ── Keyboard ──
document.addEventListener('keydown', (e) => {
  // Esc → hide
  if (e.key === 'Escape') {
    scheduleSave();
    window.junk?.hide();
    return;
  }

  // ⌘J → toggle (belt-and-suspenders)
  if ((e.metaKey || e.ctrlKey) && e.key === 'j') {
    e.preventDefault();
    scheduleSave();
    window.junk?.hide();
    return;
  }

  // ⌘A → select all
  if ((e.metaKey || e.ctrlKey) && e.key === 'a') {
    e.preventDefault();
    pad.select();
  }
});

// ── Paste anywhere ──
document.addEventListener('paste', (e) => {
  if (document.activeElement !== pad) {
    e.preventDefault();
    const text = e.clipboardData.getData('text/plain');
    const s = pad.selectionStart, end = pad.selectionEnd;
    pad.value = pad.value.slice(0, s) + text + pad.value.slice(end);
    pad.selectionStart = pad.selectionEnd = s + text.length;
    scheduleSave();
  }
});
