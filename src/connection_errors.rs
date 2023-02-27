use std::{
    error::Error,
    fmt::{self, Display},
    io,
};

pub type ResponseResult<T> = Result<T, ResponseError>;
pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct ResponseError {
    details: String,
}

impl ResponseError {
    fn new(details: String) -> Self {
        Self { details }
    }
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ResponseError {}

impl From<io::Error> for ResponseError {
    fn from(err: io::Error) -> Self {
        let details = format!("{err:?}");
        Self::new(details)
    }
}

impl From<String> for ResponseError {
    fn from(err: String) -> Self {
        Self::new(err)
    }
}

impl From<EmptyRequestError> for ResponseError {
    fn from(err: EmptyRequestError) -> Self {
        let details = format!("{err:?}");
        Self::new(details)
    }
}

#[derive(Debug)]
pub struct EmptyRequestError {
    details: String,
}

impl EmptyRequestError {
    pub fn new() -> Self {
        let details = "The request sent by the page was empty".to_string();
        Self { details }
    }
}

impl Display for EmptyRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for EmptyRequestError {}
