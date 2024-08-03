use rand::RngCore;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FrameType {
    Continuation,
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl FrameType {
    fn op_code(&self) -> u8 {
        match self {
            FrameType::Continuation => 0x0,
            FrameType::Text => 0x1,
            FrameType::Binary => 0x2,
            FrameType::Close => 0x8,
            FrameType::Ping => 0x9,
            FrameType::Pong => 0xA,
        }
    }
}

pub enum Agent {
    Server,
    Client,
}

pub struct DataFrame {
    fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    op_code: u8,
    mask: bool,
    len_indicator: u8,
    payload_len: Vec<u8>,
    mask_key: [u8; 4],
    payload: Vec<u8>,
    agent: Agent,
}

impl From<DataFrame> for Vec<u8> {
    fn from(df: DataFrame) -> Vec<u8> {
        let mut d_frame = vec![];
        d_frame.push(
            (((df.fin as u8) << 7)
                | ((df.rsv1 as u8) << 6)
                | ((df.rsv2 as u8) << 5)
                | ((df.rsv3 as u8) << 4))
                | df.op_code,
        );
        d_frame.push(((df.mask as u8) << 7) | df.len_indicator);
        d_frame.extend(df.payload_len);
        if df.mask {
            d_frame.extend_from_slice(&df.mask_key);
        }
        d_frame.extend(df.payload);
        d_frame
    }
}

impl DataFrame {
    pub fn new(agent: Agent) -> DataFrame {
        DataFrame {
            fin: false,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            op_code: 0,
            mask: false,
            len_indicator: 0,
            payload_len: vec![],
            mask_key: [0; 4],
            payload: vec![],
            agent,
        }
    }

    pub fn build(&mut self, payload: &str, f_type: FrameType) -> Result<(), String> {
        validate_payload(payload, f_type)?;

        self.fin = true;
        self.rsv1 = false;
        self.rsv2 = false;
        self.rsv3 = false;
        self.op_code = f_type.op_code();

        self.encode_payload_length(payload.len());

        match self.agent {
            Agent::Server => {
                self.mask = false;
                self.payload.extend_from_slice(payload.as_bytes());
            }
            Agent::Client => {
                self.mask = true;
                self.mask_key = get_masking_key();
                payload.as_bytes().iter().enumerate().for_each(|(i, data)| {
                    self.payload.push(*data ^ self.mask_key[i % 4]);
                });
            }
        };

        Ok(())
    }

    fn encode_payload_length(&mut self, length: usize) {
        if length < 126 {
            self.len_indicator = length as u8;
        } else if length <= 65535 {
            self.len_indicator = 126u8;
            self.payload_len
                .extend_from_slice(&(length as u16).to_be_bytes());
        } else {
            self.len_indicator = 127u8;
            self.payload_len.extend_from_slice(&length.to_be_bytes());
        }
    }
}

fn get_masking_key() -> [u8; 4] {
    let mut key = [0u8; 4];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn validate_payload(payload: &str, frame_type: FrameType) -> Result<(), &str> {
    match frame_type {
        FrameType::Continuation | FrameType::Text | FrameType::Binary => Ok(()),
        FrameType::Close | FrameType::Ping | FrameType::Pong => {
            if payload.len() > 125 {
                Err("Payload length cannot exceed 125 bytes for control frame")
            } else {
                Ok(())
            }
        }
    }
}
