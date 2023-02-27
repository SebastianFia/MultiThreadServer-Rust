use std::{
    error::Error,
    fmt::{self, Display},
    io,
};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct WorkerCreationError {
    details: String,
}

impl WorkerCreationError {
    pub fn new(details: String) -> Self {
        Self { details }
    }
}

impl Display for WorkerCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error creating worker: {}", self.details)
    }
}

impl Error for WorkerCreationError {}

impl From<io::Error> for WorkerCreationError {
    fn from(err: io::Error) -> Self {
        let details = format!("{err:?}");
        Self::new(details)
    }
}
