use crossterm::{
    ExecutableCommand,
    cursor::Show,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::{Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct TerminalGuard {}

impl TerminalGuard {
    pub fn new() -> Self {
        let _ = stdout().execute(crossterm::terminal::EnterAlternateScreen);

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            // Signal the app to stop
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        TerminalGuard {}
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = stdout();
        let _ = disable_raw_mode();
        let _ = out.execute(LeaveAlternateScreen);
        let _ = out.execute(Show);
        let _ = out.flush();
    }
}
