use std::fmt;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Scheme {
    pub protocol: Protocol
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Protocol {
    Http,
    Https,
    InvalidProtocol,
}

#[derive(Error)]
pub struct InvalidScheme {}

impl InvalidScheme {
    fn new() -> Self {
        InvalidScheme {}
    }
}

impl fmt::Debug for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidScheme").finish()
    }
}

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid scheme")
    }
}

impl Scheme {
    pub fn new(uri: &str) -> Option<Self> {
        let re_http = Regex::new(r"^http://").unwrap();
        let re_https = Regex::new(r"^https://").unwrap();
        if re_http.is_match(uri) {
            return Some(Scheme {
                protocol: Protocol::Http,
                });
        }
        if re_https.is_match(uri) {
            return Some(Scheme {
                protocol: Protocol::Https,
            });
        }
        None
    }
}

impl Protocol {
    pub fn to_string(&self) -> Result<String, InvalidScheme> {
        match self {
            Protocol::Http => Ok("http".to_string()),
            Protocol::Https => Ok("https".to_string()),
            Protocol::InvalidProtocol => Err(InvalidScheme::new())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_scheme_http() {
        let uri = "http://terassyi.net";
        assert_eq!(super::Scheme::new(uri), Some(super::Scheme{protocol: super::Protocol::Http}));
    }
    #[test]
    fn test_new_scheme_https() {
        let uri = "https://terassyi.net";
        assert_eq!(super::Scheme::new(uri), Some(super::Scheme{protocol: super::Protocol::Https}));
    }
    #[test]
    fn test_new_scheme_none() {
        let uri = "/terassyi.net";
        assert_eq!(super::Scheme::new(uri), None);
    }
}