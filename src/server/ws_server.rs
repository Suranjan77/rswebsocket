use std::collections::HashMap;
use std::io::BufRead;

use url::Url;

use crate::server::errors;
use crate::server::errors::{get_bad_request, HTTPError};
use crate::ws_core::base64::decode;
use crate::ws_core::http_utils::{parse_headers, validate_http_version};
use crate::ws_core::{base64, sha1};

pub struct WSServer {
    key: String,
    resource: String,
    host: String,
    origin: String,
    sub_protocol: String,
    extensions: Vec<String>,
    version: u8,
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

    pub fn parse_handshake(&mut self, c_handshake: Vec<u8>) -> Result<(), HTTPError> {
        let h_lines: Vec<String> = c_handshake
            .lines()
            .map(|res| res.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();

        let headers: HashMap<String, String> = parse_headers(&h_lines);

        if h_lines.is_empty() {
            return Err(get_bad_request("Invalid Request"));
        }

        let status: Vec<&str> = h_lines.first().unwrap().splitn(3, " ").collect();
        if status.len() != 3 {
            return Err(get_bad_request("Invalid HTTP Request"));
        }

        verify_http_method(status[0])?;
        verify_resource_uri(status[1])?;
        let _ = match validate_http_version(status[2]) {
            Ok(()) => Ok(()),
            Err(e) => Err(get_bad_request(e)),
        };

        self.resource = status[1].to_string();

        validate_headers(&headers)?;

        self.extract_headers_info(&headers);

        Ok(())
    }

    pub fn create_handshake(&self) -> Vec<u8> {
        let mut res = vec![];
        res.extend_from_slice("HTTP/1.1 101 Switching Protocols\n".as_bytes());
        res.extend_from_slice("Upgrade: websocket\nConnection: Upgrade\n".as_bytes());
        res.extend_from_slice("Sec-WebSocket-Accept: ".as_bytes());
        res.extend_from_slice(self.create_accept_key().as_bytes());
        res
    }

    fn extract_headers_info(&mut self, headers: &HashMap<String, String>) {
        self.host = headers.get("host").unwrap().to_string();
        self.version = headers
            .get("sec-websocket-version")
            .unwrap()
            .parse::<u8>()
            .unwrap();
        self.sub_protocol = match headers.get("sec-websocket-protocol") {
            Some(v) => v.to_string(),
            None => String::from(""),
        };
        self.origin = match headers.get("origin") {
            Some(v) => v.to_string(),
            None => String::from(""),
        };
        self.key = headers.get("sec-websocket-key").unwrap().to_string();
        self.extensions = match headers.get("sec-websocket-extensions") {
            Some(v) => v.split(",").map(|s| s.trim().to_string()).collect(),
            None => vec![],
        }
    }

    fn create_accept_key(&self) -> String {
        let hash_str = sha1::hash(&(self.key.clone() + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"));
        base64::encode(hash_str.as_slice())
    }
}

fn validate_headers(headers: &HashMap<String, String>) -> Result<(), HTTPError> {
    match headers.get("host") {
        Some(_) => (),
        None => return Err(get_bad_request("Invalid header <Host>")),
    };

    match headers.get("upgrade") {
        Some(upgrade) => {
            if !upgrade.eq_ignore_ascii_case("websocket") {
                return Err(get_bad_request(
                    "Invalid header value upgrade <upgrade: websocket>",
                ));
            }
        }
        None => return Err(get_bad_request("Invalid header <Upgrade>")),
    };

    match headers.get("connection") {
        Some(connection) => {
            if !connection.eq_ignore_ascii_case("upgrade") {
                return Err(get_bad_request(
                    "Invalid header value connection <connection: upgrade>",
                ));
            }
        }
        None => return Err(get_bad_request("Invalid header <Connection>")),
    }

    match headers.get("sec-websocket-key") {
        Some(key) => {
            if decode(key).len() != 16 {
                return Err(get_bad_request(
                    "Invalid header value sec-websocket-key <sec-websocket-key: 16 random \
                    btyes base64 encoded>",
                ));
            }
        }
        None => return Err(get_bad_request("Invalid header <sec-websocket-key>")),
    }

    match headers.get("sec-websocket-version") {
        Some(version) => {
            if !version.eq("13") {
                return Err(get_bad_request(
                    "Version not supported <sec-websocket-version: 13>",
                ));
            }
        }
        None => return Err(get_bad_request("Invalid header <sec-websocket-version>")),
    }

    Ok(())
}

fn verify_resource_uri(p0: &str) -> Result<(), HTTPError> {
    match Url::parse(p0) {
        Ok(_) => Ok(()),
        Err(_) => Err(get_bad_request("Malformed resource uri")),
    }
}

fn verify_http_method(p0: &str) -> Result<(), HTTPError> {
    match p0 {
        "GET" => Ok(()),
        _ => Err(errors::get_not_allowed("Method Not Allowed")),
    }
}
