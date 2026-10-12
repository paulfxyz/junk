// Unit tests for the tab store (src/notes.js). Pure data logic, no DOM needed.
const test = require('node:test');
const assert = require('node:assert/strict');
const N = require('../src/notes.js');

test('initial state has one tab, and keeps legacy text', () => {
  const s = N.initial('old note');
  assert.equal(s.tabs.length, 1);
  assert.equal(N.active(s).text, 'old note');
});
test('decode migrates legacy content when no snapshot exists', () => {
  assert.equal(N.active(N.decode(null, 'legacy')).text, 'legacy');
});
test('snapshot round-trips and unknown fields are dropped', () => {
  const s = N.initial('a'); N.add(s); N.update(s, 'b');
  const raw = JSON.stringify({ ...s, tabs: s.tabs.map(t => ({ ...t, evil: '<img>' })) });
  const d = N.decode(raw);
  assert.deepEqual(d.tabs.map(t => t.text), ['a', 'b']);
  assert.equal(d.activeId, s.activeId);
  assert.ok(!('evil' in d.tabs[0]));
});
test('invalid snapshots throw so the app can back them up', () => {
  assert.throws(() => N.decode('{"version":2,"tabs":[]}'));
  assert.throws(() => N.decode('not json'));
  assert.throws(() => N.decode(JSON.stringify({ version: 1, tabs: [{ id: 'x', text: 1 }] })));
  const dup = { id: 'x', text: '' };
  assert.throws(() => N.decode(JSON.stringify({ version: 1, tabs: [dup, dup] })));
});
test('unknown activeId falls back to the first tab', () => {
  const s = N.initial('a'); s.activeId = 'ghost';
  assert.equal(N.decode(JSON.stringify(s)).activeId, s.tabs[0].id);
});
test('add selects the new tab; select ignores unknown ids', () => {
  const s = N.initial('a'); const t = N.add(s);
  assert.equal(s.activeId, t.id);
  N.select(s, 'nope'); assert.equal(s.activeId, t.id);
  N.select(s, s.tabs[0].id); assert.equal(s.activeId, s.tabs[0].id);
});
test('closing the active tab selects a neighbour', () => {
  const s = N.initial('a'); N.add(s); N.add(s);
  const [t0, t1, t2] = s.tabs.map(t => t.id);
  N.select(s, t1); N.close(s, t1);
  assert.equal(s.activeId, t2);
  N.close(s, t2); assert.equal(s.activeId, t0);
});
test('closing the last tab leaves one blank tab and keeps the text recoverable', () => {
  const s = N.initial('precious'); N.close(s, s.tabs[0].id);
  assert.equal(s.tabs.length, 1); assert.equal(N.active(s).text, '');
  N.reopen(s);
  assert.deepEqual(s.tabs.map(t => t.text), ['precious']);
});
test('reopen restores the most recently closed tab', () => {
  const s = N.initial('a'); const b = N.add(s); N.update(s, 'b');
  N.close(s, b.id); assert.equal(s.tabs.length, 1);
  N.reopen(s); assert.equal(N.active(s).text, 'b'); assert.equal(s.tabs.length, 2);
  N.reopen(s); assert.equal(s.tabs.length, 2); // nothing left to reopen
});
test('closed history is bounded', () => {
  const s = N.initial('');
  for (let i = 0; i < 40; i++) { const t = N.add(s); N.close(s, t.id); }
  assert.ok(s.closed.length <= 20);
});
test('titles use the first non-empty line, else a numbered fallback', () => {
  assert.equal(N.title({ text: '\n  hello world \nmore' }, 0), 'hello world');
  assert.equal(N.title({ text: '' }, 2), 'Note 3');
  assert.equal(N.title({ text: 'x'.repeat(100) }, 0).length, 48);
});
test('filenames are safe and take an extension', () => {
  assert.equal(N.filename({ text: 'Shopping list' }), 'Shopping list.txt');
  assert.equal(N.filename({ text: 'a/b\\c:d?*' }, 'md'), 'a-b-c-d--.md');
  assert.equal(N.filename({ text: '' }), 'Note 1.txt');
  assert.ok(!/[<>:"/\\|?*]/.test(N.filename({ text: '<x>"y"|z' })));
});
