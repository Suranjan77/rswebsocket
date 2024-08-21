use crate::data_frame_tx::{Agent, FrameType};

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
pub struct DFParser {}

pub struct DFPayload {
    pub data: Vec<u8>,
    pub f_type: FrameType,
}

impl DFParser {
    /// Caller needs to create new `buffer` of Vec<u8> and read the stream till the *EOF*
    /// into the `buffer`
    pub fn parse(buf: &[u8], agent: Agent) -> Result<DFPayload, String> {
        if buf.len() < 3 {
            return Err("Minimum frame length is 2 bytes".to_string());
        }

        if buf.first().unwrap() & 0xF0 != 0x80 {
            return Err(
                "Fin bit unset or rsv1-3 set, this impl assumes no fragmentation and rsv1-3 unset"
                    .to_string(),
            );
        }

        let op_code = buf.first().unwrap() & 0xF;
        let f_type: FrameType = match op_code.try_into() {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(m_len) = buf.get(1) {
            match agent {
                Agent::Server => {
                    if m_len & 0x80 != 0x80 {
                        return Err("Mask bit unset is not allowed for client msg".to_string());
                    }
                    true
                }
                Agent::Client => {
                    if m_len & 0x80 != 0 {
                        return Err("Mask bit set is not allowed for server nsg".to_string());
                    }
                    false
                }
            };

            match f_type {
                FrameType::Continuation | FrameType::Text | FrameType::Binary => (),
                FrameType::Close | FrameType::Ping | FrameType::Pong => {
                    if m_len & 0x7F > 125 {
                        return Err("Control frame length exceeds 125 bytes".to_string());
                    }
                }
            }

            let mut mask_idx = 2usize;

            // finds actual payload length based on the 3rd byte
            let payload_len = match m_len & 0x7F {
                ..126 => (m_len & 0x7F) as usize,
                126 => match buf.get(2..=3) {
                    Some(l) => {
                        mask_idx = 4;
                        let cl = u16::from_be_bytes(l.try_into().unwrap());
                        cl as usize
                    }
                    None => return Err("Invalid payload, missing payload length".to_string()),
                },
                _ => match buf.get(2..10) {
                    None => return Err("Invalid payload, missing payload length".to_string()),
                    Some(l) => {
                        mask_idx = 10;
                        usize::from_be_bytes(l.try_into().unwrap())
                    }
                },
            };

            let payload: Vec<u8> = match agent {
                Agent::Server => match buf.get(mask_idx..mask_idx + 4) {
                    Some(m) => match buf.get(mask_idx + 4..payload_len + mask_idx + 4) {
                        Some(l) => l
                            .iter()
                            .enumerate()
                            .map(|(i, data)| *data ^ m[i % 4])
                            .collect(),
                        None => return Err("Invalid payload from client, incomplete payload".to_string()),
                    },
                    None => return Err("Invalid mask".to_string()),
                },
                Agent::Client => match buf.get(mask_idx..payload_len) {
                    Some(l) => l.to_vec(),
                    None => return Err("Invalid payload from server, incomplete payload".to_string()),
                },
            };

            Ok(DFPayload {
                data: payload,
                f_type,
            })
        } else {
            return Err("Invalid dataframe, missing payload length".to_string());
        }
    }
}
