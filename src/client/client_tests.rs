#[cfg(test)]
mod tests {
    use crate::client::ws_client::WSClient;
    use crate::ws_core::http_utils::parse_headers;

    #[test]
    fn test_invalid_host_url() {
        assert!(WSClient::new("w/sss").is_err());
    }

    #[test]
    fn test_valid_handshake() {
        let ws_client = WSClient::new("wss://www.example.com/chat/").unwrap();

        let handshake = ws_client.create_handshake();

        let h_lines: Vec<String> = handshake
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|z| z.to_string())
            .collect();

        assert_eq!(
            h_lines.first(),
            Some(&String::from("GET wss://www.example.com/chat/ HTTP/1.1"))
        );

        let headers = parse_headers(&h_lines);

        assert_eq!(headers.get("host"), Some(&String::from("www.example.com")));
        assert_eq!(headers.get("connection"), Some(&String::from("Upgrade")));
        assert_eq!(headers.get("upgrade"), Some(&String::from("websocket")));
        assert!(headers.contains_key("sec-websocket-key"));
        assert_eq!(
            headers.get("sec-websocket-version"),
            Some(&String::from("13"))
        );
    }

    #[test]
    fn test_server_handshake_parsing() {
        let server_handshake = "HTTP/1.1 101 Switching Protocols\n\
        upgrade: websocket\n\
        Connection: Upgrade\n\
        Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

        let ws_client = WSClient::new("wss://www.example.com/chat/").unwrap();

        ws_client
            .parse_server_handshake(server_handshake.as_bytes().to_vec())
            .unwrap();
    }

    #[test]
    fn test_server_handshake_parsing_invalid() {
        let server_handshake = "HTTP/1.1 101 Switching Protocols\n\
        Connection: Upgrade\n\
        Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

        let ws_client = WSClient::new("wss://www.example.com/chat/").unwrap();

        assert!(ws_client
            .parse_server_handshake(server_handshake.as_bytes().to_vec())
            .is_err());
    }
}
