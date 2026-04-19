use std::sync::Mutex;
use n_framework_core_cli_abstractions::Spinner;

pub struct CliclackSpinner {
    pub(crate) inner: Mutex<Option<cliclack::ProgressBar>>,
}

impl Spinner for CliclackSpinner {
    fn set_message(&self, _message: &str) {
        // Not directly supported on cliclack ProgressBar
    }

    fn success(&self, message: &str) {
        let mut inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(pb) = inner.take() {
            pb.stop(message);
        }
    }

    fn error(&self, message: &str) {
        let mut inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(pb) = inner.take() {
            pb.error(message);
        }
    }

    fn cancel(&self, message: &str) {
        let mut inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(pb) = inner.take() {
            pb.cancel(message);
        }
    }

    fn stop(&self, message: &str) {
        let mut inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(pb) = inner.take() {
            pb.stop(message);
        }
    }

    fn is_finished(&self) -> bool {
        let inner = match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        inner.is_none()
    }
}
