
extern crate regex;

use std::env;
use crate::server::Server;
use crate::http::request::Request;
use crate::http::response::Response;
use std::path::Path;
use std::fs::File;
use std::io::Read;


pub mod uri;
pub mod http;

extern crate chrono;
extern crate chrono_tz;
extern crate httpdate;

mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    // let mut server = Server::new("/etc/static/assets/html")
    let host = format!(":{}", port);
    println!("{}", host);
    // let mut server = Server::new("src/static/assets/html")
    let mut server = Server::new("/etc/rushttp/static/assets/html")
        .bind(&host);
    server.register("/", "GET", index_handler);
    server.serve()
}

fn index_handler(_: Request<String>) -> String {
    let mut buf = [0u8; 256];
    let mut file = File::open("/etc/rushttp/static/assets/html/index.html").expect("not found");
    file.read(&mut buf).expect("failed to read");
    String::from_utf8(buf.to_vec()).unwrap()
}
