use rand::RngCore;

use crate::ws_core;

//todo: Extensions
pub fn create_handshake(req_uri: String, host: String) {
    let mut handshake: String = String::from("");
    // line
    handshake.push_str("GET ");
    handshake.push_str(req_uri.as_str());
    handshake.push_str(" HTTP/1.1\n");

    handshake.push_str("Host: ");
    handshake.push_str(host.as_str());
    handshake.push('\n');
    handshake.push_str("Upgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Version: 13\n");

    // websocket key
    handshake.push_str("Sec-WebSocket-Key: ");
    handshake.push_str(sec_ws_key().as_str());

    println!("{}", handshake);
}

fn sec_ws_key() -> String {
    let mut nonce = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut nonce);
    ws_core::base64::encode(&nonce)
}

//todo: fragmentation
///
/// ## Websocket payload Data-frame
///
/// ```
///     0                   1                   2                   3
///      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///     +-+-+-+-+-------+-+-------------+-------------------------------+
///     |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
///     |I|S|S|S|  (4)  |A|     (7)     |             (16/64)           |
///     |N|V|V|V|       |S|             |   (if payload len==126/127)   |
///     | |1|2|3|       |K|             |                               |
///     +-+-+-+-+-------+-+-------------+ - - - - - - - - - - - - - - - +
///     |     Extended payload length continued, if payload len == 127  |
///     + - - - - - - - - - - - - - - - +-------------------------------+
///     |                               |Masking-key, if MASK set to 1  |
///     +-------------------------------+-------------------------------+
///     | Masking-key (continued)       |          Payload Data         |
///     +-------------------------------- - - - - - - - - - - - - - - - +
///     :                     Payload Data continued ...                :
///     + - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +
///     |                     Payload Data continued ...                |
///     +---------------------------------------------------------------+
/// ```
///
fn create_data_frame(payload: &str) -> Vec<u8> {
    let mut d_frame: Vec<u8> = vec![];

    //fin,rsv1,rsv2,rsv3,opcode
    d_frame.push(0b10000001u8);

    d_frame.extend_from_slice(&encode_payload_length(payload.len()));

    let mask = masking_key();
    d_frame.extend_from_slice(&mask);

    payload.as_bytes().iter().enumerate().for_each(|(i, data)| {
        d_frame.push(*data ^ mask[i % 4]);
    });

    d_frame
}

fn masking_key() -> [u8; 4] {
    let mut key = [0u8; 4];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

/// ## Mask + encoded payload length
/// If payload length is 0 to 125 bytes long then 8 bits are used to encode mask and length,
/// MSB represent mask flag and remaining 7 bits represent length of the payload.
///
/// Or, if payload length is 126 bytes long then 3 bytes are used, first byte represent mask and 126,
/// remaining 2 bytes represent length of payload as 16-bit unsigned integer (u8)
///
/// Finally, if payload length is more than 126 bytes long, then 9 bytes are used, first byte represent mask and 127,
/// remaining 8 bytes represent length of payload as 64-bit unsigned integer (u64/usize)
fn encode_payload_length(length: usize) -> Vec<u8> {
    let mask_set = 0b10000000u8;

    //mask, length (in bytes)
    if length < 126 {
        // 8 bits to represent mask and length
        // MSB is 1 to set mask and remaining 7 bits represent length of data
        let mask_with_len = (length as u8) | mask_set;
        vec![mask_with_len]
    } else if length <= 65535 {
        // payload.len() <= (2^16) - 2

        // 24 bits to represent mask and length
        // First 8 bits: MSB represent mask and remaining 7 bits represent 126
        let mut d_vec = vec![126u8 | mask_set];

        // Remaining 16 bit represent length of data
        d_vec.extend_from_slice(&(length as u16).to_be_bytes());
        d_vec
    } else {
        // payload.len() >= (2^16) - 1

        // 9 bytes used to represent mask and length
        // First byte for mask and 127
        let mut d_vec = vec![127u8 | mask_set];

        // Remaining 8 bytes to represent length
        d_vec.extend_from_slice(&length.to_be_bytes());
        d_vec
    }
}

fn send() {}

fn receive() {}

fn close_connection() {}

#[cfg(test)]
mod tests {
    use crate::client::create_data_frame;

    #[test]
    fn test_data_frame_1() {
        let d_frame = create_data_frame("Hello");

        assert_eq!(d_frame.len(), 11);

        let mut expected = vec![0x81u8, 0x85];

        let mask = &d_frame[2..=5];
        expected.extend_from_slice(mask);

        "Hello".as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });

        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_data_frame_2() {
        let data = "Hello World";

        let d_frame = create_data_frame(data);

        assert_eq!(d_frame.len(), 17);

        let mut expected = vec![0x81u8, 0x8B];

        let mask = &d_frame[2..=5];
        expected.extend_from_slice(mask);

        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });

        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_data_frame_long() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains every letter of the English alphabet. It's often used for font displays and testing character recognition.";

        let d_frame = create_data_frame(data);

        assert_eq!(d_frame.len(), 181);

        let mut expected = vec![0x81u8, 0xFE, 0x00, 0xAD];

        let mask = &d_frame[4..=7];
        expected.extend_from_slice(mask);

        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });

        assert_eq!(expected, d_frame);
    }
}
