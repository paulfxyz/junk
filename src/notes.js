/* Junk's local tab store. Plain data only: no HTML, file paths or cloud sync.
   Stable IDs (not array indices) prevent a delayed save from writing into the
   wrong tab. Persist the whole snapshot atomically before switching tabs. */
(function (root, factory) {
  const api = factory();
  if (typeof module === 'object' && module.exports) module.exports = api;
  else root.JunkNotes = api;
})(typeof globalThis !== 'undefined' ? globalThis : this, () => {
  const newId = () => (globalThis.crypto?.randomUUID?.() ||
    `note-${Date.now()}-${Math.random().toString(36).slice(2)}`);
  const blank = (text = '') => ({ id: newId(), text });
  function initial(text = '') {
    const tab = blank(text);
    return { version: 1, activeId: tab.id, tabs: [tab], closed: [] };
  }
  function decode(raw, legacy = '') {
    if (!raw) return initial(legacy);
    const value = JSON.parse(raw);
    const validTab = t => t && typeof t.id === 'string' &&
      t.id.length > 0 && typeof t.text === 'string';
    if (value.version !== 1 || !Array.isArray(value.tabs) ||
        !value.tabs.length || !value.tabs.every(validTab) ||
        new Set(value.tabs.map(t => t.id)).size !== value.tabs.length) {
      throw new Error('Invalid tab snapshot');
    }
    // Whitelist fields; data from disk is not executable UI.
    const tabs = value.tabs.map(t => ({ id: t.id, text: t.text }));
    const closed = Array.isArray(value.closed) ? value.closed.filter(validTab)
      .slice(-20).map(t => ({ id: t.id, text: t.text })) : [];
    return { version: 1, tabs, closed,
      activeId: tabs.some(t => t.id === value.activeId) ? value.activeId : tabs[0].id };
  }
  const active = state => state.tabs.find(t => t.id === state.activeId);
  function update(state, text) { active(state).text = text; }
  function add(state) {
    const tab = blank(); state.tabs.push(tab); state.activeId = tab.id; return tab;
  }
  function select(state, id) {
    if (state.tabs.some(t => t.id === id)) state.activeId = id;
  }
  function close(state, id) {
    const index = state.tabs.findIndex(t => t.id === id);
    if (index < 0) return;
    const [tab] = state.tabs.splice(index, 1);
    state.closed.push(tab); state.closed = state.closed.slice(-20);
    if (!state.tabs.length) state.tabs.push(blank());
    if (state.activeId === id) {
      state.activeId = state.tabs[Math.min(index, state.tabs.length - 1)].id;
    }
  }
  function reopen(state) {
    const tab = state.closed.pop(); if (!tab) return;
    // A blank replacement left by closing the last tab is not a real note.
    if (state.tabs.length === 1 && !state.tabs[0].text) state.tabs.length = 0;
    if (state.tabs.some(t => t.id === tab.id)) tab.id = newId();
    state.tabs.push(tab); state.activeId = tab.id;
  }
  function title(tab, index) {
    return tab.text.split(/\r?\n/).find(line => line.trim())?.trim().slice(0, 48) ||
      `Note ${index + 1}`;
  }
  function filename(tab, ext = 'txt') {
    const stem = title(tab, 0).replace(/[<>:"/\\|?*\x00-\x1f]/g, '-')
      .replace(/[. ]+$/g, '').slice(0, 48) || 'junk';
    return `${stem}.${ext}`;
  }
  return { initial, decode, active, update, add, select, close, reopen, title, filename };
});
