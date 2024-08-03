#[cfg(test)]
mod tests {
    use crate::ws_core::data_frame::{Agent, DataFrame, FrameType};

    #[test]
    fn test_df_1() {
        let data = "Hello";
        let mut df = DataFrame::new(Agent::Client);
        df.build(data, FrameType::Text).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 11);
        let mut expected = vec![0x81u8, 0x85];
        let mask = &d_frame[2..=5];
        expected.extend_from_slice(mask);
        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_df_2() {
        let data = "Hello World";
        let mut df = DataFrame::new(Agent::Client);
        df.build(data, FrameType::Text).unwrap();
        let d_frame = Vec::from(df);
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
    fn test_df_long() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        let mut df = DataFrame::new(Agent::Client);
        df.build(data, FrameType::Text).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 181);
        let mut expected = vec![0x81u8, 0xFE, 0x00, 0xAD];
        let mask = &d_frame[4..=7];
        expected.extend_from_slice(mask);
        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_server_sent_df() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        let mut df = DataFrame::new(Agent::Server);
        df.build(data, FrameType::Text).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 177);
        let expected = vec![
            0x81u8, 0x7E, 0x00, 0xAD, 0x54, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x20,
            0x62, 0x72, 0x6f, 0x77, 0x6e, 0x20, 0x66, 0x6f, 0x78, 0x20, 0x6a, 0x75, 0x6d, 0x70,
            0x73, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x7a,
            0x79, 0x20, 0x64, 0x6f, 0x67, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x61,
            0x6e, 0x67, 0x72, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
            0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x6c, 0x65, 0x74, 0x74, 0x65, 0x72, 0x20,
            0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x45, 0x6e, 0x67, 0x6c, 0x69, 0x73, 0x68,
            0x20, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x62, 0x65, 0x74, 0x2e, 0x20, 0x49, 0x74, 0x27,
            0x73, 0x20, 0x6f, 0x66, 0x74, 0x65, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66,
            0x6f, 0x72, 0x20, 0x66, 0x6f, 0x6e, 0x74, 0x20, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61,
            0x79, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67,
            0x20, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x63,
            0x6f, 0x67, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
        ];
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_client_close_df() {
        let data = "Close for no reason"; //19
        let mut df = DataFrame::new(Agent::Client);
        df.build(data, FrameType::Close).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 25);
        let mut expected = vec![0x88u8, 0x93];
        let mask = &d_frame[2..=5];
        expected.extend_from_slice(mask);
        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });
        assert_eq!(expected, d_frame);
    }
    #[test]
    fn test_server_close_df() {
        let data = "Close for no reason"; //19
        let mut df = DataFrame::new(Agent::Server);
        df.build(data, FrameType::Close).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 21);
        let expected = vec![
            0x88u8, 0x13, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f,
            0x20, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e,
        ];
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_long_ping_df() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        let mut df = DataFrame::new(Agent::Client);
        assert!(df.build(data, FrameType::Ping).is_err());
    }
}
