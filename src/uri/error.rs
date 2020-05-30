use crate::uri::scheme::InvalidScheme;

pub struct Error {
    inner: ErrorKind
}

pub enum ErrorKind {
    InvalidScheme(InvalidScheme),
    InvalidAuthority,
    InvalidHost,
    InvalidPort,
    InvalidPath,
    InvalidQuery,
}