// Guards the JS <-> Rust command boundary.
//
// Tauri v2 converts Rust argument names to camelCase on the JS side
// (`always_on_top` -> `alwaysOnTop`). Sending the snake_case key makes the call
// fail at runtime with "missing required key", and app.js swallows the error.
// That silently broke the "Always on top" toggle for several releases, so this
// test compares every ipc('command', {...}) call with the Rust signature.
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');

const rust = fs.readFileSync('src-tauri/src/main.rs', 'utf8');
const js = fs.readFileSync('src/app.js', 'utf8');
const camel = s => s.replace(/_([a-z])/g, (_, c) => c.toUpperCase());
const INJECTED = /^(AppHandle|WebviewWindow|Window|State|tauri::)/;

// Keys of an object literal, ignoring anything nested inside (), [] or {}.
function topLevelKeys(src) {
  const keys = []; let depth = 0, token = '';
  const flush = () => { const k = token.split(':')[0].trim(); if (k) keys.push(k); token = ''; };
  for (const ch of src.slice(1, -1)) {
    if ('([{'.includes(ch)) depth++;
    if (')]}'.includes(ch)) depth--;
    if (ch === ',' && depth === 0) flush(); else token += ch;
  }
  flush();
  return keys;
}

const commands = {};
for (const m of rust.matchAll(/#\[tauri::command\]\s*(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)/g)) {
  // Split at top-level commas only: `State<'_, T>` contains a comma of its own.
  const params = []; let depth = 0, cur = '';
  for (const ch of m[2]) {
    if ('(<[{'.includes(ch)) depth++;
    if (')>]}'.includes(ch)) depth--;
    if (ch === ',' && depth === 0) { params.push(cur); cur = ''; } else cur += ch;
  }
  params.push(cur);
  commands[m[1]] = params.map(a => a.trim()).filter(Boolean)
    .map(a => [a.slice(0, a.indexOf(':')).trim(), a.slice(a.indexOf(':') + 1).trim()])
    .filter(([, type]) => !INJECTED.test(type))
    .map(([name]) => camel(name));
}

test('every ipc() call targets a registered command with matching arg names', () => {
  const registered = rust.match(/generate_handler!\[([\s\S]*?)\]/)[1]
    .split(',').map(s => s.trim()).filter(Boolean);
  let checked = 0;
  for (const m of js.matchAll(/ipc\('(\w+)'(?:,\s*(\{[\s\S]*?\n?[^\n]*?\}))?\s*\)/g)) {
    const [, name, obj = ''] = m;
    assert.ok(registered.includes(name), `${name} is not in generate_handler!`);
    assert.deepEqual(topLevelKeys(obj).sort(), [...commands[name]].sort(), `args for ${name}`);
    checked++;
  }
  assert.ok(checked >= 8, `only checked ${checked} calls`);
});

test('new 3.2 commands exist', () => {
  assert.deepEqual(commands.save_text_file.sort(), ['contents', 'suggestedName']);
  assert.deepEqual(commands.set_auto_hide, ['seconds']);
});

test('dialog plugin is Rust-only: the WebView gets no dialog permission', () => {
  const caps = fs.readFileSync('src-tauri/capabilities/default.json', 'utf8');
  assert.ok(!/dialog:|fs:/.test(caps));
});
