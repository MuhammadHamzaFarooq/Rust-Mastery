use reqwest::Error as NetworkError;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type Cause = Box<error::Error>;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    message: Cow<'static, str>,
    cause: Option<Cause>,
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Api,
    Network,
    Serialization,
}

impl Error {
    pub fn api<E: error::Error + 'static>(error: E) -> Self {
        Error {
            kind: Kind::Api,
            message: Cow::from("The weather API returned an error."),
            cause: Some(Box::new(error)),
        }
    }

    pub fn serialization<E: error::Error + 'static>(error: E) -> Self {
        Error {
            kind: Kind::Serialization,
            message: Cow::from("Unable to read response."),
            cause: Some(Box::new(error)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cause {
            None => write!(f, "{}", self.message),
            Some(ref cause) => write!(f, "{} ({})", self.message, cause),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause.as_ref().map(|cause| cause.as_ref())
    }
}

impl From<NetworkError> for Error {
    fn from(error: NetworkError) -> Self {
        Error {
            kind: Kind::Network,
            message: Cow::from("There was a network error."),
            cause: Some(Box::new(error)),
        }
    }
}
