use crate::ws_core::data_frame_tx::{Agent, FrameType};

pub trait WSHandler {
    fn handle_text_msg(msg: String);
    fn handle_bin_msg(msg: Vec<u8>);
    fn handle_control(ctrl_msg: String, f_type: FrameType);
}

impl TryFrom<u8> for FrameType {
    type Error = &'static str;

    fn try_from(op_code: u8) -> Result<Self, Self::Error> {
        match op_code {
            0x0 => Ok(FrameType::Continuation),
            0x1 => Ok(FrameType::Text),
            0x2 => Ok(FrameType::Binary),
            0x8 => Ok(FrameType::Close),
            0x9 => Ok(FrameType::Ping),
            0xA => Ok(FrameType::Pong),
            _ => Err("Invalid op_code"),
        }
    }
}

/// A parser that can parse dataframe from a buffer of u8 and call the handler with parsed payload
/// when the handler is implementation of the trait WSHandler
pub struct DFParser<H> {
    agent: Agent,
    handler: H,
}

impl<H> DFParser<H>
where
    H: WSHandler,
{
    pub fn new(agent: Agent, handler: H) -> Self {
        DFParser { agent, handler }
    }

    /// Caller needs to create new `buffer` of Vec<u8> and read the stream till the *EOF*
    /// into the `buffer`
    pub fn parse(&self, buf: &Vec<u8>) -> Result<(), String> {
        if buf.len() < 3 {
            return Err("Minimum frame length is 3 bytes".to_string());
        }

        if buf.first().unwrap() & 0xF0 == 0x80 {
            return Err(
                "Fin bit unset or rsv1-3 set, this impl assumes no fragmentation and rsv1-3 unset"
                    .to_string(),
            );
        }

        let f_type: FrameType = match (buf.get(1).unwrap() & 0xF).try_into() {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(m_len) = buf.get(3) {
            match self.agent {
                Agent::Client => {
                    if m_len & 0x80 != 0x80 {
                        return Err("Mask bit unset is not allowed for client".to_string());
                    }
                    true
                }
                Agent::Server => {
                    if m_len & 0x80 != 0 {
                        return Err("Mask bit set is not allowed for server".to_string());
                    }
                    false
                }
            };

            match f_type {
                FrameType::Continuation | FrameType::Text | FrameType::Binary => (),
                FrameType::Close | FrameType::Ping | FrameType::Pong => {
                    if m_len % 0x7F > 125 {
                        return Err("Control frame length exceeds 125 bytes".to_string());
                    }
                }
            }

            let mut mask_idx = 4usize;

            // finds actual payload length based on the 3rd byte
            let payload_len = match m_len & 0x7F {
                ..126 => (m_len & 0x7F) as usize,
                126 => match buf.get(4..=5) {
                    Some(l) => {
                        mask_idx = 6;
                        usize::from_be_bytes(l.try_into().unwrap())
                    }
                    None => return Err("Invalid payload, missing payload length".to_string()),
                },
                _ => match buf.get(4..12) {
                    None => return Err("Invalid payload, missing payload length".to_string()),
                    Some(l) => {
                        mask_idx = 12;
                        usize::from_be_bytes(l.try_into().unwrap())
                    }
                },
            };

            let payload: Vec<u8> = match self.agent {
                Agent::Client => match buf.get(mask_idx..mask_idx + 4) {
                    Some(m) => match buf.get(mask_idx + 4..payload_len) {
                        Some(l) => l
                            .iter()
                            .enumerate()
                            .map(|(i, data)| *data ^ m[i % 4])
                            .collect(),
                        None => return Err("Invalid payload, incomplete payload".to_string()),
                    },
                    None => return Err("Invalid mask".to_string()),
                },
                Agent::Server => match buf.get(mask_idx..payload_len) {
                    Some(l) => l.to_vec(),
                    None => return Err("Invalid payload, incomplete payload".to_string()),
                },
            };

            match f_type {
                FrameType::Continuation | FrameType::Text => {
                    H::handle_text_msg(String::from_utf8(payload).unwrap())
                }
                FrameType::Binary => H::handle_bin_msg(payload),
                FrameType::Close | FrameType::Ping | FrameType::Pong => {
                    H::handle_control(String::from_utf8(payload).unwrap(), f_type)
                }
            }
        } else {
            return Err("Invalid dataframe, missing payload length".to_string());
        }

        Ok(())
    }
}
