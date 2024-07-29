use std::collections::HashMap;
use std::io::BufRead;

use crate::ws_core::{base64, sha1};
use crate::ws_core::base64::decode;
use crate::ws_core::ws_errors::{HTTPError, HTTPStatus};

pub struct WSServer {
    //todo Just for the sake of testing, remove this pub
    //The key needs to be derived from client's handshake
    pub key: String,
    pub resource: String,
    pub host: String,
    pub origin: String,
    pub sub_protocol: String,
    pub extensions: Vec<String>,
    pub version: u8,
}

impl Default for WSServer {
    fn default() -> Self {
        WSServer {
            resource: String::from(""),
            host: String::from(""),
            origin: String::from(""),
            key: String::from(""),
            sub_protocol: String::from(""),
            extensions: vec![],
            version: 13,
        }
    }
}

impl WSServer {
    pub fn new() -> Self {
        WSServer {
            ..Default::default()
        }
    }

    pub fn parse_client_handshake(&mut self, c_handshake: Vec<u8>) -> Result<(), HTTPError> {
        let h_lines: Vec<String> = c_handshake
            .lines()
            .map(|res| res.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();

        if h_lines.is_empty() {
            return Err(HTTPError {
                message: "Invalid Request".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            });
        }

        let status: Vec<&str> = h_lines.first().unwrap().splitn(3, " ").collect();
        if status.len() != 3 {
            return Err(HTTPError {
                code: 400,
                message: "Invalid HTTP Request".to_string(),
                status: HTTPStatus::BadRequest,
            });
        }

        verify_http_status(status[0])?;
        verify_resource_uri(status[1])?;
        validate_http_version(status[2])?;

        self.resource = status[1].to_string();

        let headers: HashMap<String, String> = h_lines
            .iter()
            .skip(1)
            .map(|d| {
                let mut header = d.splitn(2, ":");
                (
                    header.next().unwrap().to_ascii_lowercase(),
                    header.next().unwrap().to_string(),
                )
            })
            .filter(|(k, v)| !k.is_empty() && !v.is_empty())
            .collect();

        validate_headers(&headers);

        self.extract_headers_info(&headers);

        Ok(())
    }

    pub fn extract_headers_info(&mut self, headers: &HashMap<String, String>) {
        self.host = headers.get("host").unwrap().to_string();
        self.version = headers
            .get("sec-websocket-version")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.sub_protocol = headers.get("sec-websocket-protocol").unwrap().to_string();
        self.origin = headers.get("origin").unwrap().to_string();
        self.key = headers.get("sec-websocket-key").unwrap().to_string();
        self.extensions = headers
            .get("sec-websocket-extensions")
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();
    }

    pub fn create_accept_key(&self) -> String {
        let hash_str = sha1::hash(&(self.key.clone() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"));
        base64::encode(hash_str.as_slice())
    }
}

fn validate_headers(headers: &HashMap<String, String>) -> Result<(), HTTPError> {
    match headers.get("host") {
        Some(_) => (),
        None => {
            return Err(HTTPError {
                message: "Invalid header <Host>".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            })
        }
    };

    match headers.get("upgrade") {
        Some(upgrade) => {
            if !upgrade.eq_ignore_ascii_case("upgrade") {
                return Err(HTTPError {
                    message: "Invalid header value upgrade <upgrade: websocket>".to_string(),
                    code: 400,
                    status: HTTPStatus::BadRequest,
                });
            }
        }
        None => {
            return Err(HTTPError {
                message: "Invalid header <Upgrade>".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            })
        }
    };

    match headers.get("connection") {
        Some(connection) => {
            if !connection.eq_ignore_ascii_case("upgrade") {
                return Err(HTTPError {
                    message: "Invalid header value connection <connection: upgrade>".to_string(),
                    code: 400,
                    status: HTTPStatus::BadRequest,
                });
            }
        }
        None => {
            return Err(HTTPError {
                message: "Invalid header <Connection>".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            })
        }
    }

    match headers.get("sec-websocket-key") {
        Some(key) => {
            if decode(key).len() != 16 {
                return Err(HTTPError {
                    message:
                        "Invalid header value sec-websocket-key <sec-websocket-key: 16 random \
                    btyes base64 encoded>"
                            .to_string(),
                    code: 400,
                    status: HTTPStatus::BadRequest,
                });
            }
        }
        None => {
            return Err(HTTPError {
                message: "Invalid header <sec-websocket-key>".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            })
        }
    }

    match headers.get("sec-websocket-version") {
        Some(version) => {
            if !version.eq("13") {
                return Err(HTTPError {
                    message: "Version not supported <sec-websocket-version: 13>".to_string(),
                    code: 400,
                    status: HTTPStatus::BadRequest,
                });
            }
        }
        None => {
            return Err(HTTPError {
                message: "Invalid header <sec-websocket-version>".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            })
        }
    }

    Ok(())
}

fn validate_http_version(p0: &str) -> Result<(), HTTPError> {
    match p0.splitn(2, "/").last() {
        Some(v) => match v.parse::<f32>() {
            Ok(version) => {
                if version < 1.1 {
                    Err(HTTPError {
                        message: "Invalid HTTP version".to_string(),
                        code: 400,
                        status: HTTPStatus::BadRequest,
                    })
                } else {
                    Ok(())
                }
            }
            Err(_) => Err(HTTPError {
                message: "Invalid HTTP version".to_string(),
                code: 400,
                status: HTTPStatus::BadRequest,
            }),
        },
        None => Err(HTTPError {
            message: "Invalid Request".to_string(),
            code: 400,
            status: HTTPStatus::BadRequest,
        }),
    }
}

//todo
fn verify_resource_uri(p0: &str) -> Result<(), HTTPError> {
    Ok(())
}

fn verify_http_status(p0: &str) -> Result<(), HTTPError> {
    match p0 {
        "GET" => Ok(()),
        _ => Err(HTTPError {
            code: 405,
            message: "Method Not Allowed".to_string(),
            status: HTTPStatus::MethodNotAllowed,
        }),
    }
}
