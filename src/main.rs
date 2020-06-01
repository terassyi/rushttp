
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

mod server;

fn main() {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.join(Path::new("src/static/assets/html"));
    let mut server = Server::new(cwd.to_str().unwrap())
        .bind(":9999");
    server.register("/", "GET", index_handler);
    server.serve()
}

fn index_handler(_: Request<String>) -> String {
    let mut buf = [0u8; 256];
    let mut file = File::open("./src/static/assets/html/index.html").expect("not found");
    file.read(&mut buf).expect("failed to read");
    String::from_utf8(buf.to_vec()).unwrap()
}
