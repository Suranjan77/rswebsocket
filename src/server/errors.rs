use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HTTPStatus {
    BadRequest,
    Unauthorised,
    MethodNotAllowed,
}

#[derive(Debug, Clone)]
pub struct HTTPError {
    pub code: u16,
    pub message: String,
    pub status: HTTPStatus,
}

impl fmt::Display for HTTPError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code: {} message: {} status {:?}",
            self.code, self.message, self.status
        )
    }
}

pub fn get_bad_request(msg: &str) -> HTTPError {
    HTTPError {
        message: msg.to_string(),
        code: 400,
        status: HTTPStatus::BadRequest,
    }
}

pub fn get_not_allowed(msg: &str) -> HTTPError {
    HTTPError {
        message: msg.to_string(),
        code: 405,
        status: HTTPStatus::MethodNotAllowed,
    }
}
