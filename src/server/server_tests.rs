#[cfg(test)]
mod tests {
    use crate::server::ws_server::WSServer;

    #[test]
    fn test_ws_handshake() {
        let handshake = "GET ws://echo.websocket.org/ HTTP/1.1\n\
                Host: echo.websocket.org\n\
                Connection: Upgrade\n\
                Pragma: no-cache\n\
                Cache-Control: no-cache\n\
                Upgrade: websocket\n\
                Origin: https://websocketking.com\n\
                Sec-WebSocket-Version: 13\n\
                Accept-Encoding: gzip, deflate, br, zstd\n\
                Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\n\
                Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits";

        let expected_res_handshake = "HTTP/1.1 101 Switching Protocols\n\
        Upgrade: websocket\n\
        Connection: Upgrade\n\
        Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
            .to_ascii_lowercase();

        let mut ws_server = WSServer::new();
        ws_server
            .parse_client_handshake(handshake.as_bytes().to_vec())
            .unwrap();

        let res_handshake = ws_server.create_handshake_response();

        let actual_handskhake = String::from_utf8(res_handshake)
            .unwrap()
            .to_ascii_lowercase();

        assert_eq!(expected_res_handshake, actual_handskhake);
    }
}
