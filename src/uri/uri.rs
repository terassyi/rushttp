
use super::scheme::Scheme;
use super::authority::*;
use super::query::Query;
use super::path::Path;
use std::collections::HashMap;
use thiserror::Error;
use crate::uri::scheme::Protocol;
use std::fmt;


#[derive(Debug, Eq, PartialEq)]
pub struct Uri {
    scheme: Option<Scheme>,
    authority: Option<Authority>,
    path_and_query: Option<PathAndQuery>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PathAndQuery {
    pub path: Option<Path>,
    pub query: Option<Query>,
}

#[derive(Error)]
pub struct InvalidUri {}

impl Uri {
    pub fn new(uri: &str) -> Option<Self> {
        let scheme = Scheme::new(uri);
        let authority = Authority::new(uri);
        let path_and_query = PathAndQuery::new(uri);
        Some(Uri {
            scheme,
            authority,
            path_and_query
        })
    }

    pub fn scheme(&self) -> Option<String> {
        match self.scheme {
            Some(s) => s.protocol.to_string().ok(),
            None => None,
        }
    }

    pub fn user(&self) -> Option<&str> {
        match &self.authority {
            Some(a) => a.user(),
            None => None,
        }
    }

    pub fn password(&self) -> Option<&str> {
        match &self.authority {
            Some(a) => a.password(),
            None => None,
        }
    }

    pub fn host(&self) -> Option<&str> {
        match &self.authority {
            Some(a) => Some(a.host()),
            None => None,
        }
    }

    pub fn port(&self) -> Option<usize> {
        match &self.authority {
            Some(a) => match a.port() {
                Some(p) => Some(p),
                None => match &self.scheme {
                    Some(s) => match s.protocol {
                        Protocol::Http => Some(80),
                        Protocol::Https => Some(443),
                        Protocol::InvalidProtocol => None,
                    },
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn path(&self) -> &str {
        match &self.path_and_query {
            Some(p) => p.path(),
            None => "/",
        }
    }

    pub fn query(&self) -> Option<HashMap<String, String>> {
        match &self.path_and_query {
            Some(q) => q.query(),
            None => None
        }
    }
}

impl PathAndQuery {
    fn new(uri: &str) -> Option<PathAndQuery> {
        Some(PathAndQuery{
            path: Path::new(uri),
            query: Query::new(uri),
        })
    }

    fn path(&self) -> &str {
        match &self.path {
            Some(p) => p.path(),
            None => "/",
        }
    }

    fn query(&self) -> Option<HashMap<String, String>> {
        self.query()
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri {
            scheme: None,
            authority: None,
            path_and_query: None
        }
    }
}

impl InvalidUri {
    pub fn new() -> Self {
        InvalidUri {}
    }
}

impl fmt::Debug for InvalidUri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidUri").finish()
    }
}

impl fmt::Display for InvalidUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid HTTP uri")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_uri() {
        let uri = "https://test.terassyi.net/hogehoge/fuugafuga/index.php?query=test";
        let res = super::Uri::new(uri);
        // assert_eq!(res.scheme, Some(super::Scheme{protocol: super::Protocol::Https}));
        // assert_eq!(res.authority.unwrap().user_info, None);
        // assert_eq!(res.authority.unwrap().host, "test.terassyi.net".to_string());
        // assert_eq!(res.authority.unwrap().port, None);
        assert_eq!(res.unwrap().path_and_query.unwrap().path.unwrap(), super::Path{path: "/hogehoge/fuugafuga/index.php".to_string()});
    }
    #[test]
    fn test_new_uri_from_path() {
        let uri = "/hoge/fuga/index.html";
        let res = super::Uri::new(uri);
        // assert_eq!(res.authority, None);
        assert_eq!(res.unwrap().path_and_query.unwrap().path.unwrap(), super::Path{path: "/hoge/fuga/index.html".to_string()});
    }
}