const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const { parseHTML } = require('linkedom');
const md = require('../src/markdown.js');

function render(text) {
  const { document } = parseHTML('<html><body><div id="root"></div></body></html>');
  const root = document.getElementById('root');
  md.render(root, text);
  return root;
}
for (const prefix of ['', '# ', '## ', '### ', '- ', '1. ', '> ']) {
  test(`HTML is literal in block ${JSON.stringify(prefix)}`, () => {
    const payload = '<img src=x onerror="globalThis.compromised=1"><script>bad()</script>';
    const root = render(prefix + payload);
    assert.equal(root.querySelectorAll('img,script,iframe,svg').length, 0);
    assert.ok(root.textContent.includes(payload));
  });
}
for (const payload of [
  '**<img src=x onerror=bad()>**',
  '_<svg onload=bad()>_',
  '`<iframe src="https://example.com">`',
  '[<img src=x>](https://example.com)',
  '# <span data-audit-marker="yes">Harmless marker</span>',
  '- <a href="javascript:bad()">text</a>',
  '&lt;img src=x onerror=bad()&gt;'
]) {
  test(`inert inline content: ${payload}`, () => {
    const root = render(payload);
    assert.equal(root.querySelectorAll('img,script,iframe,svg,span').length, 0);
    for (const el of root.querySelectorAll('*')) {
      assert.ok([...el.attributes].every(a => !a.name.startsWith('on')));
    }
  });
}
for (const url of [
  'javascript:alert(1)', 'JaVaScRiPt:alert%281%29',
  'data:text/html,hello', 'vbscript:bad', 'file:///etc/passwd',
  'tauri://localhost', 'ipc://localhost', 'asset://localhost/a',
  '//example.com', '/relative', 'https://user:secret@example.com',
  'java\tscript:bad', 'https://example.com/\npath',
  'javascript&colon;alert', 'mailto:'
]) {
  test(`reject unsafe/ambiguous URL ${JSON.stringify(url)}`, () => {
    assert.equal(md.safeUrl(url), null);
    assert.equal(render(`[open](${url})`).querySelectorAll('a').length, 0);
  });
}
for (const url of ['https://example.com/a?q=one&x=two', 'http://example.com', 'mailto:hello@example.com']) {
  test(`allow explicit safe URL ${url}`, () => {
    const a = render(`[open](${url})`).querySelector('a');
    assert.ok(a);
    assert.equal(a.getAttribute('href'), new URL(url).href);
    assert.equal(a.getAttribute('rel'), 'noopener noreferrer');
    assert.equal(a.getAttribute('target'), '_blank');
  });
}
test('quotes in URL cannot inject attributes', () => {
  const root = render('[open](https://example.com/"onmouseover="bad)');
  assert.equal(root.querySelector('[onmouseover]'), null);
});
test('formatting and mixed lists remain supported', () => {
  const r = render('# Title\n**bold** and _italic_ and `code`\n- first\n- second\n1. third\n> quoted\n---');
  for (const selector of ['h1','strong','em','code','ul','ol','blockquote','hr']) assert.ok(r.querySelector(selector));
  assert.equal(r.querySelectorAll('ul li').length, 2);
  assert.equal(r.querySelectorAll('ol li').length, 1);
});
test('code is literal, including HTML and Markdown punctuation', () => {
  const r = render('```html\n<img src=x>\n**not bold**\n```');
  assert.equal(r.querySelector('pre code').textContent, '<img src=x>\n**not bold**');
  assert.equal(r.querySelectorAll('img,strong').length, 0);
});
test('unterminated and tilde fences preserve text', () => {
  assert.equal(render('~~~\n<svg>\n**literal**').querySelector('pre code').textContent, '<svg>\n**literal**');
});
test('replacement does not duplicate the prior preview', () => {
  const r = render('# Old');
  md.render(r, '# New');
  assert.equal(r.textContent, 'New');
  assert.equal(r.querySelectorAll('h1').length, 1);
});
test('CSP and local resources remain restricted', () => {
  const config = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json'));
  const csp = config.app.security.csp;
  assert.ok(csp.includes("script-src 'self'"));
  assert.ok(csp.includes("object-src 'none'"));
  assert.ok(!csp.includes('unsafe-eval'));
  assert.ok(!/script-src[^;]*unsafe-inline/.test(csp));
  const html = fs.readFileSync('src/index.html','utf8');
  assert.ok(!html.includes('fonts.googleapis.com'));
  assert.ok(!/<script(?![^>]*\bsrc=)[^>]*>/i.test(html));
  const capabilities = JSON.parse(fs.readFileSync('src-tauri/capabilities/default.json'));
  assert.equal(capabilities.local, true);
  assert.ok(!capabilities.permissions.includes('http:default'));
});
