#[cfg(test)]
mod tests {
    use crate::server::ws_server::WSServer;

    #[test]
    fn test_accept_key() {
        let ws_server = WSServer {
            resource: String::from(""),
            host: String::from(""),
            origin: String::from(""),
            key: String::from("dGhlIHNhbXBsZSBub25jZQ=="),
            sub_protocol: String::from(""),
            extensions: vec![],
            version: 13,
        };

        assert_eq!(
            ws_server.create_accept_key(),
            "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        );
    }
}
