use crate::http::error::Error as HttpError;
use std::path::StripPrefixError;
use thiserror::Error;
use std::error;
use std::fmt;
use std::fmt::Debug;
use crate::server::ServerError;

#[derive(Error)]
pub struct Error {
    inner: ErrorKind
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    Http(HttpError),
    Path(StripPrefixError),
    Io(std::io::Error),
    Server(ServerError),
}

impl Error {
    /// Return true if the underlying error has the same type as T.
    pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }

    /// Return a reference to the lower level, inner error.
    pub fn get_ref(&self) -> &(dyn error::Error + 'static) {
        use self::ErrorKind::*;

        match self.inner {
            Http(ref e) => e,
            Path(ref e) => e,
            Io(ref e) => e,
            Server(ref e) => e,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("http::Error")
            // Skip the noise of the ErrorKind enum
            .field(&self.get_ref())
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.get_ref(), f)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("http error kind")
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Error {
        Error {
            inner: ErrorKind::Http(err),
        }
    }
}

impl From<StripPrefixError> for Error {
    fn from(err: StripPrefixError) -> Error {
        Error {
            inner: ErrorKind::Path(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            inner: ErrorKind::Io(err),
        }
    }
}

impl From<ServerError> for Error {
    fn from(err: ServerError) -> Error {
        Error {
            inner: ErrorKind::Server(err),
        }
    }
}

impl<T: Debug> From<std::result::Result<T, Error>> for Error {
    fn from(res: Result<T, Error>) -> Self {
        res.unwrap_err()
    }
}