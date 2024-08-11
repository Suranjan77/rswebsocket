use ws_client::client::WSClientStream;
use ws_core::data_frame_tx::Agent;
use ws_core::WSHandler;
use ws_server::server::WSServerListener;

struct ClientHandler {}

struct ServerHandler {}

impl WSHandler for ClientHandler {
    fn who(&self) -> Agent {
        Agent::Client
    }

    fn handle_text_msg(&self, msg: String) {
        todo!()
    }

    fn handle_bin_msg(&self, msg: Vec<u8>) {
        todo!()
    }
}

impl WSHandler for ServerHandler {
    fn who(&self) -> Agent {
        Agent::Server
    }

    fn handle_text_msg(&self, msg: String) {
        todo!()
    }

    fn handle_bin_msg(&self, msg: Vec<u8>) {
        todo!()
    }
}

fn main() {
    let client = WSClientStream::connect("127.0.0.1:8080", ClientHandler {}).unwrap();

    let server = WSServerListener::init(8080u16, ServerHandler {}).unwrap();
}
