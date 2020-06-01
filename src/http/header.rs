use std::collections::{HashMap, BinaryHeap};
use thiserror::Error;
use std::{fmt, error};
use regex::Regex;
use crate::http::parser::ParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Header {
    pub name: Vec<HeaderName>,
    pub map: HashMap<String, String>
}

#[derive(Debug, Eq, PartialEq)]
pub struct HeaderName(pub String);

#[derive(Debug, Eq, PartialEq)]
pub struct HeaderValue(pub String);

#[derive(Error)]
pub enum InvalidHeader {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue)
}

#[derive(Error)]
pub struct InvalidHeaderName {}

#[derive(Error)]
pub struct InvalidHeaderValue {}

impl HeaderName {
    pub fn new(name: &[u8]) -> Result<HeaderName, InvalidHeaderName> {
        // should not unwrap
        Ok(HeaderName(String::from_utf8(name.to_vec()).unwrap()))
    }

    // pub fn to_string(&self) -> String {
    //     (self).into()
    // }
}

impl From<HeaderName> for String {
    fn from(n: HeaderName) -> String {
        n.0
    }
}

impl HeaderValue {
    pub fn new(value: &[u8]) -> Result<HeaderValue, InvalidHeaderName> {
        Ok(HeaderValue(String::from_utf8(value.to_vec()).unwrap()))
    }

//     pub fn to_string(&self) -> String {
//         (self).into()
//     }
}

impl From<HeaderValue> for String {
    fn from(v: HeaderValue) -> String {
        v.0
    }
}

// impl ToString for HeaderName {
//     #[inline]
//     fn to_string(&self) -> String {
//         String::from(self)
//     }
// }

// impl ToString for HeaderValue {
//     #[inline]
//     fn to_string(&self) -> String {
//         String::from(self)
//     }
// }

// impl<HeaderValue, String> Into<String> for HeaderValue where String: From<HeaderValue>
// {
//     fn into(self) -> String {
//         String::from(self)
//     }
// }

impl Header {
    pub fn new() -> Self {
        Header::default()
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.name.push(HeaderName(key.to_string()));
        self.map.entry(key.to_string()).or_insert(value.to_string());
    }

    pub fn parse(&mut self, src: &str) -> Result<(), InvalidHeader> {
        let re = Regex::new(".*: .*").unwrap();
        if !re.is_match(src) {
            return Err(InvalidHeader::Name(InvalidHeaderName::new()));
        }
        let mut name_value = src.split(": ");
        let n: &str = name_value.next().unwrap();
        let v: &str = name_value.next().unwrap();
        self.add(n, v);
        Ok(())
    }

    pub fn format(&self) -> Result<String, ParseError> {
        let mut headers = String::new();
        for (name, value) in self.map.iter() {
            let header = &format!("{}: {}\r\n", name, value);
            headers = headers + header;
        }
        // headers = headers + "\r\n";
        Ok(headers)
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            name: Vec::new(),
            map: HashMap::new(),
        }
    }
}

impl From<InvalidHeaderName> for InvalidHeader {
    fn from(err: InvalidHeaderName) -> InvalidHeader {
        InvalidHeader::Name(err)
    }
}

impl InvalidHeader {
    pub fn is<T: error::Error + 'static>(&self) -> bool {
        self.get_ref().is::<T>()
    }

    pub fn get_ref(&self) -> &(dyn error::Error + 'static) {
        use self::InvalidHeader::*;

        match self {
            Name(ref e) => e,
            Value(ref e) => e,
        }
    }
}

impl From<InvalidHeaderValue> for InvalidHeader {
    fn from(err: InvalidHeaderValue) -> InvalidHeader {
        InvalidHeader::Value(err)
    }
}

impl fmt::Debug for InvalidHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("http::Header::Error")
            .field(&self.get_ref())
            .finish()
    }
}

impl fmt::Display for InvalidHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.get_ref(), f)
    }
}

impl InvalidHeaderName {
    fn new() -> Self {
        InvalidHeaderName {}
    }
}

impl fmt::Debug for InvalidHeaderName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderName").finish()
    }
}

impl fmt::Display for InvalidHeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid HTTP header name")
    }
}

impl InvalidHeaderValue {
    fn new() -> Self {
        InvalidHeaderValue {}
    }
}

impl fmt::Debug for InvalidHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderValue").finish()
    }
}

impl fmt::Display for InvalidHeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid HTTP header value")
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        let header = "Host: localhost:18888";
        let mut headers = super::Header::new();
        headers.parse(header).unwrap();
        assert_eq!(headers.name[0], super::HeaderName("Host".to_string()));
        assert_eq!(headers.map["Host"], "localhost:18888".to_string());
    }
    #[test]
    fn test_format() {
        let header = "Host: localhost:18888";
        let mut headers = super::Header::new();
        headers.parse(header).unwrap();
        headers.parse("Content-Type: text/html").unwrap();
        debug_assert_eq!(headers.format().unwrap(), format!("{}\r\n{}\r\n", header, "Content-Type: text/html"));
    }
}