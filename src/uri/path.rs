use regex::Regex;
use std::ops::Index;

#[derive(Debug, Eq, PartialEq)]
pub struct Path {
    pub path: String
}

impl Path {
    pub fn new(uri: &str) -> Option<Self> {
        let re = Regex::new("(/([a-zA-Z0-9]|\\.|-|:|\\+)+)+").unwrap();
        let path = match re.captures(uri) {
            Some(caps) => caps.index(0 as usize).to_string(),
            None => return None,
        };

        let re_host = Regex::new("//([a-zA-Z0-9]|\\.)+(:\\d+)?").unwrap();
        let host = match re_host.captures(uri) {
            Some(caps) => caps.index(0 as usize).to_string(),
            None => return Some(Path{path}),
        };
        let path: &str = &path[host.len()-1..];
        Some(Path{path: path.to_string()})
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_path() {
        let uri = "http://terassyi.net/hoge/fuga";
        assert_eq!(super::Path::new(uri), Some(super::Path{path: "/hoge/fuga".to_string()}));
    }
    #[test]
    fn test_new_path_with_query() {
        let uri = "http://terassyi.net/hoge+/fuga?query=test";
        assert_eq!(super::Path::new(uri), Some(super::Path{path: "/hoge+/fuga".to_string()}));
    }
    #[test]
    fn test_new_path_with_host_and_port() {
        let uri = "http://terassyi.net:8080/hoge/fuga/index.html";
        assert_eq!(super::Path::new(uri), Some(super::Path{path: "/hoge/fuga/index.html".to_string()}));
    }
    #[test]
    fn test_new_path_without_host() {
        let uri = "/hoge/fuga/index.html";
        assert_eq!(super::Path::new(uri), Some(super::Path{path: "/hoge/fuga/index.html".to_string()}));
    }
}