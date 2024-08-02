use std::collections::HashMap;
use std::io::BufRead;

use rand::RngCore;
use url::Url;

use crate::ws_core::base64;
use crate::ws_core::http_utils::{parse_headers, validate_http_version};

pub struct WSClient {
    req_uri: Url,
}

impl WSClient {
    pub fn new(req_uri: &str) -> Result<Self, &str> {
        let host_uri = match Url::parse(req_uri) {
            Ok(uri) => uri,
            Err(_) => return Err("Invalid host url"),
        };

        Ok(WSClient { req_uri: host_uri })
    }

    pub fn create_handshake(&self) -> String {
        let mut handshake: String = String::from("");
        handshake.push_str("GET ");
        handshake.push_str(self.req_uri.as_str());
        handshake.push_str(" HTTP/1.1\n");
        handshake.push_str("Host: ");
        handshake.push_str(self.req_uri.host_str().unwrap());
        handshake.push('\n');
        handshake.push_str("Upgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Version: 13\n");
        handshake.push_str("Sec-WebSocket-Key: ");
        handshake.push_str(sec_ws_key().as_str());
        handshake
    }

    pub fn parse_server_handshake(&self, c_handshake: Vec<u8>) -> Result<(), String> {
        let h_lines: Vec<String> = c_handshake
            .lines()
            .map(|res| res.unwrap())
            .take_while(|l| !l.is_empty())
            .collect();

        if h_lines.is_empty() {
            return Err("Invalid handshake".to_string());
        }

        let status: Vec<&str> = h_lines.first().unwrap().splitn(3, " ").collect();
        if status.len() != 3 {
            return Err("Invalid status line".to_string());
        }

        let _ = match validate_http_version(status[0]) {
            Err(e) => Err(e),
            _ => Ok(()),
        };

        let _ = match verify_http_status(status[1]) {
            Err(e) => Err(e),
            _ => Ok(()),
        };

        let headers: HashMap<String, String> = parse_headers(&h_lines);

        if let Err(e) = validate_headers(&headers) {
            return Err(e.to_string());
        }

        Ok(())
    }
}

fn sec_ws_key() -> String {
    let mut nonce = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut nonce);
    base64::encode(&nonce)
}

fn validate_headers(headers: &HashMap<String, String>) -> Result<(), &str> {
    match headers.get("upgrade") {
        Some(upgrade) => {
            if !upgrade.eq_ignore_ascii_case("websocket") {
                return Err("Invalid upgrade header");
            }
        }
        None => return Err("Invalid upgrade header"),
    };

    match headers.get("connection") {
        Some(connection) => {
            if !connection.eq_ignore_ascii_case("upgrade") {
                return Err("Invalid connection header");
            }
        }
        None => return Err("Invalid connection header"),
    }

    match headers.get("sec-websocket-accept") {
        Some(_) => (),
        None => return Err("Invalid websocket key"),
    }

    Ok(())
}

fn verify_http_status(p0: &str) -> Result<(), &str> {
    match p0 {
        "101" => Ok(()),
        _ => Err("Invalid server status"),
    }
}
