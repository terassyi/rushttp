use crate::http::version::Version;
use crate::uri::uri::Uri;
use crate::http::method::Method;
use crate::http::header::Header;
use crate::http::status::StatusCode;
use crate::http::parser::ParseError;

pub struct Response<T> {
    head: Parts,
    body: T
}

pub struct Parts {
    pub version: Version,
    pub status: StatusCode,
    pub header: Header,
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

    pub fn status(self, status: StatusCode) -> Builder {
        Builder {
            inner: self.inner.status(status)
        }
    }

    pub fn version(self, ver: Version) -> Builder {
        Builder {
            inner: self.inner.version(ver),
        }
    }

    pub fn header(self, header: Header) -> Builder {
        Builder {
            inner: self.inner.header(header)
        }
    }

    pub fn parts(self) -> Parts {
        self.inner
    }

    pub fn empty_response(self) -> Response<String> {
        Response {
            head: self.inner,
            body: "".to_string()
        }
    }

    pub fn response<T>(self, body: T) -> Response<T> {
        Response {
            head: self.inner,
            body,
        }
    }
}

impl Response<()> {
    pub fn builder() -> Builder {
        Builder::new()
    }

    // pub fn format(&self) -> Result<&[u8], ParseError> {
    //     let res = format!("{} {} {}\r\n", self.version().format(), self.status().to_string(), self.status().name());
    //
    // }
}

impl <T: std::fmt::Display> Response<T> {
    pub fn new(body: T) -> Response<T> {
        Response {
            head: Parts::new(),
            body
        }
    }

    pub fn from_parts(parts: Parts, body: T) -> Response<T> {
        Response {
            head: parts,
            body,
        }
    }

    pub fn version(&self) -> &Version {
        &self.head.version
    }

    pub fn status(&self) -> &StatusCode { &self.head.status }

    pub fn header(&self) -> &Header {
        &self.head.header
    }

    pub fn body(&self) -> &T {
        &self.body
    }

    pub fn format(&self) -> Result<String, ParseError> {
        let buf = format!("{} {} {}\r\n", self.version().format(), self.status().to_string(), self.status().name());
        let headers = self.header().format()?;
        let buf = format!("{}{}\r\n", buf, headers);
        Ok(format!("{}{}", buf, self.body()))
    }

}

impl Parts {
    fn new() -> Self {
        Parts {
            version: Version::default(),
            status: StatusCode::default(),
            header: Header::default()
        }
    }

    fn version(self, version: Version) -> Parts {
        Parts {
            version,
            status: self.status,
            header: self.header,
        }
    }

    fn status(self, status: StatusCode) -> Parts {
        Parts {
            version: self.version,
            status,
            header: self.header,
        }
    }

    fn header(self, header: Header) -> Parts {
        Parts {
            version: self.version,
            status: self.status,
            header,
        }
    }

    fn push_header(self, key: &str, value: &str) -> Parts {
        let mut header = self.header;
        header.add(key, value);
        Parts {
            version: self.version,
            status: self.status,
            header,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_response_format() {
        let mut h = super::Header::new();
        h.parse("Content-Type: text/html").unwrap();
        h.parse("Host: localhost:9999").unwrap();
        let b = super::Response::builder()
            .status(super::StatusCode(200))
            .header(h).empty_response();
        let b = b.format().unwrap();
        assert_eq!(b, "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nHost: localhost:9999\r\n\r\n".to_string());
    }
}