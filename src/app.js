// ═══════════════════════════════════════════════════════════════════════
    // Junk — frontend script (v3.2.0)
    //
    // Features (v3.2.0):
    //   1. Custom shortcut  — hotkey-key-input captures keydown, calls set_hotkey()
    //   2. Position memory  — saves outer_position after every drag, restores on show
    //   2b. Always on top   — default ON, toggle in prefs, persisted in localStorage
    //   3. Font size        — slider 14–28px, saved to localStorage
    //   4. Dark mode        — light/auto/dark buttons, respects prefers-color-scheme
    //   5. Markdown preview — lightweight inline parser, toggled via ⌘M
    //   6. Export           — copies textarea content to clipboard from prefs
    //   7. Tabs             — one note per tab (notes.js), ⌘T new, ⌘⇧W close, ⌘1-9 jump
    //   8. Save as file     — ⌘S asks Rust for a native Save dialog (any extension)
    //   9. Auto-hide        — pref pushed to Rust, which owns the blur timer
    //
    // KNOWN PITFALLS:
    //   - Deep-commenting index.html to 2000+ lines causes 'Can\'t find variable:
    //     windowEl' in WebKit/WKWebView (JavaScriptCore parses large ES modules
    //     differently than Node.js). Keep the file under ~1700 lines.
    //   - alwaysOnTop in tauri.conf.json only sets INITIAL state. Runtime toggle
    //     requires set_always_on_top() IPC — tauri.conf is read once at startup.
    //   - box-shadow is clipped by masksToBounds=YES (needed for rounded corners).
    //     Use shadow:true in tauri.conf.json for the ambient drop shadow instead.
    //
    // IPC bridge (v2.8.0+):
    //   getInvoke() uses __TAURI_INTERNALS__.invoke (always available) with
    //   __TAURI__.core.invoke as fallback. withGlobalTauri:true is required
    //   for event listening (window.__TAURI__.event.listen).
    // ═══════════════════════════════════════════════════════════════════════

    // ── IPC helpers ─────────────────────────────────────────────────────────

    function getInvoke() {
      return (
        window.__TAURI_INTERNALS__?.invoke ??
        window.__TAURI__?.core?.invoke ??
        null
      );
    }

    async function ipc(command, args = {}) {
      const invoke = getInvoke();
      if (!invoke) {
        console.error(`[ipc] bridge not available — cannot call "${command}"`);
        return undefined;
      }
      try {
        return await invoke(command, args);
      } catch (err) {
        console.error(`[ipc] "${command}" failed:`, err);
        throw err;
      }
    }

    // ── DOM refs ─────────────────────────────────────────────────────────────
    const editor          = document.getElementById('editor');
    const mdPreview       = document.getElementById('md-preview');
    const saveStatus      = document.getElementById('save-status');
    const windowEl        = document.getElementById('window');
    const editorDim       = document.getElementById('editor-dim');
    const prefsPanel      = document.getElementById('prefs-panel');
    const prefsCloseBtn   = document.getElementById('prefs-close');
    const footerPrefsBtn  = document.getElementById('footer-prefs-btn');
    const toggleAlwaysOnTop = document.getElementById('toggle-always-on-top');
    const toggleDimBlur   = document.getElementById('toggle-dim-blur');
    const toggleLaunch    = document.getElementById('toggle-launch');
    const toggleAutoUpdate= document.getElementById('toggle-autoupdate');
    const toggleMarkdown  = document.getElementById('toggle-markdown');
    const checkUpdateBtn  = document.getElementById('check-update-btn');
    const updateStatus    = document.getElementById('update-status');
    const currentVersionEl= document.getElementById('current-version');
    const btnMd           = document.getElementById('btn-md');
    const btnCopy         = document.getElementById('btn-copy');
    const btnExport       = document.getElementById('btn-export');
    const btnSave         = document.getElementById('btn-save');
    const btnSavePrefs    = document.getElementById('btn-save-prefs');
    const btnNewTab       = document.getElementById('btn-new-tab');
    const tabsEl          = document.getElementById('tabs');
    const toggleAutoHide  = document.getElementById('toggle-auto-hide');
    const autoHideSlider  = document.getElementById('auto-hide-slider');
    const autoHideLabel   = document.getElementById('auto-hide-label');
    const autoHideRow     = document.getElementById('auto-hide-delay-row');
    const hotkeyInput     = document.getElementById('hotkey-key-input');
    const hotkeyModifier  = document.getElementById('hotkey-modifier');
    const hotkeyStatus    = document.getElementById('hotkey-status');
    const fontSizeSlider  = document.getElementById('font-size-slider');
    const fontSizeLabel   = document.getElementById('font-size-label');
    const themeBtns       = document.querySelectorAll('.theme-btn');

    // ── localStorage keys ────────────────────────────────────────────────────
    const KEY_CONTENT     = 'junk-content';
    const KEY_AUTO_UPDATE = 'junk-auto-update';
    const KEY_FONT_SIZE   = 'junk-font-size';
    const KEY_THEME       = 'junk-theme';       // 'light' | 'auto' | 'dark'
    const KEY_MD_MODE     = 'junk-md-mode';     // 'true' | 'false'
    const KEY_HOTKEY      = 'junk-hotkey';      // e.g. 'KeyJ'
    const KEY_WIN_POS     = 'junk-win-pos';     // JSON {x,y}
    const KEY_WIN_SIZE    = 'junk-win-size';    // JSON {w,h}
    const KEY_ALWAYS_TOP  = 'junk-always-top';  // 'true' | 'false'
    const KEY_DIM_BLUR    = 'junk-dim-blur';    // 'true' | 'false'
    const KEY_NOTES       = 'junk-notes-v1';    // JSON snapshot from notes.js (all tabs)
    const KEY_NOTES_BACKUP = 'junk-notes-backup'; // last unreadable snapshot, kept for recovery
    const KEY_AUTO_HIDE   = 'junk-auto-hide';   // 'true' | 'false' (false = always float)
    const KEY_AUTO_HIDE_SECS = 'junk-auto-hide-seconds'; // 3..120, default 10

    // Platform label for tooltips: ⌘ on macOS, Ctrl+ elsewhere.
    const IS_MAC = /Mac|iPhone|iPad/.test(navigator.platform || navigator.userAgent);
    const MOD = IS_MAC ? '⌘' : 'Ctrl+';
    const MAX_TABS = 30;

    // ── Content persistence (tabs) ───────────────────────────────────────────
    // All notes live in ONE localStorage entry (KEY_NOTES) written atomically,
    // so a crash can never leave tabs half-saved. notes.js owns the data rules
    // (and is unit-tested); this file only wires it to the DOM.
    let notes = window.JunkNotes.initial();

    function persistNotes() {
      try {
        localStorage.setItem(KEY_NOTES, JSON.stringify(notes));
        return true;
      } catch (err) {
        // Quota exceeded or storage disabled. Say so instead of pretending.
        console.error('[notes] persist failed:', err);
        flashStatus('not saved', true);
        return false;
      }
    }

    function loadContent() {
      // Migration: pre-3.2 versions stored a single note in 'junk-content'.
      // It becomes the first tab; the old key is left untouched as a backup.
      const legacy = localStorage.getItem(KEY_CONTENT) || '';
      const raw = localStorage.getItem(KEY_NOTES);
      try {
        notes = window.JunkNotes.decode(raw, legacy);
      } catch (err) {
        console.error('[notes] snapshot unreadable, starting fresh:', err);
        try { localStorage.setItem(KEY_NOTES_BACKUP, raw || ''); } catch (_) { /* ignore */ }
        notes = window.JunkNotes.initial(legacy);
      }
      showActiveTab({ focus: false });
    }

    /** Show a short message in the footer status label (autosave, copy, save…). */
    function flashStatus(text, isError = false) {
      saveStatus.textContent = text;
      saveStatus.classList.toggle('error', isError);
      saveStatus.classList.remove('saving');
      void saveStatus.offsetWidth;   // restart the CSS fade animation
      saveStatus.classList.add('saving');
    }

    let saveTimer = null;
    function saveContent() {
      saveTimer = null;
      window.JunkNotes.update(notes, editor.value);
      if (persistNotes()) flashStatus('saved');
      updateActiveTabTitle();
      if (mdMode) renderMarkdown();
    }
    function scheduleSave() {
      if (saveTimer !== null) clearTimeout(saveTimer);
      saveTimer = setTimeout(saveContent, 300);
    }
    /** Write any pending keystrokes now (before switching tabs, saving, hiding). */
    function flushSave() {
      if (saveTimer !== null) {
        clearTimeout(saveTimer);
        saveContent();
      }
    }

    // ── Tabs ─────────────────────────────────────────────────────────────────
    // Tab chips are built with createElement/textContent only — note text is
    // user data and must never be parsed as HTML.
    function renderTabs() {
      tabsEl.replaceChildren();
      notes.tabs.forEach((tab, i) => {
        const el = document.createElement('div');
        el.className = 'tab' + (tab.id === notes.activeId ? ' active' : '');
        el.dataset.id = tab.id;
        el.setAttribute('role', 'tab');
        el.setAttribute('aria-selected', String(tab.id === notes.activeId));
        el.tabIndex = 0;

        const title = document.createElement('span');
        title.className = 'tab-title';
        title.textContent = window.JunkNotes.title(tab, i);

        const close = document.createElement('button');
        close.className = 'tab-close';
        close.type = 'button';
        close.tabIndex = -1;
        close.setAttribute('aria-label', 'Close tab');
        close.title = `Close tab (${MOD}${IS_MAC ? '⇧' : 'Shift+'}W)`;
        close.innerHTML = '<svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>';

        el.append(title, close);
        tabsEl.appendChild(el);
      });
      tabsEl.querySelector('.tab.active')?.scrollIntoView({ block: 'nearest', inline: 'nearest' });
    }

    /** Cheap update while typing: only the active chip's label changes. */
    function updateActiveTabTitle() {
      const i = notes.tabs.findIndex(t => t.id === notes.activeId);
      const el = tabsEl.querySelector('.tab.active .tab-title');
      if (el && i >= 0) el.textContent = window.JunkNotes.title(notes.tabs[i], i);
    }

    function showActiveTab({ focus = true } = {}) {
      editor.value = window.JunkNotes.active(notes).text;
      renderTabs();
      if (mdMode) renderMarkdown();
      else if (focus) focusEditor();
    }

    function switchTab(id) {
      if (id === notes.activeId) return;
      flushSave();
      window.JunkNotes.select(notes, id);
      persistNotes();
      showActiveTab();
    }
    function switchTabBy(delta) {
      const i = notes.tabs.findIndex(t => t.id === notes.activeId);
      const n = notes.tabs.length;
      switchTab(notes.tabs[(i + delta + n) % n].id);
    }
    function newTab() {
      if (prefsOpen) closePrefs();
      if (notes.tabs.length >= MAX_TABS) { flashStatus('tab limit', true); return; }
      flushSave();
      window.JunkNotes.add(notes);
      persistNotes();
      showActiveTab();
    }
    function closeTab(id = notes.activeId) {
      flushSave();
      const hadText = !!notes.tabs.find(t => t.id === id)?.text;
      window.JunkNotes.close(notes, id);
      persistNotes();
      showActiveTab();
      if (hadText) flashStatus(`closed · ${MOD}${IS_MAC ? '⇧' : 'Shift+'}T reopens`);
    }
    function reopenTab() {
      if (!notes.closed.length) { flashStatus('nothing to reopen'); return; }
      flushSave();
      window.JunkNotes.reopen(notes);
      persistNotes();
      showActiveTab();
    }

    tabsEl.addEventListener('click', (e) => {
      const tabEl = e.target.closest('.tab');
      if (!tabEl) return;
      if (e.target.closest('.tab-close')) closeTab(tabEl.dataset.id);
      else switchTab(tabEl.dataset.id);
    });
    // Middle-click closes a tab (browser convention).
    tabsEl.addEventListener('auxclick', (e) => {
      const tabEl = e.target.closest('.tab');
      if (e.button === 1 && tabEl) { e.preventDefault(); closeTab(tabEl.dataset.id); }
    });
    tabsEl.addEventListener('keydown', (e) => {
      const tabEl = e.target.closest('.tab');
      if (tabEl && (e.key === 'Enter' || e.key === ' ')) {
        e.preventDefault();
        switchTab(tabEl.dataset.id);
      }
    });

    // ── Save as file ─────────────────────────────────────────────────────────
    // The WebView cannot write files, and Blob downloads are unreliable inside
    // Tauri's webview. Rust shows the native Save dialog and writes the file;
    // the user picks the name and extension (.txt, .md, .log, anything).
    async function saveToFile() {
      flushSave();
      const tab = window.JunkNotes.active(notes);
      if (!tab.text) { flashStatus('nothing to save'); return; }
      try {
        const path = await ipc('save_text_file', {
          contents: tab.text,
          suggestedName: window.JunkNotes.filename(tab, mdMode ? 'md' : 'txt'),
        });
        if (path) flashStatus('saved to file');
      } catch (err) {
        console.error('[save] failed:', err);
        flashStatus('save failed', true);
      }
      setTimeout(focusEditor, 20);
    }

    // ── Focus ────────────────────────────────────────────────────────────────
    function focusEditor() {
      if (!mdMode) {
        editor.focus({ preventScroll: false });
        const len = editor.value.length;
        editor.setSelectionRange(len, len);
      }
    }

    // ── Fly-in ───────────────────────────────────────────────────────────────
    function triggerFlyIn() {
      requestAnimationFrame(() => {
        windowEl.style.animationName = 'none';
        void windowEl.offsetWidth;
        windowEl.style.animationName = '';
      });
    }

    // ── Hide window ──────────────────────────────────────────────────────────
    async function hideWindow() {
      await ipc('hide_window');
    }

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 1: Custom shortcut
    // ════════════════════════════════════════════════════════════════════════
    //
    // The user clicks the hotkey input field, then presses any key.
    // We capture the KeyboardEvent.code (e.g. "KeyK") and send it to Rust
    // via invoke('set_hotkey', { key: "KeyK" }). Rust unregisters the old
    // shortcut and registers the new one. No restart needed.
    //
    // Stored in localStorage so on next launch we display the saved key.
    // (The Rust side always registers KeyJ by default — JS calls set_hotkey
    // on startup if a custom key is saved.)
    //
    // WHY KeyboardEvent.code and not .key?
    //   .key gives "j" (layout-dependent, affected by CapsLock/shift).
    //   .code gives "KeyJ" — layout-independent physical key name. This is
    //   what tauri_plugin_global_shortcut::Code uses. Always use .code.

    let hotkeyCapturing = false;
    let currentHotkey = localStorage.getItem(KEY_HOTKEY) || 'KeyJ';

    function displayHotkey(code) {
      // Map Code name to a human-readable display label
      const label = code
        .replace(/^Key/, '')
        .replace(/^Digit/, '')
        .replace('Space', '␣');
      hotkeyInput.value = label;
    }

    function initHotkeyDisplay() {
      // Show the platform modifier
      const isMac = navigator.platform.toUpperCase().includes('MAC') ||
                    navigator.userAgent.includes('Macintosh');
      hotkeyModifier.textContent = isMac ? '⌘' : 'Ctrl';
      displayHotkey(currentHotkey);
    }

    async function applyHotkey(code) {
      try {
        await ipc('set_hotkey', { key: code });
        currentHotkey = code;
        localStorage.setItem(KEY_HOTKEY, code);
        displayHotkey(code);
        setHotkeyStatus('Shortcut saved', 'ok');
        setTimeout(() => setHotkeyStatus('', ''), 2000);
      } catch (err) {
        setHotkeyStatus('Key not available', 'error');
        setTimeout(() => setHotkeyStatus('', ''), 3000);
        console.error('[hotkey] set_hotkey failed:', err);
      }
    }

    function setHotkeyStatus(msg, cls) {
      hotkeyStatus.textContent = msg;
      hotkeyStatus.className = cls;
    }

    // Click the input to start capturing; Esc to cancel
    hotkeyInput.addEventListener('click', () => {
      hotkeyCapturing = true;
      hotkeyInput.style.borderColor = 'var(--accent)';
      hotkeyInput.value = '…';
      setHotkeyStatus('Press any key', '');
    });

    hotkeyInput.addEventListener('keydown', async (e) => {
      if (!hotkeyCapturing) return;
      e.preventDefault(); e.stopPropagation();

      if (e.code === 'Escape') {
        // Cancel capture, restore previous value
        hotkeyCapturing = false;
        hotkeyInput.style.borderColor = '';
        displayHotkey(currentHotkey);
        setHotkeyStatus('', '');
        return;
      }

      // Ignore bare modifier keys
      if (['MetaLeft','MetaRight','ControlLeft','ControlRight',
           'ShiftLeft','ShiftRight','AltLeft','AltRight'].includes(e.code)) {
        return;
      }

      hotkeyCapturing = false;
      hotkeyInput.style.borderColor = '';
      await applyHotkey(e.code);
    });

    // Restore saved hotkey on startup
    async function restoreHotkey() {
      const saved = localStorage.getItem(KEY_HOTKEY);
      if (saved && saved !== 'KeyJ') {
        // Re-register the saved shortcut in Rust without changing localStorage
        try {
          await ipc('set_hotkey', { key: saved });
          currentHotkey = saved;
        } catch (err) {
          // If registration fails (key conflict etc.) fall back to KeyJ
          localStorage.removeItem(KEY_HOTKEY);
          currentHotkey = 'KeyJ';
        }
      }
      displayHotkey(currentHotkey);
    }

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 2: Window position memory
    // ════════════════════════════════════════════════════════════════════════
    //
    // After every drag, we save the window's outer_position to localStorage.
    // On startup (and every time the window is shown via tauri://focus), we
    // call set_window_position() to restore it.
    //
    // WHY physical coordinates?
    //   Tauri's outer_position() returns PhysicalPosition — pixel-exact screen
    //   coords that don't change with display scaling. Logical coords would
    //   give wrong results on Retina/HiDPI displays when scale factors change.
    //
    // OFF-SCREEN GUARD:
    //   If the saved position is outside the current screen geometry (user
    //   moved to a smaller monitor), we skip restoration and let the window
    //   stay centered. We detect "off-screen" as: x < -100 || y < -100 or
    //   x > screen.width + 200 || y > screen.height + 200.
    //   (We allow 200px overhang because a partially-visible window is fine.)

    let isDragging = false;

    async function saveWindowGeometry() {
      try {
        const pos = await ipc('get_window_position');
        if (pos) {
          localStorage.setItem(KEY_WIN_POS, JSON.stringify({ x: pos.x, y: pos.y }));
        }
        // Also save the window's logical size via the outer dimensions
        const size = await ipc('get_window_size');
        if (size) {
          localStorage.setItem(KEY_WIN_SIZE, JSON.stringify({ w: size.width, h: size.height }));
        }
      } catch (err) {
        // Non-fatal — geometry just won't be saved this time
        console.warn('[position] saveWindowGeometry failed:', err);
      }
    }

    // Keep legacy alias for tauri://focus path
    const saveWindowPosition = saveWindowGeometry;

    async function restoreWindowGeometry() {
      // Restore size first, then position (so the window lands in the right spot)
      const rawSize = localStorage.getItem(KEY_WIN_SIZE);
      if (rawSize) {
        try {
          const { w, h } = JSON.parse(rawSize);
          if (w >= 300 && h >= 200 && w <= 3000 && h <= 2000) {
            await ipc('set_window_size', { width: w, height: h });
          }
        } catch (err) {
          console.warn('[position] restoreWindowSize failed:', err);
        }
      }

      const rawPos = localStorage.getItem(KEY_WIN_POS);
      if (!rawPos) return;

      try {
        const { x, y } = JSON.parse(rawPos);

        // Off-screen guard — skip if way outside current screen bounds
        const sw = window.screen.width  + 200;
        const sh = window.screen.height + 200;
        if (x < -100 || y < -100 || x > sw || y > sh) {
          console.warn('[position] saved position off-screen, skipping restore');
          return;
        }

        await ipc('set_window_position', { x, y });
      } catch (err) {
        console.warn('[position] restoreWindowPosition failed:', err);
      }
    }

    const restoreWindowPosition = restoreWindowGeometry;

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 2b: Always on top
    // ════════════════════════════════════════════════════════════════════════
    //
    // Junk’s core UX is floating above all windows — it’s a scratchpad you
    // summon instantly and it should stay visible. This was the original default
    // (alwaysOnTop: true in tauri.conf.json) but was inadvertently set to false
    // during the v3.0.2 visual rework.
    //
    // We restore it here: default = ON, persisted in localStorage so the user
    // can turn it off if they prefer. Synced to Rust via set_always_on_top().
    //
    // WHY Rust IPC instead of just tauri.conf.json?
    //   tauri.conf sets the initial state only. To toggle at runtime we must
    //   call window.set_always_on_top() via Tauri IPC.

    async function applyAlwaysOnTop(on) {
      toggleAlwaysOnTop.checked = on;
      localStorage.setItem(KEY_ALWAYS_TOP, String(on));
      try {
        await ipc('set_always_on_top', { alwaysOnTop: on });
      } catch (err) {
        console.warn('[always-on-top] IPC failed:', err);
      }
    }

    function loadAlwaysOnTop() {
      // Default: true (on). Only false if user explicitly disabled it.
      const saved = localStorage.getItem(KEY_ALWAYS_TOP);
      const on = saved !== 'false'; // undefined/null/true all map to ON
      applyAlwaysOnTop(on);
    }

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 2c: Dim when unfocused
    // ════════════════════════════════════════════════════════════════════════
    //
    // When the user clicks into another app, Junk fades to 70% opacity.
    // On re-focus it snaps back to 100%. The transition (0.18s ease) is
    // defined in CSS on .window so both directions animate smoothly.
    //
    // Implementation:
    //   - junk://show         → fly-in + restorePos + alwaysOnTop (hide→show only)
    //   - junk://blur         → setNativeOpacity(0.5)  (if pref is ON)
    //   - junk://focus-change → setNativeOpacity(1.0)  (re-focus, no fly-in)
    //   - tauri://focus       → setNativeOpacity(1.0)  (safety net only)
    //   On macOS/Windows: native IPC for opacity. On Linux: CSS inline opacity on #window.
    //
    // WHY junk://blur instead of tauri://blur?
    //   tauri://blur is a WebKit-level event and does not fire reliably on
    //   macOS for always-on-top windows. We emit junk://blur from Rust via
    //   WindowEvent::Focused(false) → NSWindowDelegate windowDidResignKey,
    //   which fires unconditionally regardless of window level.
    //
    // The class is always cleared on focus-change so toggling the pref OFF
    // while blurred doesn't leave it stuck at 50%.

    // setNativeOpacity: sets opacity on the entire native window compositor
    // surface. Platform behaviour:
    //   macOS:   NSWindow.alphaValue via objc2 IPC — dims the whole native
    //            surface including the vibrancy layer (CSS opacity can't).
    //   Windows: SetLayeredWindowAttributes via Win32 IPC — whole-window alpha.
    //   Linux:   No universal cross-compositor API; we toggle the CSS class
    //            .window--blurred on #window instead. This works because Linux
    //            Junk windows have no native backdrop-blur layer — the WebView
    //            background IS the window, so CSS opacity dims everything.
    //
    // We detect Linux by the absence of Macintosh + Win32 in the UA string.
    // This is reliable inside Tauri's WebView since the UA is set by the OS.
    const _isLinux = !navigator.userAgent.includes('Macintosh') &&
                     !navigator.userAgent.includes('Win32') &&
                     !navigator.userAgent.includes('Windows');
    const _windowEl = document.getElementById('window');

    function setNativeOpacity(value) {
      if (_isLinux) {
        // CSS opacity path for Linux — no native IPC needed.
        if (_windowEl) {
          _windowEl.style.opacity = value < 1.0 ? String(value) : '';
        }
        return;
      }
      // macOS / Windows: native IPC handles it.
      ipc('set_window_opacity', { opacity: value }).catch(() => {});
    }

    function applyDimBlur(on) {
      toggleDimBlur.checked = on;
      localStorage.setItem(KEY_DIM_BLUR, String(on));
      // If turning OFF while blurred, restore native opacity immediately
      if (!on) setNativeOpacity(1.0);
    }

    function loadDimBlur() {
      const saved = localStorage.getItem(KEY_DIM_BLUR);
      const on = saved !== 'false'; // default ON
      toggleDimBlur.checked = on;
      // Window starts focused — ensure full opacity
      setNativeOpacity(1.0);
    }

    // ════════════════════════════════════════════════════════════════════════
    // Auto-hide when unfocused
    // ════════════════════════════════════════════════════════════════════════
    //
    // The timer itself runs in Rust (auto_hide.rs): WebView timers can be
    // throttled once the window is in the background, which is exactly when
    // this one must fire. JS only stores the preference and pushes it down.
    // 0 seconds means "always float".
    function formatSeconds(n) {
      if (n < 60) return `${n}s`;
      const m = Math.floor(n / 60), r = n % 60;
      return r ? `${m}m ${r}s` : `${m}m`;
    }

    function readAutoHideSeconds() {
      const n = parseInt(localStorage.getItem(KEY_AUTO_HIDE_SECS), 10);
      return n >= 3 && n <= 120 ? n : 10;
    }

    function syncAutoHide() {
      const on = toggleAutoHide.checked;
      const secs = parseInt(autoHideSlider.value, 10) || 10;
      autoHideLabel.textContent = formatSeconds(secs);
      autoHideRow.classList.toggle('is-disabled', !on);
      autoHideSlider.disabled = !on;
      ipc('set_auto_hide', { seconds: on ? secs : 0 }).catch(() => {});
    }

    function loadAutoHide() {
      toggleAutoHide.checked = localStorage.getItem(KEY_AUTO_HIDE) !== 'false'; // default ON
      autoHideSlider.value = String(readAutoHideSeconds());
      syncAutoHide();
    }

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 3: Font size preference
    // ════════════════════════════════════════════════════════════════════════
    //
    // We drive font size via a CSS custom property --font-size on :root.
    // This means both #editor and #md-preview pick it up automatically —
    // no JS needed to update both elements separately.

    function applyFontSize(px) {
      document.documentElement.style.setProperty('--font-size', `${px}px`);
      fontSizeLabel.textContent = `${px}px`;
      fontSizeSlider.value = px;
    }

    function loadFontSize() {
      const saved = parseInt(localStorage.getItem(KEY_FONT_SIZE), 10);
      const size = (saved >= 14 && saved <= 28) ? saved : 22;
      applyFontSize(size);
    }

    fontSizeSlider.addEventListener('input', () => {
      const px = parseInt(fontSizeSlider.value, 10);
      applyFontSize(px);
      localStorage.setItem(KEY_FONT_SIZE, String(px));
    });

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 4: Dark mode
    // ════════════════════════════════════════════════════════════════════════
    //
    // Three modes: 'light', 'auto', 'dark'.
    //   auto — reads window.matchMedia('(prefers-color-scheme: dark)')
    //          and listens for changes in real time.
    //   light / dark — hard overrides, ignore system preference.
    //
    // Applied by setting data-theme="dark" or removing it on :root.
    // All colours are driven by CSS custom properties so the switch is
    // instant and transition-smooth.
    //
    // WHY data-theme attribute instead of a class?
    //   A data attribute on :root is cleaner for CSS selectors and is the
    //   conventional approach for multi-theme apps.

    let themeMode = 'auto'; // persisted preference
    let darkModeMediaQuery = null;

    function applyTheme(mode) {
      themeMode = mode;
      const root = document.documentElement;

      // Update button active states
      themeBtns.forEach(btn => {
        btn.classList.toggle('active', btn.dataset.theme === mode);
      });

      if (mode === 'dark') {
        root.setAttribute('data-theme', 'dark');
        if (darkModeMediaQuery) {
          darkModeMediaQuery.removeEventListener('change', onSystemThemeChange);
          darkModeMediaQuery = null;
        }
      } else if (mode === 'light') {
        root.removeAttribute('data-theme');
        if (darkModeMediaQuery) {
          darkModeMediaQuery.removeEventListener('change', onSystemThemeChange);
          darkModeMediaQuery = null;
        }
      } else {
        // 'auto' — follow system
        followSystemTheme();
      }

      localStorage.setItem(KEY_THEME, mode);
    }

    function onSystemThemeChange(e) {
      document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : '');
      if (!e.matches) document.documentElement.removeAttribute('data-theme');
    }

    function followSystemTheme() {
      if (!darkModeMediaQuery) {
        darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
        darkModeMediaQuery.addEventListener('change', onSystemThemeChange);
      }
      // Apply current system preference immediately
      if (darkModeMediaQuery.matches) {
        document.documentElement.setAttribute('data-theme', 'dark');
      } else {
        document.documentElement.removeAttribute('data-theme');
      }
    }

    function loadTheme() {
      const saved = localStorage.getItem(KEY_THEME) || 'auto';
      applyTheme(saved);
    }

    themeBtns.forEach(btn => {
      btn.addEventListener('click', () => applyTheme(btn.dataset.theme));
    });

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 5: Markdown preview
    // ════════════════════════════════════════════════════════════════════════
    //
    // A DOM-only Markdown renderer in markdown.js; note content is never HTML.
    // Handles the most common Markdown constructs used in scratchpad notes:
    //   h1/h2/h3, bold, italic, inline code, fenced code blocks,
    //   blockquotes, horizontal rules, unordered/ordered lists, links, br.
    //
    // Explicit safe link schemes only. This is a small scratchpad dialect,
    // not a complete CommonMark implementation.
    //
    // TOGGLING:
    //   When MD mode is on: textarea is hidden, mdPreview shows the rendered
    //   HTML. The underlying text in localStorage is always the raw Markdown —
    //   the preview is purely a read-time view.
    //
    //   ⌘M (macOS) / Ctrl+M (Win/Linux) is the keyboard shortcut.

    let mdMode = localStorage.getItem(KEY_MD_MODE) === 'true';

    function renderMarkdown() {
      window.JunkMarkdown.render(mdPreview, editor.value);
    }

    function setMdMode(on) {
      mdMode = on;
      localStorage.setItem(KEY_MD_MODE, String(on));
      toggleMarkdown.checked = on;
      btnMd.classList.toggle('active', on);

      if (on) {
        renderMarkdown();
        editor.style.display = 'none';
        mdPreview.style.display = 'block';
      } else {
        editor.style.display = '';
        mdPreview.style.display = 'none';
        focusEditor();
      }
    }

    function loadMdMode() {
      setMdMode(localStorage.getItem(KEY_MD_MODE) === 'true');
    }

    // ════════════════════════════════════════════════════════════════════════
    // FEATURE 6: Plain-text export (copy all to clipboard)
    // ════════════════════════════════════════════════════════════════════════
    //
    // Copies the raw textarea content to the clipboard.
    // Works from both the footer copy button and the prefs panel export button.
    //
    // WHY navigator.clipboard.writeText and not document.execCommand('copy')?
    //   execCommand is deprecated and unreliable. navigator.clipboard is the
    //   modern async API. It requires a secure context (HTTPS or localhost) —
    //   Tauri's custom-protocol satisfies this requirement.

    async function copyAllToClipboard(sourceBtn) {
      const text = editor.value;
      if (!text) return;

      try {
        await navigator.clipboard.writeText(text);

        // Feedback: preserve original innerHTML (may be SVG icon or text)
        const origHTML = sourceBtn.innerHTML;
        const origTitle = sourceBtn.title;
        const isSvgBtn = sourceBtn.classList.contains('icon-btn');

        if (isSvgBtn) {
          // For icon buttons: swap SVG for a checkmark icon + show save-status
          sourceBtn.innerHTML = `<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>`;
          sourceBtn.title = 'Saved!';
          // Also flash the save-status label
          flashStatus('copied');
        } else {
          // For text buttons (prefs export): swap text
          sourceBtn.textContent = 'Copied!';
          sourceBtn.classList.add('copied');
        }

        setTimeout(() => {
          sourceBtn.innerHTML = origHTML;
          sourceBtn.title = origTitle;
          sourceBtn.classList.remove('copied');
        }, 1600);
      } catch (err) {
        console.error('[export] clipboard.writeText failed:', err);
      }
    }

    // ── Preferences panel ────────────────────────────────────────────────────
    let prefsOpen = false;

    async function openPrefs() {
      if (prefsOpen) return;
      prefsOpen = true;
      windowEl.classList.add('prefs-open');
      await Promise.allSettled([
        loadLaunchAtLogin(),
        loadVersionDisplay(),
      ]);
      const autoUpdate = localStorage.getItem(KEY_AUTO_UPDATE);
      toggleAutoUpdate.checked = autoUpdate !== 'false';
      setUpdateStatus('', '');
    }

    function closePrefs() {
      if (!prefsOpen) return;
      prefsOpen = false;
      windowEl.classList.remove('prefs-open');
    }

    // ── Launch at login ──────────────────────────────────────────────────────
    async function loadLaunchAtLogin() {
      try {
        const prefs = await ipc('get_prefs');
        if (prefs && typeof prefs.launch_at_login === 'boolean') {
          toggleLaunch.checked = prefs.launch_at_login;
        }
      } catch (err) {
        console.warn('[prefs] get_prefs failed:', err);
      }
    }

    async function onLaunchAtLoginChange() {
      const enabled = toggleLaunch.checked;
      try {
        await ipc('set_launch_at_login', { enabled });
      } catch (err) {
        toggleLaunch.checked = !enabled;
        console.error('[prefs] set_launch_at_login failed:', err);
      }
    }

    // ── Version display ──────────────────────────────────────────────────────
    async function loadVersionDisplay() {
      try {
        const result = await ipc('check_for_update');
        if (result?.current) currentVersionEl.textContent = `v${result.current}`;
      } catch {
        currentVersionEl.textContent = 'v?';
      }
    }

    // ── Update check ─────────────────────────────────────────────────────────
    function setUpdateStatus(msg, cls) {
      updateStatus.textContent = '';
      updateStatus.className = '';
      if (msg) updateStatus.textContent = msg;
      if (cls) updateStatus.classList.add(cls);
    }

    async function checkForUpdate({ silent = false } = {}) {
      checkUpdateBtn.disabled = true;
      if (!silent) setUpdateStatus('Checking…', '');
      try {
        const result = await ipc('check_for_update');
        if (!result) { setUpdateStatus('Could not reach GitHub — are you online?', 'update-error'); return; }
        if (result.current) currentVersionEl.textContent = `v${result.current}`;
        if (result.up_to_date) {
          setUpdateStatus(`You're up to date (${result.current})`, 'up-to-date');
        } else {
          updateStatus.className = 'update-available';
          const link = document.createElement('a');
          link.href = result.url; link.target = '_blank'; link.rel = 'noopener noreferrer';
          link.textContent = `Update available: v${result.latest}`;
          link.style.cssText = 'color:var(--accent);font-weight:500;';
          updateStatus.appendChild(link);
        }
      } catch {
        setUpdateStatus('Could not check — are you online?', 'update-error');
      } finally {
        checkUpdateBtn.disabled = false;
      }
    }

    function maybeAutoCheckOnStartup() {
      if (localStorage.getItem(KEY_AUTO_UPDATE) === 'false') return;
      setTimeout(() => checkForUpdate({ silent: true }).catch(() => {}), 2000);
    }

    // ── Tauri event listeners ────────────────────────────────────────────────
    function setupTauriEvents() {
      const listenFn = window.__TAURI__?.event?.listen ?? null;
      if (!listenFn) {
        setTimeout(setupTauriEvents, 100);
        return;
      }
      // junk://show — emitted from Rust show_and_focus(), only on hide→show cycles.
      //
      // WHY not tauri://focus for the fly-in?
      //   tauri://focus fires on BOTH show() AND on re-focus (user clicks back
      //   into an already-visible window). Triggering triggerFlyIn() there caused
      //   the animation to replay every time focus returned — the "bug" where
      //   clicking back into Junk shows a weird double-display effect.
      //   junk://show fires only when transitioning from hidden → visible, so
      //   the fly-in plays exactly once per summon.
      listenFn('junk://show', async () => {
        // Opacity was already reset to 1.0 by Rust before show() — this is
        // a safety-net no-op, but cheap and harmless.
        setNativeOpacity(1.0);
        triggerFlyIn();
        await restoreWindowPosition();
        // Re-assert always-on-top on every show — macOS resets CGWindowLevel
        // on orderOut → makeKeyAndOrderFront (hide/show cycle).
        const alwaysOn = localStorage.getItem(KEY_ALWAYS_TOP) !== 'false';
        ipc('set_always_on_top', { alwaysOnTop: alwaysOn }).catch(() => {});
        setTimeout(focusEditor, 20);
      });

      // tauri://focus — fires on both show() and re-focus (click into window).
      // Only used as an opacity safety net now. Fly-in moved to junk://show.
      listenFn('tauri://focus', () => {
        setNativeOpacity(1.0);
      });

      // junk://blur — emitted from Rust WindowEvent::Focused(false).
      // Sets native NSWindow.alphaValue to 0.5 so the entire window surface
      // (including vibrancy layer) becomes see-through.
      listenFn('junk://blur', () => {
        const dimOn = localStorage.getItem(KEY_DIM_BLUR) !== 'false';
        if (dimOn) setNativeOpacity(0.5);
      });

      // junk://focus-change — emitted from Rust WindowEvent::Focused(true).
      // Restore full opacity whenever focus returns.
      listenFn('junk://focus-change', () => {
        setNativeOpacity(1.0);
      });
      listenFn('open-prefs', () => openPrefs());
    }

    // ── Window drag + position save ──────────────────────────────────────────
    (function registerDragListener() {
      const INTERACTIVE = [
        'textarea','input','button','select','a','label',
        '[contenteditable]','.prefs-panel','.editor-dim','.tab',
      ].join(', ');

      if (!windowEl) return;

      windowEl.addEventListener('mousedown', (e) => {
        if (e.button !== 0) return;
        let node = e.target;
        while (node && node !== windowEl) {
          if (node.matches?.(INTERACTIVE)) return;
          node = node.parentElement;
        }
        isDragging = true;
        e.preventDefault();
        ipc('start_dragging').catch(err => console.warn('[drag]', err));
      });

      // Save position after drag ends
      window.addEventListener('mouseup', () => {
        if (isDragging) {
          isDragging = false;
          // Small delay — let the OS finish moving before we read position
          setTimeout(saveWindowPosition, 80);
        }
      });

      windowEl.addEventListener('mousemove', (e) => {
        let node = e.target;
        while (node && node !== windowEl) {
          if (node.matches?.(INTERACTIVE)) { windowEl.style.cursor = ''; return; }
          node = node.parentElement;
        }
        windowEl.style.cursor = 'grab';
      });

      windowEl.addEventListener('mouseleave', () => { windowEl.style.cursor = ''; });
    })();

    // Save size whenever the window is resized
    let resizeSaveTimer = null;
    window.addEventListener('resize', () => {
      clearTimeout(resizeSaveTimer);
      resizeSaveTimer = setTimeout(saveWindowGeometry, 300);
    });

    // ── Keyboard shortcuts ───────────────────────────────────────────────────
    document.addEventListener('keydown', (e) => {
      // Don't intercept while capturing a new hotkey
      if (hotkeyCapturing) return;

      if (e.key === 'Escape') {
        e.preventDefault();
        if (prefsOpen) closePrefs(); else hideWindow();
        return;
      }
      if (e.key === ',' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        if (prefsOpen) closePrefs(); else openPrefs();
        return;
      }
      // ⌘M / Ctrl+M — toggle Markdown preview
      if (e.key === 'm' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setMdMode(!mdMode);
        return;
      }

      // Tabs and save. Matched on e.code so Shift/layouts do not change the key.
      //   ⌘T new · ⌘⇧T reopen · ⌘⇧W close · ⌘S save · ⌘1-9 jump
      //   ⌘⇧[ / ⌘⇧] or Ctrl+Tab / Ctrl+Shift+Tab to cycle
      // (⌘W is not used: the default macOS menu maps it to "Close Window".)
      const mod = e.metaKey || e.ctrlKey;
      if (e.ctrlKey && e.code === 'Tab') {
        e.preventDefault();
        switchTabBy(e.shiftKey ? -1 : 1);
        return;
      }
      if (!mod || e.altKey) return;
      if (e.code === 'KeyT') { e.preventDefault(); e.shiftKey ? reopenTab() : newTab(); return; }
      if (e.code === 'KeyW' && e.shiftKey) { e.preventDefault(); closeTab(); return; }
      if (e.code === 'KeyS' && !e.shiftKey) { e.preventDefault(); saveToFile(); return; }
      if (e.shiftKey && e.code === 'BracketRight') { e.preventDefault(); switchTabBy(1); return; }
      if (e.shiftKey && e.code === 'BracketLeft')  { e.preventDefault(); switchTabBy(-1); return; }
      if (!e.shiftKey && /^Digit[1-9]$/.test(e.code)) {
        const tab = notes.tabs[Number(e.code.slice(5)) - 1];
        if (tab) { e.preventDefault(); switchTab(tab.id); }
      }
    });

    // ── Paste handler ────────────────────────────────────────────────────────
    document.addEventListener('paste', (e) => {
      if (document.activeElement === editor || prefsOpen || mdMode) return;
      e.preventDefault();
      const text = e.clipboardData?.getData('text/plain') ?? '';
      if (!text) return;
      const start = editor.selectionStart ?? editor.value.length;
      const end   = editor.selectionEnd   ?? editor.value.length;
      editor.value = editor.value.slice(0, start) + text + editor.value.slice(end);
      const newPos = start + text.length;
      editor.setSelectionRange(newPos, newPos);
      scheduleSave();
    });

    // ── Event wiring ─────────────────────────────────────────────────────────
    editor.addEventListener('input', () => {
      scheduleSave();
      if (mdMode) renderMarkdown();
    });

    footerPrefsBtn.addEventListener('click', () => {
      if (prefsOpen) closePrefs(); else openPrefs();
    });

    editorDim.addEventListener('click', closePrefs);
    prefsCloseBtn.addEventListener('click', closePrefs);
    toggleAlwaysOnTop.addEventListener('change', () => applyAlwaysOnTop(toggleAlwaysOnTop.checked));
    toggleDimBlur.addEventListener('change', () => applyDimBlur(toggleDimBlur.checked));
    toggleLaunch.addEventListener('change', onLaunchAtLoginChange);

    toggleAutoUpdate.addEventListener('change', () => {
      localStorage.setItem(KEY_AUTO_UPDATE, String(toggleAutoUpdate.checked));
    });

    toggleMarkdown.addEventListener('change', () => setMdMode(toggleMarkdown.checked));

    btnMd.addEventListener('click', () => setMdMode(!mdMode));
    btnCopy.addEventListener('click', () => copyAllToClipboard(btnCopy));
    btnExport.addEventListener('click', () => copyAllToClipboard(btnExport));
    btnSave.addEventListener('click', saveToFile);
    btnSavePrefs.addEventListener('click', () => { closePrefs(); saveToFile(); });
    btnNewTab.addEventListener('click', newTab);
    toggleAutoHide.addEventListener('change', () => {
      localStorage.setItem(KEY_AUTO_HIDE, String(toggleAutoHide.checked));
      syncAutoHide();
    });
    // 'input' updates the label live; 'change' (on release) pushes to Rust once.
    autoHideSlider.addEventListener('input', () => {
      autoHideLabel.textContent = formatSeconds(parseInt(autoHideSlider.value, 10));
    });
    autoHideSlider.addEventListener('change', () => {
      localStorage.setItem(KEY_AUTO_HIDE_SECS, autoHideSlider.value);
      syncAutoHide();
    });
    btnNewTab.title = `New tab (${MOD}T)`;
    btnSave.title = `Save as file (${MOD}S)`;
    checkUpdateBtn.addEventListener('click', () => checkForUpdate({ silent: false }));

    // ── Startup sequence ─────────────────────────────────────────────────────
    loadContent();
    loadFontSize();
    loadTheme();
    loadMdMode();
    loadAlwaysOnTop();
    loadDimBlur();
    loadAutoHide();
    initHotkeyDisplay();
    setupTauriEvents();
    maybeAutoCheckOnStartup();
    restoreWindowPosition();

    // Restore custom hotkey (re-registers in Rust if user had changed it)
    restoreHotkey().catch(() => {});

    console.log('[junk] v3.2.0 loaded. IPC bridge:', getInvoke() ? 'OK' : 'NOT FOUND');
