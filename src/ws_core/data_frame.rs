use rand::RngCore;

//todo: fragmentation
pub fn create_data_frame(payload: &str) -> Vec<u8> {
    let mut d_frame: Vec<u8> = vec![];
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

fn encode_payload_length(length: usize) -> Vec<u8> {
    let mask_set = 0b10000000u8;
    if length < 126 {
        let mask_with_len = (length as u8) | mask_set;
        vec![mask_with_len]
    } else if length <= 65535 {
        let mut d_vec = vec![126u8 | mask_set];
        d_vec.extend_from_slice(&(length as u16).to_be_bytes());
        d_vec
    } else {
        let mut d_vec = vec![127u8 | mask_set];
        d_vec.extend_from_slice(&length.to_be_bytes());
        d_vec
    }
}

#[cfg(test)]
mod tests {
    use crate::ws_core::data_frame::create_data_frame;

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
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains every \
        letter of the English alphabet. It's often used for font displays and testing character \
        recognition.";

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
