use n_framework_core_cli_abstractions::Spinner;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) struct InquirerConsoleSpinner {
    pub(crate) message: RwLock<String>,
    pub(crate) finished: AtomicBool,
}

impl Spinner for InquirerConsoleSpinner {
    fn set_message(&self, message: &str) {
        if !self.finished.load(Ordering::SeqCst) {
            *self.message.write().unwrap() = message.to_string();
            println!("... {}", message);
        }
    }
    fn success(&self, message: &str) {
        if !self.finished.swap(true, Ordering::SeqCst) {
            println!("✔ {}", message);
        }
    }
    fn error(&self, message: &str) {
        if !self.finished.swap(true, Ordering::SeqCst) {
            println!("✖ {}", message);
        }
    }
    fn cancel(&self, message: &str) {
        if !self.finished.swap(true, Ordering::SeqCst) {
            println!("- {}", message);
        }
    }
    fn stop(&self, message: &str) {
        if !self.finished.swap(true, Ordering::SeqCst) {
            println!("{}", message);
        }
    }
    fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }
}
