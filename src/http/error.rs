use crate::http::method::InvalidMethod;
use crate::uri::uri::InvalidUri;
use crate::http::status::InvalidStatusCode;
use crate::http::header::InvalidHeader;
use crate::http::parser::ParseError;
use std::{fmt, error};
use crate::uri::uri;
use thiserror::Error;
use crate::http::{status, method};
use std::fmt::Debug;
use std::convert::From;
use crate::http::version::InvalidVersion;

#[derive(Error)]
pub struct Error {
    inner: ErrorKind
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    Method(InvalidMethod),
    Uri(InvalidUri),
    Status(InvalidStatusCode),
    Header(InvalidHeader),
    Version(InvalidVersion),
    Parse(ParseError)
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
            Status(ref e) => e,
            Method(ref e) => e,
            Uri(ref e) => e,
            Header(ref e) => e,
            Version(ref e) => e,
            Parse(ref e) => e,
        }
    }
}

// impl error::Error for Error {
//     // Return any available cause from the inner error. Note the inner error is
//     // not itself the cause.
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         self.get_ref().source()
//     }
// }

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

impl From<InvalidStatusCode> for Error {
    fn from(err: InvalidStatusCode) -> Error {
        Error {
            inner: ErrorKind::Status(err),
        }
    }
}

impl From<InvalidMethod> for Error {
    fn from(err: InvalidMethod) -> Error {
        Error {
            inner: ErrorKind::Method(err),
        }
    }
}

impl From<InvalidUri> for Error {
    fn from(err: InvalidUri) -> Error {
        Error {
            inner: ErrorKind::Uri(err),
        }
    }
}

impl From<InvalidHeader> for Error {
    fn from(err: InvalidHeader) -> Error {
        Error {
            inner: ErrorKind::Header(err),
        }
    }
}

impl From<InvalidVersion> for Error {
    fn from(err: InvalidVersion) -> Error {
        Error {
            inner: ErrorKind::Version(err),
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error {
            inner: ErrorKind::Parse(err)
        }
    }
}

impl<T: Debug> From<std::result::Result<T, Error>> for Error {
    fn from(res: Result<T, Error>) -> Self {
        res.unwrap_err()
    }
}