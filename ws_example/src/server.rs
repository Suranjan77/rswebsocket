use std::sync::mpsc::{channel, Receiver, Sender};
use std::{io, thread};
use ws_core::data_frame_tx::{Agent, FrameType};
use ws_core::{ConnectionStatus, WSHandler};
use ws_server::server::WSServerListener;

struct ServerHandler {}

impl WSHandler for ServerHandler {
    fn who(&self) -> Agent {
        Agent::Server
    }

    fn handle_text_msg(&self, msg: String) {
        println!("{}", msg);
    }

    fn handle_bin_msg(&self, msg: Vec<u8>) {
        println!("{}", msg.len());
    }
}

pub fn server() {
    println!("Starting server and client");

    let server = WSServerListener::init(8080u16, ServerHandler {}).unwrap();
    for connection in server.listen() {
        if let Ok(mut client) = connection {
            let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();

            // Store the client in concurrent list or to handle multiple clients
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

            loop {
                let mut inp = String::new();
                println!("Enter msg, (prefix /msg) > ");
                let cont = match io::stdin().read_line(&mut inp) {
                    Ok(_) => {
                        let inp_split: Vec<String> =
                            inp.splitn(2, ' ').map(|s| s.to_string()).collect();

                        if inp_split[0] == "/msg" {
                            tx.send(inp_split[1].as_bytes().to_vec()).unwrap();
                            true
                        } else {
                            println!("Shutting down client and server ...");
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
            }

            r_handle.join().unwrap();
            w_handle.join().unwrap();
            client.ws_state = ConnectionStatus::Closed;
        }
    }
}
