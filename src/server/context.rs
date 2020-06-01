use std::path::{PathBuf, Path};
use crate::http::parser::Parser;
use std::collections::HashMap;
use crate::http::request::Request;

// // #[derive(Debug)]
// pub struct Context<'a> {
//     pub root: PathBuf,
//     handlers: &'a HashMap<String, fn(&Context, Request<String>)>
// }
//
// impl <'_> Context {
//     pub fn new(root: &str, handlers: &'static HashMap<String, fn(&Context, Request<String>)>) -> Self {
//         Context {
//             root: Path::new(root).to_path_buf(),
//             handlers
//         }
//     }
//
//     pub(crate) fn get_handler(&self, path: &str) -> Option<&fn(&Context, Request<String>)> {
//         self.handlers.get(path)
//     }
// }