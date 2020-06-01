use std::net::{Ipv4Addr, TcpListener, TcpStream, Shutdown};
use std::collections::HashMap;
use crate::http::request::Request;
use std::string::ParseError;
use std::str::FromStr;
use std::io::Read;
use std::{thread, fmt};
use std::env;
use crate::http::parser::Parser;
use std::path::{Path, PathBuf};
use std::borrow::Borrow;
use crate::server::error::Error;
// use crate::server::context::Context;
use thiserror::Error;
use std::ops::Deref;
use std::sync::Arc;
use crate::server::handler::{Handler, Handlers};
use crate::http::method::Method;


mod resource;
mod response;
mod context;
mod handler;
mod error;

#[derive(Debug, Clone)]
pub struct Server {
    addr: Ipv4Addr,
    port: usize,
    root: PathBuf,
    handlers: Handlers,
}

impl Server {
    pub fn new(root: &str) -> Self {
        Server {
            addr: Ipv4Addr::new(0,0,0,0),
            port: 80,
            root: Path::new(root).to_path_buf(),
            handlers: Handlers::new(root)
        }
    }

    pub fn register(&mut self, path: &str, method: &str, handler: fn(Request<String>)) {
        let handler = Handler::new((&self.root).to_path_buf(), path, method, handler);
        self.handlers.add(handler);
    }



    pub fn bind(self, host: &str) -> Self {
        let mut addr_port = host.split(":");
        let addr = match addr_port.next() {
            Some(a) => match a {
                "" => self.addr,
                _ => Ipv4Addr::from_str(a).unwrap(),
            },
            None => self.addr,
        };
        let port: usize = match addr_port.next() {
            Some(p) => p.parse().unwrap(),
            None => self.port,
        };
        Server {
            addr,
            port,
            root: self.root,
            handlers: self.handlers
        }
    }

    pub fn serve(&self) {
        let addr = format!("{}:{}",&self.addr.to_string(), &self.port.to_string());
        let listener = TcpListener::bind(addr).expect("[error] failed to bind");

        // TODO refactor
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let handlers = self.handlers.clone();
                    thread::spawn( move || {
                        match handlers.handle(stream) {
                            Ok(_) => {
                                println!("[info] exec handler function");
                            },
                            Err(e) => {
                                println!("[error] failed to handle: {:?}", e);
                            },
                        }
                    });
                },
                Err(e) => {
                    println!("[error] failed to read: {:?}", e);
                },
            }
        }
    }
}

// fn handle(handlers: &Vec<Handler>, mut stream: TcpStream) -> Result<(), Error> {
//     let mut data = [0u8; 256];
//     loop {
//         match stream.read(&mut data) {
//             Ok(size) => {
//                 println!("[info] receive {} bytes", size);
//                 let request = Parser::new().parse_request(&data[..size])?; // should not use unwrap
//                 let path = request.uri().path();
//                 let method = request.method();
//                 println!("[info] path: {:?}", path);
//                 println!("[info] method: {:?}", request.method().as_str());
//                 if !resource::validate(ctx.root.to_str().unwrap(), path) {
//                     println!("[error] not found requested resource: {}", path);
//                 }
//                 let handler = match  {
//                     Some(f) => f,
//                     None => {
//                         println!("[error] handler function is not registered for {}", path);
//                         return Err(Error::from(ServerError::new()));
//                     }
//                 };
//                 handler(ctx,request);
//             },
//             Err(e) => {
//                 println!("[error] failed to read from stream: because of {:?}", e);
//                 stream.shutdown(Shutdown::Both).unwrap();
//                 return Err(Error::from(e));
//             }
//         }
//     }
// }

#[derive(Error)]
pub struct ServerError {}

impl ServerError {
    fn new() -> ServerError {
        ServerError {}
    }
}

impl fmt::Debug for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ServerError").finish()
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Server error")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bind() {
        let server = super::Server::new("/static/assets/html");
        assert_eq!(server.bind(":8080").port, 8080);
    }
}