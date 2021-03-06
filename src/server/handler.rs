use std::path::{Path, PathBuf};
use crate::http::request::Request;
use crate::http::method::Method;
use std::net::{TcpStream, Shutdown};
use crate::server::error::Error;
use crate::http::parser::Parser;
use std::io::{Read, Write};
use crate::server::{ServerError, resource};
use crate::server::response::response;

#[derive(Debug, Clone)]
pub struct Handler<T> {
    pub path: PathBuf,
    pub method: Method,
    pub handler: fn(Request<String>) -> T
}

#[derive(Debug, Clone)]
pub struct Handlers<T> {
    root: PathBuf,
    inner: Vec<Handler<T>>
}


impl Handler<String> {
    pub fn new(root: PathBuf, path: &str, method: &str, handler: fn(Request<String>) -> String) -> Self {
        let path = root.join(Path::new(path));
        let method = Method::from_str(method).unwrap();
        Handler {
            path,
            method,
            handler
        }
    }

    fn func(&self) -> fn(Request<String>) -> String {
        self.handler
    }

    fn exec(&self) {

    }
}

impl Handlers<String> {
    pub fn new(root: &str) -> Self {
        let root = Path::new(root).to_path_buf();
        Handlers {
            root,
            inner: Vec::new()
        }
    }

    pub fn add(&mut self, handler: Handler<String>) {
        self.inner.push(handler);
    }

    pub fn find(&self, path: &str, method: &str) -> Option<&Handler<String>> {
        let path = self.root.join(Path::new(path));
        let method = match Method::from_str(method) {
            Ok(m) => m,
            Err(_) => return None,
        };
        self.inner.iter()
            .find(|handler| handler.path == path && handler.method == method)
    }

    pub fn handle(&self, mut stream: TcpStream) -> Result<(), Error> {
        let mut data = [0u8; 256];
        loop {
            match stream.read(&mut data) {
                Ok(size) => {
                    println!("[info] receive {} bytes", size);
                    let request = Parser::new().parse_request(&data[..size])?; // should not use unwrap
                    let path = request.uri().path();
                    let method = request.method();
                    // println!("[info] path: {:?}", path);
                    // println!("[info] method: {:?}", request.method().as_str());
                    // if !resource::validate(self.root.to_str().unwrap(), path) {
                    //     println!("[error] not found requested resource: {}", path);
                    // }
                    println!("[info] handle request");
                    let handler = match self.find(path, method.as_str()) {
                        Some(f) => f,
                        None => {
                            println!("[error] handler function is not registered for {}", path);
                            return Err(Error::from(ServerError::new()));
                        }
                    };
                    let res_data = handler.func()(request);
                    let res = response(200, Some(&res_data))?;
                    println!("[info] response\r\n{}", res);
                    stream.write(res.as_bytes())?;
                    println!("[info] write to stream");
                    return Ok(());
                },
                Err(e) => {
                    println!("[error] failed to read from stream: because of {:?}", e);
                    stream.shutdown(Shutdown::Both).unwrap();
                    return Err(Error::from(e));
                }
            }
        }
    }
}