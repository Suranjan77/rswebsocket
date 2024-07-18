use rand::RngCore;
use crate::ws_core;

pub fn hand_shake(req_uri: String, host: String) {
    let mut handshake: String = String::from("");
    // line 1
    handshake.push_str("GET ");
    handshake.push_str(req_uri.as_str());
    handshake.push_str(" HTTP/1.1\n");

    handshake.push_str("Host: ");
    handshake.push_str(host.as_str());
    handshake.push('\n');
    handshake.push_str("Upgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Version: 13\n");

    // websocket key
    handshake.push_str("Sec-WebSocket-Key: ");
    handshake.push_str(ws_key().as_str());
    
    println!("{}", handshake);
}

fn ws_key() -> String {
    let mut nonce = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut nonce);
    ws_core::base64::encode(&nonce)
}

fn create_data_frame() {

}

fn send() {

}

fn receive() {

}

fn close_connection() {

}
