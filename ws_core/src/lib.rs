pub mod base64;
pub mod data_frame_rx;
pub mod data_frame_tx;
pub mod http_utils;
pub mod sha1;
mod tests;

use crate::data_frame_rx::DFParser;
use crate::data_frame_tx::DataFrame;
use data_frame_tx::{Agent, FrameType};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::Arc;

pub trait WSHandler {
    fn who(&self) -> Agent;
    fn handle_text_msg(&self, msg: String);
    fn handle_bin_msg(&self, msg: Vec<u8>);
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
}

pub struct WSStream<H> {
    pub stream: TcpStream,
    pub handler: Arc<H>,
}

impl<H> Clone for WSStream<H> {
    fn clone(&self) -> Self {
        WSStream {
            stream: self.stream.try_clone().unwrap(),
            handler: self.handler.clone(),
        }
    }
}

impl<H> WSStream<H>
where
    H: WSHandler,
{
    pub fn read(&mut self) -> Result<(), String> {
        let mut data = [0u8; 512];
        let r_size = match self.stream.read(&mut data) {
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read: {:?}", e);
                return Err("Read failed".to_string());
            }
        };

        let payload = match DFParser::parse(&data[..r_size], self.handler.who()) {
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
                    Err(_) => {
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
                    Err(_) => {
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

        let d = Vec::from(df);

        match self.stream.write_all(&d) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn shutdown(&mut self, msg: &str) -> Result<(), String> {
        self.write(msg.as_bytes(), FrameType::Close)?;
        match self.stream.shutdown(Shutdown::Both) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };
        Ok(())
    }
}
