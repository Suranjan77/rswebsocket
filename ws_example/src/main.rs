use std::env;
mod client;
mod server;
use crate::client::client;
use crate::server::server;

fn main() {
    let agent = env::var("agent").unwrap();

    if agent == "client" {
        client()
    } else if agent == "server" {
        server()
    }
}
