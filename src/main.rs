mod client;
mod server;
mod ws_core;

fn main() {
    client::create_handshake(String::from("/chat"), String::from("www.rswebsocket.com"));
    // ws_core::base64::encode("a".as_bytes());
    // ws_core::base64::decode("aGVsbG90aGVyZWU=");
}
