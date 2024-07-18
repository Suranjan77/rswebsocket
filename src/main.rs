mod client;
mod server;
mod ws_core;

fn main() {
    client::hand_shake(String::from("/chat"), String::from("www.rswebsocket.com"));
    ws_core::base64::encode("a".as_bytes());
}
