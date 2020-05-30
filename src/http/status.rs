use thiserror::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusCode(pub u16);

#[derive(Error)]
pub struct InvalidStatusCode {}

impl StatusCode {
    pub fn from_u16(code: u16) -> Result<StatusCode, InvalidStatusCode> {
        if 100 < code && code < 600 {
            return Ok(StatusCode(code));
        }
        Err(InvalidStatusCode::new())
    }

    pub fn from_bytes(code: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if code.len() < 3 {
            return Err(InvalidStatusCode::new());
        }
        let a = (code[0] - 48) as u16;
        let b = (code[1] - 48) as u16;
        let c = (code[2] - 48) as u16;
        if a > 5 || a == 0 || b > 9 || c > 9 {
            return Err(InvalidStatusCode::new());
        }
        Ok(StatusCode(a * 100 + b * 10 + c))
    }

    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    pub fn to_string(&self) -> String {
        let num: u16 = (*self).into();
        num.to_string()
    }
}

impl From<StatusCode> for u16 {
    fn from(status: StatusCode) -> u16 {
        status.0
    }
}

impl InvalidStatusCode {
    fn new() -> InvalidStatusCode {
        InvalidStatusCode {}
    }
}

impl fmt::Debug for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidStatusCode").finish()
    }
}

impl fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid HTTP status code")
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode(200)
    }
}

const CONTINUE: StatusCode =  StatusCode(100);

const OK :StatusCode = StatusCode(200);
const ACCEPTED: StatusCode = StatusCode(202);

const BAD_REQUEST: StatusCode = StatusCode(400);
const UNAUTHORIZED: StatusCode = StatusCode(401);
const FORBIDDEN: StatusCode = StatusCode(403);
const NOT_FOUND: StatusCode = StatusCode(404);
const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);
const REQUEST_TIMEOUT: StatusCode = StatusCode(408);

const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(500);
const NOT_IMPLEMENTED: StatusCode = StatusCode(501);
const BAD_GATEWAY: StatusCode = StatusCode(502);
const SERVICE_UNAVAILABLE: StatusCode = StatusCode(503);


#[cfg(test)]
mod tests {
    #[test]
    fn test_as_str() {
        assert_eq!(super::OK.to_string(), "200".to_string());
    }
}