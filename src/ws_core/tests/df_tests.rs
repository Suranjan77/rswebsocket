#[cfg(test)]
mod tests {
    use crate::ws_core::data_frame_rx::DFParser;
    use crate::ws_core::data_frame_tx::{Agent, DataFrame, FrameType};
    use crate::ws_core::ws::WSHandler;

    #[test]
    fn test_df_1() {
        let data = "Hello";
        let df = DataFrame::build(data.as_bytes(), FrameType::Text, Agent::Client).unwrap();
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
        let df = DataFrame::build(data.as_bytes(), FrameType::Text, Agent::Client).unwrap();
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
        let df = DataFrame::build(data.as_bytes(), FrameType::Text, Agent::Client).unwrap();
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
    fn test_bin_df() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        let df = DataFrame::build(data.as_bytes(), FrameType::Binary, Agent::Client).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 181);
        let mut expected = vec![0x82u8, 0xFE, 0x00, 0xAD];
        let mask = &d_frame[4..=7];
        expected.extend_from_slice(mask);
        data.as_bytes().iter().enumerate().for_each(|(i, data)| {
            expected.push(*data ^ mask[i % 4]);
        });
        assert_eq!(expected, d_frame);

        for d in d_frame {
            print!("0x{:02X}, ", d);
        }
    }

    #[test]
    fn test_server_sent_df() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        let df = DataFrame::build(data.as_bytes(), FrameType::Text, Agent::Server).unwrap();
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
        let df = DataFrame::build(data.as_bytes(), FrameType::Close, Agent::Client).unwrap();
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
        let df = DataFrame::build(data.as_bytes(), FrameType::Close, Agent::Server).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 21);
        let expected = vec![
            0x88u8, 0x13, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f,
            0x20, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e,
        ];
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_server_close_with_no_msg() {
        let data = ""; //19
        let mut df = DataFrame::build(data.as_bytes(), FrameType::Close, Agent::Server).unwrap();
        let d_frame = Vec::from(df);
        assert_eq!(d_frame.len(), 2);
        let expected = vec![0x88u8, 0];
        assert_eq!(expected, d_frame);
    }

    #[test]
    fn test_long_ping_df() {
        let data = "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition.";
        assert!(DataFrame::build(data.as_bytes(), FrameType::Ping, Agent::Client).is_err());
    }

    struct TestWSHandler {
        control_msg: String,
        text_msg: String,
        bin_msg: Vec<u8>,
        f_type: FrameType,
        agent: Agent,
    }

    impl WSHandler for TestWSHandler {
        fn who(&self) -> Agent {
            self.agent
        }

        fn handle_text_msg(&self, msg: String) {
            assert_eq!(msg, self.text_msg);
        }

        fn handle_bin_msg(&self, msg: Vec<u8>) {
            assert_eq!(msg, self.bin_msg);
        }

        fn handle_control(&self, ctrl_msg: String, f_type: FrameType) {
            assert_eq!(ctrl_msg, self.control_msg);
            assert_eq!(self.f_type, f_type);
        }
    }

    #[test]
    fn test_df_parsing() {
        let handler = TestWSHandler {
            control_msg: "Close for no reason".to_string(),
            text_msg: "The quick brown fox jumps over the lazy dog. This pangram contains \
            every letter of the English alphabet. It's often used for font displays and testing \
            character recognition."
                .to_string(),
            bin_msg: vec![
                0x54, 0x68, 0x65, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6B, 0x20, 0x62, 0x72, 0x6F, 0x77,
                0x6E, 0x20, 0x66, 0x6F, 0x78, 0x20, 0x6A, 0x75, 0x6D, 0x70, 0x73, 0x20, 0x6F, 0x76,
                0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6C, 0x61, 0x7A, 0x79, 0x20, 0x64, 0x6F,
                0x67, 0x2E, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x61, 0x6E, 0x67, 0x72, 0x61,
                0x6D, 0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x73, 0x20, 0x65, 0x76, 0x65,
                0x72, 0x79, 0x20, 0x6C, 0x65, 0x74, 0x74, 0x65, 0x72, 0x20, 0x6F, 0x66, 0x20, 0x74,
                0x68, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x6C, 0x69, 0x73, 0x68, 0x20, 0x61, 0x6C, 0x70,
                0x68, 0x61, 0x62, 0x65, 0x74, 0x2E, 0x20, 0x49, 0x74, 0x27, 0x73, 0x20, 0x6F, 0x66,
                0x74, 0x65, 0x6E, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x66,
                0x6F, 0x6E, 0x74, 0x20, 0x64, 0x69, 0x73, 0x70, 0x6C, 0x61, 0x79, 0x73, 0x20, 0x61,
                0x6E, 0x64, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6E, 0x67, 0x20, 0x63, 0x68, 0x61,
                0x72, 0x61, 0x63, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x63, 0x6F, 0x67, 0x6E, 0x69,
                0x74, 0x69, 0x6F, 0x6E, 0x2E,
            ],
            f_type: FrameType::Close,
            agent: Agent::Client,
        };

        let close_df = vec![
            0x88, 0x93, 0xC6, 0x53, 0x81, 0xC1, 0x85, 0x3F, 0xEE, 0xB2, 0xA3, 0x73, 0xE7, 0xAE,
            0xB4, 0x73, 0xEF, 0xAE, 0xE6, 0x21, 0xE4, 0xA0, 0xB5, 0x3C, 0xEF,
        ];
        DFParser::parse(&close_df, handler.who()).unwrap();

        let text_msg_df = vec![
            0x81u8, 0xFE, 0x00, 0xAD, 0x9B, 0x26, 0x0A, 0x86, 0xCF, 0x4E, 0x6F, 0xA6, 0xEA, 0x53,
            0x63, 0xE5, 0xF0, 0x06, 0x68, 0xF4, 0xF4, 0x51, 0x64, 0xA6, 0xFD, 0x49, 0x72, 0xA6,
            0xF1, 0x53, 0x67, 0xF6, 0xE8, 0x06, 0x65, 0xF0, 0xFE, 0x54, 0x2A, 0xF2, 0xF3, 0x43,
            0x2A, 0xEA, 0xFA, 0x5C, 0x73, 0xA6, 0xFF, 0x49, 0x6D, 0xA8, 0xBB, 0x72, 0x62, 0xEF,
            0xE8, 0x06, 0x7A, 0xE7, 0xF5, 0x41, 0x78, 0xE7, 0xF6, 0x06, 0x69, 0xE9, 0xF5, 0x52,
            0x6B, 0xEF, 0xF5, 0x55, 0x2A, 0xE3, 0xED, 0x43, 0x78, 0xFF, 0xBB, 0x4A, 0x6F, 0xF2,
            0xEF, 0x43, 0x78, 0xA6, 0xF4, 0x40, 0x2A, 0xF2, 0xF3, 0x43, 0x2A, 0xC3, 0xF5, 0x41,
            0x66, 0xEF, 0xE8, 0x4E, 0x2A, 0xE7, 0xF7, 0x56, 0x62, 0xE7, 0xF9, 0x43, 0x7E, 0xA8,
            0xBB, 0x6F, 0x7E, 0xA1, 0xE8, 0x06, 0x65, 0xE0, 0xEF, 0x43, 0x64, 0xA6, 0xEE, 0x55,
            0x6F, 0xE2, 0xBB, 0x40, 0x65, 0xF4, 0xBB, 0x40, 0x65, 0xE8, 0xEF, 0x06, 0x6E, 0xEF,
            0xE8, 0x56, 0x66, 0xE7, 0xE2, 0x55, 0x2A, 0xE7, 0xF5, 0x42, 0x2A, 0xF2, 0xFE, 0x55,
            0x7E, 0xEF, 0xF5, 0x41, 0x2A, 0xE5, 0xF3, 0x47, 0x78, 0xE7, 0xF8, 0x52, 0x6F, 0xF4,
            0xBB, 0x54, 0x6F, 0xE5, 0xF4, 0x41, 0x64, 0xEF, 0xEF, 0x4F, 0x65, 0xE8, 0xB5,
        ];
        DFParser::parse(&text_msg_df, handler.who()).unwrap();

        let binary_msg_df = vec![
            0x82, 0xFE, 0x00, 0xAD, 0xAF, 0x1B, 0xFE, 0xC2, 0xFB, 0x73, 0x9B, 0xE2, 0xDE, 0x6E,
            0x97, 0xA1, 0xC4, 0x3B, 0x9C, 0xB0, 0xC0, 0x6C, 0x90, 0xE2, 0xC9, 0x74, 0x86, 0xE2,
            0xC5, 0x6E, 0x93, 0xB2, 0xDC, 0x3B, 0x91, 0xB4, 0xCA, 0x69, 0xDE, 0xB6, 0xC7, 0x7E,
            0xDE, 0xAE, 0xCE, 0x61, 0x87, 0xE2, 0xCB, 0x74, 0x99, 0xEC, 0x8F, 0x4F, 0x96, 0xAB,
            0xDC, 0x3B, 0x8E, 0xA3, 0xC1, 0x7C, 0x8C, 0xA3, 0xC2, 0x3B, 0x9D, 0xAD, 0xC1, 0x6F,
            0x9F, 0xAB, 0xC1, 0x68, 0xDE, 0xA7, 0xD9, 0x7E, 0x8C, 0xBB, 0x8F, 0x77, 0x9B, 0xB6,
            0xDB, 0x7E, 0x8C, 0xE2, 0xC0, 0x7D, 0xDE, 0xB6, 0xC7, 0x7E, 0xDE, 0x87, 0xC1, 0x7C,
            0x92, 0xAB, 0xDC, 0x73, 0xDE, 0xA3, 0xC3, 0x6B, 0x96, 0xA3, 0xCD, 0x7E, 0x8A, 0xEC,
            0x8F, 0x52, 0x8A, 0xE5, 0xDC, 0x3B, 0x91, 0xA4, 0xDB, 0x7E, 0x90, 0xE2, 0xDA, 0x68,
            0x9B, 0xA6, 0x8F, 0x7D, 0x91, 0xB0, 0x8F, 0x7D, 0x91, 0xAC, 0xDB, 0x3B, 0x9A, 0xAB,
            0xDC, 0x6B, 0x92, 0xA3, 0xD6, 0x68, 0xDE, 0xA3, 0xC1, 0x7F, 0xDE, 0xB6, 0xCA, 0x68,
            0x8A, 0xAB, 0xC1, 0x7C, 0xDE, 0xA1, 0xC7, 0x7A, 0x8C, 0xA3, 0xCC, 0x6F, 0x9B, 0xB0,
            0x8F, 0x69, 0x9B, 0xA1, 0xC0, 0x7C, 0x90, 0xAB, 0xDB, 0x72, 0x91, 0xAC, 0x81,
        ];
        DFParser::parse(&binary_msg_df, handler.who()).unwrap();
    }
}
