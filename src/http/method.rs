use std::fmt;
use thiserror::Error;
use std::error::Error;
// use std::error::Error;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
    HEAD,
    TRACE,
    CONNECT,
    PATCH,
}

#[derive(Error)]
pub struct InvalidMethod {}

impl Method {
    pub fn from_bytes(method: &[u8]) -> Result<Self, InvalidMethod> {
        match method.len() {
            0 => Err(InvalidMethod::new()),
            3 => match method {
                b"GET" => Ok(Method::GET),
                b"PUT" => Ok(Method::PUT),
                _ => Err(InvalidMethod::new())
            },
            4 => match method {
                b"POST" => Ok(Method::POST),
                b"HEAD" => Ok(Method::HEAD),
                _ => Err(InvalidMethod::new())
            },
            5 => match method {
                b"PATCH" => Ok(Method::PATCH),
                b"TRACE" => Ok(Method::TRACE),
                _ => Err(InvalidMethod::new())
            },
            6 => match method {
                b"DELETE" => Ok(Method::DELETE),
                _ => Err(InvalidMethod::new()),
            },
            7 => match method {
                b"OPTIONS" => Ok(Method::OPTIONS),
                b"CONNECT" => Ok(Method::CONNECT),
                _ => Err(InvalidMethod::new())
            },
            _ => Err(InvalidMethod::new())
        }
    }

    pub fn from_str(method: &str) -> Result<Method, InvalidMethod> {
        match method.len() {
            0 => Err(InvalidMethod::new()),
            3 => match method {
                "GET" => Ok(Method::GET),
                "PUT" => Ok(Method::PUT),
                _ => Err(InvalidMethod::new())
            },
            4 => match method {
                "POST" => Ok(Method::POST),
                "HEAD" => Ok(Method::HEAD),
                _ => Err(InvalidMethod::new())
            },
            5 => match method {
                "PATCH" => Ok(Method::PATCH),
                "TRACE" => Ok(Method::TRACE),
                _ => Err(InvalidMethod::new())
            },
            6 => match method {
                "DELETE" => Ok(Method::DELETE),
                _ => Err(InvalidMethod::new()),
            },
            7 => match method {
                "OPTIONS" => Ok(Method::OPTIONS),
                "CONNECT" => Ok(Method::CONNECT),
                _ => Err(InvalidMethod::new())
            },
            _ => Err(InvalidMethod::new())
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::TRACE => "TRACE",
            Method::HEAD => "HEAD",
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl InvalidMethod {
    fn new() -> Self {
        InvalidMethod {}
    }
}

impl fmt::Debug for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidMethod").finish()
    }
}

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid HTTP method")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_from_bytes() {
        assert_eq!(super::Method::from_bytes(b"GET").unwrap(), super::Method::GET);
    }
    #[test]
    fn test_as_str() {
        assert_eq!(super::Method::GET.as_str(), "GET");
    }
}