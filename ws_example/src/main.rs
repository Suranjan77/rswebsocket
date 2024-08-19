use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};
use ws_client::client::WSClientStream;
use ws_core::data_frame_tx::{Agent, FrameType};
use ws_core::{WSHandler, WSStream};
use ws_server::server::WSServerListener;

struct ClientHandler {}

struct ServerHandler {}

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

fn main() {
    println!("Starting server and client");
    let (c_tx, server_handle) = start_server();
    thread::sleep(Duration::from_secs(2));
    // let (c_tx, client_handle) = start_client();
    // thread::sleep(Duration::from_secs(2));

    loop {
        let mut inp = String::new();
        println!("Enter msg, (prefix /msg) > ");
        let cont = match io::stdin().read_line(&mut inp) {
            Ok(_) => {
                let inp_split: Vec<String> = inp.splitn(2, ' ').map(|s| s.to_string()).collect();

                if inp_split[0] == "/msg" {
                    c_tx.send(inp_split[1].as_bytes().to_vec()).unwrap();
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

    server_handle.join().unwrap();
    // client_handle.join().unwrap();
}

fn start_client() -> (Sender<Vec<u8>>, JoinHandle<()>) {
    let (c_tx, c_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    let main_client_thread = thread::spawn(move || {
        let mut client =
            WSClientStream::connect("ws://127.0.0.1:8080", ClientHandler {}).unwrap();

        let mut c1 = client.clone();
        let client_read_thread = thread::spawn(move || loop {
            c1.read().unwrap();
        });


        let client_write_thread = thread::spawn(move || loop {
            match c_rx.recv() {
                Ok(d) => client.write(&d[..], FrameType::Text).unwrap(),
                Err(e) => println!("Error in c_rx, {:?}", e),
            }
        });

        println!("Client connected");

        client_write_thread.join().unwrap();
        client_read_thread.join().unwrap();
    });
    (c_tx, main_client_thread)
}

fn start_server() -> (Sender<Vec<u8>>, JoinHandle<()>) {
    let (s_tx, s_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    let main_server_thread = thread::spawn(move || {
        let mut server = WSServerListener::init(8080u16, ServerHandler {}).unwrap();

        let mut s1 = server.clone();
        let server_read_thread = thread::spawn(move || loop {
            s1.read().unwrap();
        });

        let server_write_thread = thread::spawn(move || loop {
            match s_rx.recv() {
                Ok(d) => {
                    server
                        .write(&d[..], FrameType::Text).unwrap();
                }
                Err(e) => println!("Error in s_rx, {:?}", e),
            }
        });

        server_write_thread.join().unwrap();
        server_read_thread.join().unwrap();
    });
    (s_tx, main_server_thread)
}
