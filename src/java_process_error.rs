use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::sync::Arc;
use std::time::SystemTime;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct JavaProcessError {
    pub(crate) exit_code: Option<i32>,
    pub(crate) timestamp: SystemTime,
    pub(crate) source: Arc<dyn StdError + Send + Sync + 'static>,
}

impl StdError for JavaProcessError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.deref())
    }
}

impl Display for JavaProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Java process failed at {:?} with exit code: {}",
            self.timestamp,
            self.exit_code
                .map_or("unknown".to_string(), |c| c.to_string())
        )
    }
}