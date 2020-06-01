use crate::http::version::Version;
use crate::uri::uri::Uri;
use crate::http::method::Method;
use crate::http::header::{Header, HeaderName, HeaderValue};
use crate::http::status::StatusCode;

pub struct Request<T> {
    head: Parts,
    body: T
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parts {
    pub version: Version,
    pub uri: Uri,
    pub method: Method,
    pub header: Header,
    // status: StatusCode,
}

pub struct Builder {
    pub inner: Parts
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            inner: Parts::new()
        }
    }

    pub fn parts(self) -> Parts {
        self.inner
    }

    pub fn method(self, method: Method) -> Builder {
        Builder {
            inner: self.inner.method(method)
        }
    }

    pub fn uri_from_str(self, uri: &str) -> Builder {
        Builder {
            inner: self.inner.uri_from_str(uri)
        }
    }

    pub fn uri(self, uri: Uri) -> Builder {
        Builder {
            inner: self.inner.uri(uri)
        }
    }

    pub fn header(self, header: Header) -> Builder {
        Builder {
            inner: self.inner.header(header)
        }
    }

    pub fn push_header(self, key: &str, value: &str) -> Builder {
        Builder {
            inner: self.inner.push_header(key, value),
        }
    }

    pub fn version(self, ver: Version) -> Builder {
        Builder {
            inner: self.inner.version(ver),
        }
    }
}

impl Request<()> {

    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<T> Request<T> {
    pub fn new(body: T) -> Request<T> {
        Request {
            head: Parts::new(),
            body,
        }
    }

    pub fn from_parts(parts: Parts, body: T) -> Request<T> {
        Request {
            head: parts,
            body,
        }
    }

    pub fn set_body(self, data: T) -> Request<T> {
        Request::from_parts(self.head, data)
    }

    pub fn version(&self) -> &Version {
        &self.head.version
    }

    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    pub fn method(&self) -> &Method {
        &self.head.method
    }

    pub fn header(&self) -> &Header {
        &self.head.header
    }

    pub fn body(&self) -> &T {
        &self.body
    }
}

impl Parts {
    fn new() -> Self {
        Parts {
            version: Version::default(),
            uri: Uri::default(),
            method: Method::default(),
            header: Header::default(),
            // status: StatusCode::default()
        }
    }

    fn method(self, method: Method) -> Parts {
        Parts {
            version: self.version,
            uri: self.uri,
            method,
            header: self.header
        }
    }

    fn uri_from_str(self, src: &str) -> Parts {
        let uri = Uri::new(src).unwrap();
        Parts {
            version: self.version,
            uri,
            method: self.method,
            header: self.header
        }
    }

    fn uri(self, src: Uri) -> Parts {
        Parts {
            version: self.version,
            uri: src,
            method: self.method,
            header: self.header
        }
    }

    fn version(self, version: Version) -> Parts {
        Parts {
            version,
            uri: self.uri,
            method: self.method,
            header: self.header,
        }
    }

    fn header(self, header: Header) -> Parts {
        Parts {
            version: self.version,
            uri: self.uri,
            method: self.method,
            header
        }
    }

    fn push_header(self, key: &str, value: &str) -> Parts {
        let mut header = self.header;
        header.add(key, value);
        Parts {
            version: self.version,
            uri: self.uri,
            method: self.method,
            header
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::http::header::{HeaderName, HeaderValue};
    use std::collections::HashMap;

    #[test]
    fn test_parts_method() {
        let p = super::Parts::new();
        let mut wanted = super::Parts::new();
        wanted.method = super::Method::POST;
        assert_eq!(p.method(super::Method::POST), wanted);
    }
    #[test]
    fn test_parts_header() {
        let p = super::Parts::new();
        let mut wanted = super::Parts::new();
        let mut maps = HashMap::new();
        maps.insert("test_header".to_string(), "test_value".to_string());
        wanted.header = super::Header {
            name: vec![super::HeaderName("test_header".to_string())],
            map: maps
        };
        let p = p.push_header("test_header", "test_value");
        assert_eq!(p.header.map[&"test_header".to_string()], "test_value".to_string());
    }
}