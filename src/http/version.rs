use std::fmt;
use thiserror::Error;
use regex::Regex;
use std::ops::Index;

#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    HTTP09,
    HTTP10,
    HTTP11,
    HTTP2,
    HTTP3,
}

#[derive(Error)]
pub struct InvalidVersion {}

impl Version {
    pub fn new() -> Self {
        Version::default()
    }

    pub fn parse(src: &str) -> Result<Self, InvalidVersion> {
        let re = Regex::new("HTTP/\\d(\\.\\d)?").unwrap();
        if !re.is_match(src) {
            return Err(InvalidVersion::new());
        }
        let version_re = Regex::new("\\d(\\.\\d)?").unwrap();
        match version_re.captures(src) {
            Some(caps) => match caps.index(0 as usize) {
                "0.9" => Ok(Version::HTTP09),
                "1.0" => Ok(Version::HTTP10),
                "1.1" => Ok(Version::HTTP11),
                "2" => Ok(Version::HTTP2),
                "3" => Ok(Version::HTTP3),
                _ => Err(InvalidVersion::new()),
            },
            None => return Err(InvalidVersion::new()),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::HTTP11
    }
}

impl InvalidVersion {
     fn new() -> Self {
         InvalidVersion {}
     }
}

impl fmt::Debug for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidVersion").finish()
    }
}

impl fmt::Display for InvalidVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid HTTP version")
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_version_10() {
        assert_eq!(super::Version::parse("HTTP/1.0").unwrap(), super::Version::HTTP10);
    }
    #[test]
    fn test_parse_version_2() {
        assert_eq!(super::Version::parse("HTTP/2").unwrap(), super::Version::HTTP2);
    }
}