use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use url::Url;

use crate::client::ws_client::WSClient;
use crate::server::ws_server::WSServer;
use crate::ws_core::data_frame_rx::DFParser;
use crate::ws_core::data_frame_tx::{Agent, DataFrame, FrameType};

pub trait WSHandler {
    fn who(&self) -> Agent;
    fn handle_text_msg(&self, msg: String);
    fn handle_bin_msg(&self, msg: Vec<u8>);
}

#[derive(Eq, PartialEq)]
enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
}

pub struct WSStream<H> {
    ws_state: ConnectionStatus,
    stream: TcpStream,
    handler: H,
    client: WSClient,
    server: WSServer,
}

// todo: Isolate client and server logic in their respective libraries
impl<H> WSStream<H>
where
    H: WSHandler,
{
    pub fn connect(host: &str, handler: H) -> Result<WSStream<H>, String> {
        let host_uri = match Url::parse(host) {
            Ok(uri) => uri,
            Err(_) => return Err("Invalid host url".to_string()),
        };

        let tcp_stream = match TcpStream::connect(host_uri.to_string()) {
            Ok(t) => t,
            Err(e) => {
                println!("{:?}", e);
                return Err("Connection failed".to_string());
            }
        };

        let mut ws_stream = WSStream {
            ws_state: ConnectionStatus::Connecting,
            stream: tcp_stream,
            handler,
            client: WSClient::new(host_uri)?,
            server: WSServer::new(),
        };

        match handshake(&mut ws_stream) {
            Ok(_) => Ok(ws_stream),
            Err(e) => Err(e),
        }
    }

    pub fn read(&mut self) -> Result<(), String> {
        let mut data = vec![];
        match self.stream.read_to_end(&mut data) {
            Ok(s) => println!("{} bytes read", s),
            Err(e) => {
                println!("Failed to read: {:?}", e);
                return Err("Read failed".to_string());
            }
        };

        let payload = match DFParser::parse(&data, self.handler.who()) {
            Ok(p) => p,
            Err(e) => {
                self.shutdown("Invalid data")?;
                return Err(e);
            }
        };

        match payload.f_type {
            FrameType::Continuation | FrameType::Text => {
                let msg = match String::from_utf8(payload.data) {
                    Ok(s) => s,
                    Err(e) => {
                        self.shutdown("Invalid data")?;
                        return Err("Invalid utf8 string payload".to_string());
                    }
                };

                self.handler.handle_text_msg(msg)
            }
            FrameType::Binary => self.handler.handle_bin_msg(payload.data),
            FrameType::Close | FrameType::Ping | FrameType::Pong => {
                if payload.f_type == FrameType::Close {
                    self.shutdown("Close accepted")?
                }

                let msg = match String::from_utf8(payload.data) {
                    Ok(s) => s,
                    Err(e) => {
                        self.shutdown("Invalid data")?;
                        return Err("Invalid utf8 string payload".to_string());
                    }
                };

                if payload.f_type == FrameType::Ping {
                    self.write(msg.as_bytes(), FrameType::Pong)?;
                }
            }
        };

        Ok(())
    }

    pub fn write(&mut self, data: &[u8], f_type: FrameType) -> Result<(), String> {
        let df = match DataFrame::build(data, f_type, self.handler.who()) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        if self.ws_state == ConnectionStatus::Open {
            match self.stream.write_all(&Vec::from(df)) {
                Ok(_) => Ok(()),
                Err(e) => {
                    self.ws_state = ConnectionStatus::Closed;
                    return Err(e.to_string());
                }
            }
        } else {
            return Err("Connection closed".to_string());
        }
    }

    pub fn shutdown(&mut self, msg: &str) -> Result<(), String> {
        self.write(msg.as_bytes(), FrameType::Close)?;
        self.ws_state == ConnectionStatus::Closed;
        match self.stream.shutdown(Shutdown::Both) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };
        Ok(())
    }
}
fn handshake<H>(ws_stream: &mut WSStream<H>) -> Result<(), String>
where
    H: WSHandler,
{
    match ws_stream.handler.who() {
        Agent::Client => {
            let handshake = ws_stream.client.create_handshake();

            match ws_stream.stream.write(handshake.as_bytes()) {
                Ok(s) => println!("{} bytes written", s),
                Err(e) => {
                    println!("Failed to write handshake: {:?}", e);
                    return Err("Handshake failed".to_string());
                }
            };

            let mut res_handshake = vec![];
            match ws_stream.stream.read_to_end(&mut res_handshake) {
                Ok(s) => println!("{} bytes read", s),
                Err(e) => {
                    println!("Failed to read handshake: {:?}", e);
                    return Err("Handshake failed".to_string());
                }
            };

            match ws_stream.client.parse_handshake(res_handshake) {
                Ok(_) => ws_stream.ws_state = ConnectionStatus::Open,
                Err(e) => {
                    ws_stream.ws_state = ConnectionStatus::Closed;
                    return Err(e);
                }
            }
        }
        Agent::Server => {
            let handshake = ws_stream.server.create_handshake();

            match ws_stream.stream.write(&handshake) {
                Ok(s) => println!("{} bytes written", s),
                Err(e) => {
                    println!("Failed to write handshake: {:?}", e);
                    return Err("Handshake failed".to_string());
                }
            };

            let mut res_handshake = vec![];
            match ws_stream.stream.read_to_end(&mut res_handshake) {
                Ok(s) => println!("{} bytes read", s),
                Err(e) => {
                    println!("Failed to read handshake: {:?}", e);
                    return Err("Handshake failed".to_string());
                }
            };

            match ws_stream.server.parse_handshake(res_handshake) {
                Ok(_) => ws_stream.ws_state = ConnectionStatus::Open,
                Err(e) => {
                    ws_stream.ws_state = ConnectionStatus::Closed;
                    println!("HttpError {:?}", e);
                    return Err(e.message);
                }
            }
        }
    }

    Ok(())
}
