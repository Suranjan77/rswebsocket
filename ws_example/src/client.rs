use std::sync::mpsc::{channel, Receiver, Sender};
use std::{io, thread};
use ws_client::client::WSClient;
use ws_core::data_frame_tx::{Agent, FrameType};
use ws_core::WSHandler;

struct ClientHandler {}

impl WSHandler for ClientHandler {
    fn who(&self) -> Agent {
        Agent::Client
    }

    fn handle_text_msg(&self, msg: String) {
        println!("{}", msg);
    }

    fn handle_bin_msg(&self, msg: Vec<u8>) {
        println!("{}", msg.len());
    }
}

pub fn client() {
    let mut client = WSClient::connect("http://127.0.0.1:8080", ClientHandler {}).unwrap();

    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

    let mut read_stream = client.ws_stream.clone();
    let r_handle = thread::spawn(move || loop {
        if let Err(e) = read_stream.read() {
            panic!("{e}");
        }
    });

    let mut write_stream = client.ws_stream.clone();
    let w_handle = thread::spawn(move || loop {
        if let Ok(msg) = rx.recv() {
            write_stream.write(&msg, FrameType::Text).unwrap()
        }
    });

    let io_handle = thread::spawn(move || loop {
        let mut inp = String::new();
        println!("Enter msg, (prefix /msg) > ");
        let cont = match io::stdin().read_line(&mut inp) {
            Ok(_) => {
                let inp_split: Vec<String> = inp.splitn(2, ' ').map(|s| s.to_string()).collect();

                if inp_split[0] == "/msg" {
                    tx.send(inp_split[1].as_bytes().to_vec()).unwrap();
                    true
                } else {
                    println!("Shutting down client ...");
                    false
                }
            }
            Err(_) => {
                println!("Error getting input from user");
                false
            }
        };

        if !cont {
            break;
        }
    });

    client.ws_stream.shutdown("Shut down").unwrap();

    io_handle.join().unwrap();
    r_handle.join().unwrap();
    w_handle.join().unwrap();
}
