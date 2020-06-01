use crate::http::request::{Request, Builder};
use crate::http::response::Response;
use thiserror::Error;
use std::fmt;

use crate::http::method::{InvalidMethod, Method};
use crate::http::error::{ErrorKind, Error};
use crate::uri::uri::{Uri, InvalidUri};
use std::io::Read;
use std::net::TcpStream;
use crate::http::error::ErrorKind::{Parse, Status};
use crate::http::version::Version;
use crate::uri::error::ErrorKind::InvalidPath;
use crate::http::header::Header;
use crate::http::status::StatusCode;


pub struct Parser {
    // stream: TcpStream
}

#[derive(Error)]
pub struct ParseError {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_request(&self, buf: &[u8]) -> Result<Request<String>, Error> {
        let request_builder = Request::builder();
        // parse data from tcp stream
        let mut data = String::from_utf8(buf.to_vec())
            .map_err(|_| Error::from(ParseError::new()))?;
        let mut request_header_body = data.split("\r\n\r\n");
        let mut request = request_header_body.next().ok_or(Error::from(ParseError::new()))?
            .split("\r\n");
        let body = request_header_body.next().unwrap_or("").to_string();
        // validate http packet
        let request_line: &str = request.next().unwrap();
        let ver = Version::parse(request_line).map_err(|e| Error::from(e))?;
        let mut  split_line = request_line.split_whitespace();
        let method = match split_line.next() {
            Some(m) => Method::from_str(m)?,
            None => return Err(Error::from(ParseError::new())),
        };
        let path = match split_line.next() {
            Some(p) => match Uri::new(p) {
                Some(uri) => uri,
                None => return Err(Error::from(InvalidUri::new())),
            },
            None => return Err(Error::from(ParseError::new())),
        };
        let request_builder = request_builder.method(method)
            .uri(path).version(ver);

        // parse header
        let mut header = Header::new();
        for line in request {
            header.parse(line)?;
        }
        let request_builder = request_builder.header(header);
        let req = Request::from_parts(request_builder.parts(), body);
        Ok(req)
    }

    pub fn parse_response(&self, buf: &[u8]) -> Result<Response<String>, Error> {
        let mut data = String::from_utf8(buf.to_vec())
            .map_err(|_| Error::from(ParseError::new()))?;
        let response_builder = Response::builder();
        let mut response_header_body = data.split("\r\r\n\n");
        let mut response = response_header_body.next().ok_or(Error::from(ParseError::new()))?
            .split("\r\n");
        let body = response_header_body.next().unwrap_or("").to_string();
        println!("body: {:?}", body);
        // validate http response
        let response_line: &str = response.next().unwrap();
        let mut split_line = response_line.split_whitespace();
        let ver = split_line.next().ok_or(Error::from(ParseError::new()))?;
        let version = Version::parse(ver).map_err(|e| Error::from(e))?;
        let status = StatusCode::from_bytes(split_line.next().ok_or(Error::from(ParseError::new()))?
            .as_bytes())?;
        let response_builder = response_builder.version(version).status(status);

        // parse header
        let mut header = Header::new();
        for line in response {
            header.parse(line)?;
            println!("{:?}", line);
        }
        let response_builder = response_builder.header(header);
        let res = Response::from_parts(response_builder.parts(), body);
        Ok(res)
    }
}
impl ParseError {
    fn new() -> ParseError {
        ParseError {}
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseError").finish()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("parse error")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_request() {
        use crate::http::header::HeaderValue;
        let parser = super::Parser::new();
        let packet = "GET / HTTP/1.1\r\nHost: terassyi.net\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\nrequest body\r\nhoge".as_bytes();
        let req = parser.parse_request(packet).unwrap();
        assert_eq!(req.method().as_str(), "GET");
        assert_eq!(req.version(), &super::Version::HTTP11);
        assert_eq!(req.uri().path(), "/");
        assert_eq!(req.header().map["User-Agent"], "curl/7.54.0".to_string());
        assert_eq!(req.body(), "request body\r\nhoge");
    }

    #[test]
    fn test_parse_response() {
        use crate::http::status::StatusCode;
        use crate::http::header::HeaderValue;
        let parser = super::Parser::new();
        let packet = "HTTP/1.1 200 OK\r\nDate: Sun, 25 Mar 2018 14:19:50 GMT\r\nContent-Type: text/html; charset=utf-8\r\n\r\nresponse body\r\nhogehoge".as_bytes();
        let res = parser.parse_response(packet).unwrap();
        assert_eq!(res.version(), &super::Version::HTTP11);
        assert_eq!(res.status(), &StatusCode::from_u16(200).unwrap());
        assert_eq!(res.header().map["Content-Type"], "text/html; charset=utf-8".to_string());
        assert_eq!(res.body(), "response body\r\nhogehoge");
    }
}