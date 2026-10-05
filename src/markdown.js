/* Junk's deliberately small Markdown renderer. Note content is never HTML. */
(function (global) {
  'use strict';

  function safeUrl(raw) {
    // No implicit base URL, protocol-relative links, controls or whitespace.
    if (/[\u0000-\u0020\u007f]/.test(raw)) return null;
    try {
      const url = new URL(raw);
      if (!['https:', 'http:', 'mailto:'].includes(url.protocol)) return null;
      if (url.username || url.password) return null;
      if (url.protocol === 'mailto:' && !url.pathname) return null;
      return url.href;
    } catch {
      return null;
    }
  }

  function inline(parent, text, doc, depth = 0, allowLinks = true) {
    if (depth >= 8) {
      parent.appendChild(doc.createTextNode(text));
      return;
    }
    // Match text tokens before constructing nodes. Generated markup is never
    // fed back through a string parser, and raw HTML has no special meaning.
    const token = /`([^`\n]+)`|\*\*([^*\n]+)\*\*|__([^_\n]+)__|\*([^*\n]+)\*|_([^_\n]+)_|\[([^\]\n]+)\]\(([^)\n]*)\)/g;
    let end = 0;
    for (const match of text.matchAll(token)) {
      parent.appendChild(doc.createTextNode(text.slice(end, match.index)));
      let node;
      if (match[1] !== undefined) {
        node = doc.createElement('code');
        node.textContent = match[1];
      } else if (match[2] !== undefined || match[3] !== undefined) {
        node = doc.createElement('strong');
        inline(node, match[2] ?? match[3], doc, depth + 1, allowLinks);
      } else if (match[4] !== undefined || match[5] !== undefined) {
        node = doc.createElement('em');
        inline(node, match[4] ?? match[5], doc, depth + 1, allowLinks);
      } else {
        const href = allowLinks ? safeUrl(match[7]) : null;
        if (href) {
          node = doc.createElement('a');
          node.setAttribute('href', href);
          node.setAttribute('target', '_blank');
          node.setAttribute('rel', 'noopener noreferrer');
          inline(node, match[6], doc, depth + 1, false);
        } else {
          node = doc.createTextNode(match[0]);
        }
      }
      parent.appendChild(node);
      end = match.index + match[0].length;
    }
    parent.appendChild(doc.createTextNode(text.slice(end)));
  }

  function fragment(raw, doc) {
    const result = doc.createDocumentFragment();
    let list = null;
    let fence = null;
    let buffer = [];
    function closeList() { list = null; }
    function codeBlock() {
      const pre = doc.createElement('pre');
      const code = doc.createElement('code');
      code.textContent = buffer.join('\n');
      pre.appendChild(code);
      result.appendChild(pre);
      buffer = [];
    }
    function block(tag, text, parent = result) {
      const element = doc.createElement(tag);
      inline(element, text, doc);
      parent.appendChild(element);
      return element;
    }
    for (const line of String(raw).replace(/\r\n?/g, '\n').split('\n')) {
      const marker = line.match(/^ {0,3}(`{3,}|~{3,})(.*)$/);
      if (fence) {
        if (marker && marker[1][0] === fence[0] &&
            marker[1].length >= fence.length && marker[2].trim() === '') {
          codeBlock();
          fence = null;
        } else {
          buffer.push(line);
        }
        continue;
      }
      if (marker) {
        closeList();
        fence = marker[1];
        continue;
      }
      if (/^(\*{3,}|-{3,}|_{3,})\s*$/.test(line)) {
        closeList();
        result.appendChild(doc.createElement('hr'));
        continue;
      }
      const heading = line.match(/^(#{1,3})\s+(.*)$/);
      if (heading) {
        closeList();
        block('h' + heading[1].length, heading[2]);
        continue;
      }
      if (line.startsWith('> ')) {
        closeList();
        block('blockquote', line.slice(2));
        continue;
      }
      const item = line.match(/^([-*+]|\d+\.)\s+(.*)$/);
      if (item) {
        const type = /\d/.test(item[1]) ? 'ol' : 'ul';
        if (!list || list.tagName.toLowerCase() !== type) {
          list = doc.createElement(type);
          result.appendChild(list);
        }
        block('li', item[2], list);
        continue;
      }
      closeList();
      if (!line.trim()) result.appendChild(doc.createElement('br'));
      else block('p', line);
    }
    if (fence) codeBlock();
    return result;
  }

  function render(root, raw) {
    const content = fragment(raw, root.ownerDocument);
    root.textContent = '';
    root.appendChild(content);
  }

  const api = Object.freeze({ render, fragment, safeUrl });
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  else global.JunkMarkdown = api;
})(typeof globalThis !== 'undefined' ? globalThis : window);
