
extern crate regex;

use std::env;
use crate::server::Server;
use crate::http::request::Request;
use crate::http::response::Response;
use std::path::Path;


pub mod uri;
pub mod http;

mod server;

fn main() {
    let cwd = env::current_dir().unwrap();
    let cwd = cwd.join(Path::new("src/static/assets/html"));
    let server = Server::new(cwd.to_str().unwrap())
        .bind(":9999");

}

fn index_handler(req: Request<String>) {
    let res = Response::builder();
}
