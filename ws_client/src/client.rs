use rand::RngCore;
use std::collections::HashMap;
use std::io::{BufRead, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use url::Url;

use ws_core::http_utils::{parse_headers, validate_http_version};
use ws_core::{base64, ConnectionStatus, WSHandler, WSStream};

pub struct WSClient<H> {
    pub ws_state: ConnectionStatus,
    pub ws_stream: WSStream<H>,
}

impl<H> WSClient<H>
where
    H: WSHandler,
{
    pub fn connect(host: &str, handler: H) -> Result<Self, String> {
        let host_uri = match Url::parse(host) {
            Ok(uri) => uri,
            Err(_) => return Err("Invalid host url".to_string()),
        };

        let soc_addr = SocketAddrV4::new(
            Ipv4Addr::from_str(host_uri.host_str().unwrap()).unwrap(),
            host_uri.port().unwrap(),
        );

        let mut tcp_stream = match TcpStream::connect(soc_addr) {
            Ok(t) => t,
            Err(e) => {
                println!("{:?}", e);
                return Err("Connection failed".to_string());
            }
        };

        match handshake(&host_uri, &mut tcp_stream) {
            Ok(_) => Ok(WSClient {
                ws_state: ConnectionStatus::Open,
                ws_stream: WSStream {
                    stream: tcp_stream,
                    handler: Arc::new(handler),
                },
            }),
            Err(e) => Err(e),
        }
    }
}
fn handshake(host: &Url, stream: &mut TcpStream) -> Result<(), String> {
    let handshake = create_handshake(host);

    match stream.write_all(handshake.as_bytes()) {
        Ok(_) => println!("\nClient Handshake\n{handshake}\n"),
        Err(e) => {
            println!("Failed to write handshake: {:?}", e);
            return Err("Handshake failed".to_string());
        }
    };

    let mut buf = [0u8; 512];
    let r_size = match stream.read(&mut buf) {
        Ok(n) => n,
        Err(e) => {
            println!("Failed to read handshake: {:?}", e);
            return Err("Handshake failed".to_string());
        }
    };

    parse_handshake(buf[..r_size].to_vec())?;

    Ok(())
}

fn create_handshake(host: &Url) -> String {
    let mut handshake: String = String::from("");
    handshake.push_str("GET ");
    handshake.push_str(host.as_str());
    handshake.push_str(" HTTP/1.1\n");
    handshake.push_str("Host: ");
    handshake.push_str(host.host_str().unwrap());
    handshake.push('\n');
    handshake.push_str("Upgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Version: 13\n");
    handshake.push_str("Sec-WebSocket-Key: ");
    let key = sec_ws_key();
    handshake.push_str(&key);
    handshake
}

fn parse_handshake(c_handshake: Vec<u8>) -> Result<(), String> {
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

fn verify_http_status(status: &str) -> Result<(), &str> {
    match status {
        "101" => Ok(()),
        _ => Err("Invalid ws_server status"),
    }
}
