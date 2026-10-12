//! Native blur timer policy — independent of WebView timer throttling.
//! Generation tickets invalidate old sleeps on focus, hide/show, settings
//! changes and native dialogs. The actual hide runs on the UI thread and
//! rechecks visibility/focus there, avoiding the focus-at-deadline race.
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct AutoHide {
    pub seconds: AtomicU64,
    pub focused: AtomicBool,
    pub dialog_open: AtomicBool,
    generation: AtomicU64,
}
impl Default for AutoHide {
    fn default() -> Self {
        Self { seconds: AtomicU64::new(10), focused: AtomicBool::new(false),
            dialog_open: AtomicBool::new(false), generation: AtomicU64::new(0) }
    }
}
impl AutoHide {
    pub fn cancel(&self) { self.generation.fetch_add(1, Ordering::SeqCst); }
    pub fn ticket(&self) -> u64 {
        self.generation.fetch_add(1, Ordering::SeqCst) + 1
    }
    pub fn can_hide(&self, ticket: u64) -> bool {
        self.generation.load(Ordering::SeqCst) == ticket &&
        self.seconds.load(Ordering::SeqCst) > 0 &&
        !self.focused.load(Ordering::SeqCst) &&
        !self.dialog_open.load(Ordering::SeqCst)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn default_is_ten_seconds() {
        assert_eq!(AutoHide::default().seconds.load(Ordering::SeqCst), 10);
    }
    #[test] fn focus_cancels_expired_ticket() {
        let p = AutoHide::default(); let t = p.ticket();
        assert!(p.can_hide(t)); p.focused.store(true, Ordering::SeqCst);
        p.cancel(); assert!(!p.can_hide(t));
    }
    #[test] fn newest_blur_wins() {
        let p = AutoHide::default(); let old = p.ticket(); let new = p.ticket();
        assert!(!p.can_hide(old)); assert!(p.can_hide(new));
    }
    #[test] fn always_float_and_dialogs_cannot_hide() {
        let p = AutoHide::default(); let t = p.ticket();
        p.seconds.store(0, Ordering::SeqCst); assert!(!p.can_hide(t));
        p.seconds.store(10, Ordering::SeqCst);
        p.dialog_open.store(true, Ordering::SeqCst); assert!(!p.can_hide(t));
    }
    #[test] fn show_hide_and_settings_cancel_old_timers() {
        let p = AutoHide::default(); let t = p.ticket(); p.cancel();
        assert!(!p.can_hide(t));
    }
}
