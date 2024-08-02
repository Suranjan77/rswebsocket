#[cfg(test)]
mod tests {
    use crate::server::errors::HTTPStatus;
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

    #[test]
    fn test_ws_invalid_http_version() {
        let handshake = "GET ws://echo.websocket.org/ HTTP/1.0\n\
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

        let mut ws_server = WSServer::new();

        validate_400_error(handshake, &mut ws_server, 400, HTTPStatus::BadRequest);
    }

    #[test]
    fn test_ws_invalid_key() {
        let handshake = "GET ws://echo.websocket.org/ HTTP/1.0\n\
                Host: echo.websocket.org\n\
                Connection: Upgrade\n\
                Pragma: no-cache\n\
                Cache-Control: no-cache\n\
                Upgrade: websocket\n\
                Origin: https://websocketking.com\n\
                Sec-WebSocket-Version: 13\n\
                Accept-Encoding: gzip, deflate, br, zstd\n\
                Sec-WebSocket-Key: YWJj\n\
                Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits";

        let mut ws_server = WSServer::new();

        validate_400_error(handshake, &mut ws_server, 400, HTTPStatus::BadRequest);
    }

    #[test]
    fn test_ws_missing_headers() {
        let handshake = "GET ws://echo.websocket.org/ HTTP/1.0\n\
                Host: echo.websocket.org\n\
                Pragma: no-cache\n\
                Cache-Control: no-cache\n\
                Upgrade: websocket\n\
                Origin: https://websocketking.com\n\
                Sec-WebSocket-Version: 13\n\
                Accept-Encoding: gzip, deflate, br, zstd\n\
                Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\n\
                Sec-WebSocket-Extensions: permessage-deflate; client_max_window_bits";

        let mut ws_server = WSServer::new();

        validate_400_error(handshake, &mut ws_server, 400, HTTPStatus::BadRequest);
    }

    #[test]
    fn test_ws_wrong_http_method() {
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

        let mut ws_server = WSServer::new();

        validate_400_error(handshake, &mut ws_server, 405, HTTPStatus::MethodNotAllowed);
    }

    fn validate_400_error(
        handshake: &str,
        ws_server: &mut WSServer,
        http_code: u16,
        status: HTTPStatus,
    ) {
        match ws_server.parse_client_handshake(handshake.as_bytes().to_vec()) {
            Ok(()) => (),
            Err(e) => {
                assert_eq!(e.code, http_code);
                assert_eq!(e.status, status);
            }
        }
    }
}
