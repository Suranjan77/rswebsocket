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

pub struct Context<H> {
    pub ws_state: ConnectionStatus,
    pub stream: TcpStream,
    pub handler: Arc<H>,
}

impl<H> Clone for Context<H>
where
    H: WSHandler,
{
    fn clone(&self) -> Self {
        Context {
            ws_state: self.ws_state.clone(),
            stream: self.stream.try_clone().unwrap(),
            handler: self.handler.clone(),
        }
    }
}

pub trait WSStream<H>
where
    H: WSHandler,
{
    fn context(&mut self) -> &mut Context<H>;

    fn read(&mut self) -> Result<(), String> {
        let mut data = vec![];
        let ctx = self.context();
        match ctx.stream.read_to_end(&mut data) {
            Ok(s) => println!("{} bytes read", s),
            Err(e) => {
                println!("Failed to read: {:?}", e);
                return Err("Read failed".to_string());
            }
        };

        let payload = match DFParser::parse(&data, ctx.handler.who()) {
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

                ctx.handler.handle_text_msg(msg)
            }
            FrameType::Binary => ctx.handler.handle_bin_msg(payload.data),
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
    fn write(&mut self, data: &[u8], f_type: FrameType) -> Result<(), String> {
        let ctx = self.context();
        let df = match DataFrame::build(data, f_type, ctx.handler.who()) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        if ctx.ws_state == ConnectionStatus::Open {
            match ctx.stream.write_all(&Vec::from(df)) {
                Ok(_) => Ok(()),
                Err(e) => {
                    ctx.ws_state = ConnectionStatus::Closed;
                    Err(e.to_string())
                }
            }
        } else {
            Err("Connection closed".to_string())
        }
    }

    fn shutdown(&mut self, msg: &str) -> Result<(), String> {
        self.write(msg.as_bytes(), FrameType::Close)?;
        let ctx = self.context();
        ctx.ws_state = ConnectionStatus::Closed;
        match ctx.stream.shutdown(Shutdown::Both) {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };
        Ok(())
    }
}
