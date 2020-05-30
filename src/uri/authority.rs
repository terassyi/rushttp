use regex::Regex;
use std::ops::Index;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Authority {
    pub user_info: Option<UserInfo>,
    pub host: String,
    pub port: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserInfo {
    pub user: String,
    pub password: String,
}

impl Authority {
    pub fn new(uri: &str) -> Option<Self> {
        let user_info = UserInfo::new(uri);
        let re_host = Regex::new("(//|@)([0-9a-z]{0,62}\\.)+[a-z]+").unwrap();
        let re_port = Regex::new(":\\d{1,5}").unwrap();
        let host = match re_host.captures(uri) {
            Some(caps) => {
                let mut h = caps.index(0 as usize);
                match h.chars().next().unwrap() {
                    '/' => h[2..].to_string(),
                    '@' => h[1..].to_string(),
                    _ => return None,
                }
            },
            None => return None,
        };
        let port: Option<usize> = match re_port.captures(uri) {
            Some(caps) => match caps.index(0)[1..].parse() {
                Ok(p) => Some(p),
                _ => None,
            },
            None => None,
        };
        Some(Authority{
            user_info,
            host,
            port,
        })
    }

    // pub fn user_info(&self) -> Option<UserInfo> {
    //     self.user_info
    // }

    pub fn user(&self) -> Option<&str> {
        match &self.user_info {
            Some(i) => Some(&i.user),
            None => None,
        }
    }

    pub fn password(&self) -> Option<&str> {
        match &self.user_info {
            Some(i) => Some(&i.password),
            None => None,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> Option<usize> {
        self.port
    }
}

impl Default for Authority {
    fn default() -> Self {
        Authority {
            user_info: None,
            host: "".to_string(),
            port: None
        }
    }
}

impl UserInfo {
    fn new(uri: &str) -> Option<Self> {
        let re = Regex::new("\\w+:\\w+@").unwrap();
        if let Some(caps) = re.captures(uri) {
            let info: Vec<&str> = caps.index(0 as usize).split(':').collect();
            let mut password = String::from(info[1 as usize]);
            password.pop().unwrap(); // should handle error
            return Some(UserInfo {
                user: String::from(info[0 as usize]),
                password,
            });
        }
        None
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            user: "".to_string(),
            password: "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new_user_info() {
        let  uri = "http://terassyi:hogehoge@terassyi.net:8080";
        assert_eq!(super::UserInfo::new(uri), Some(super::UserInfo{
            user: String::from("terassyi"),
            password: String::from("hogehoge"),
        }));
    }
    #[test]
    fn test_new_user_info_none() {
        let uri = "http://terassyi.net:8080";
        assert_eq!(super::UserInfo::new(uri), None);
    }

    #[test]
    fn test_new_authority() {
        let uri = "http://terassyi.net";
        assert_eq!(super::Authority::new(uri), Some(super::Authority{
            user_info: None,
            host: "terassyi.net".to_string(),
            port: None
        }));
    }
    #[test]
    fn test_new_authority_with_port() {
        let uri = "https://terassyi.net:8080";
        assert_eq!(super::Authority::new(uri), Some(super::Authority{
            user_info: None,
            host: "terassyi.net".to_string(),
            port: Some(8080)
        }));
    }
    #[test]
    fn test_new_authority_with_subdomain() {
        let uri = "https://test.terassyi.net:8080";
        assert_eq!(super::Authority::new(uri), Some(super::Authority{
            user_info: None,
            host: "test.terassyi.net".to_string(),
            port: Some(8080)
        }));
    }
    #[test]
    fn test_new_authority_with_user_info() {
        let uri = "https://terassyi:password@test.terassyi.net:8080";
        assert_eq!(super::Authority::new(uri), Some(super::Authority{
            user_info: Some(super::UserInfo{
                user: "terassyi".to_string(),
                password: "password".to_string(),
            }),
            host: "test.terassyi.net".to_string(),
            port: Some(8080)
        }));
    }
    #[test]
    fn test_new_authority_none() {
        let uri = "/hoge/fuga/index.html";
        assert_eq!(super::Authority::new(uri), None);
    }
}