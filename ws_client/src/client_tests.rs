// #[cfg(test)]
// mod tests {
//     use ws_core::http_utils::parse_headers;
//     use url::Url;
//
//     #[test]
//     fn test_valid_handshake() {
//         let ws_client = WSClient::new(Url::parse("wss://www.example.com/chat/").unwrap()).unwrap();
//
//         let handshake = ws_client.create_handshake();
//
//         let h_lines: Vec<String> = handshake
//             .lines()
//             .take_while(|l| !l.is_empty())
//             .map(|z| z.to_string())
//             .collect();
//
//         assert_eq!(
//             h_lines.first(),
//             Some(&String::from("GET wss://www.example.com/chat/ HTTP/1.1"))
//         );
//
//         let headers = parse_headers(&h_lines);
//
//         assert_eq!(headers.get("host"), Some(&String::from("www.example.com")));
//         assert_eq!(headers.get("connection"), Some(&String::from("Upgrade")));
//         assert_eq!(headers.get("upgrade"), Some(&String::from("websocket")));
//         assert!(headers.contains_key("sec-websocket-key"));
//         assert_eq!(
//             headers.get("sec-websocket-version"),
//             Some(&String::from("13"))
//         );
//     }
//
//     #[test]
//     fn test_server_handshake_parsing() {
//         let server_handshake = "HTTP/1.1 101 Switching Protocols\n\
//         upgrade: websocket\n\
//         Connection: Upgrade\n\
//         Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
//
//         let ws_client = WSClient::new(Url::parse("wss://www.example.com/chat/").unwrap()).unwrap();
//
//         ws_client
//             .parse_handshake(server_handshake.as_bytes().to_vec())
//             .unwrap();
//     }
//
//     #[test]
//     fn test_server_handshake_parsing_invalid() {
//         let server_handshake = "HTTP/1.1 101 Switching Protocols\n\
//         Connection: Upgrade\n\
//         Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
//
//         let ws_client = WSClient::new(Url::parse("wss://www.example.com/chat/").unwrap()).unwrap();
//
//         assert!(ws_client
//             .parse_handshake(server_handshake.as_bytes().to_vec())
//             .is_err());
//     }
// }
