use std::{net::{TcpListener, TcpStream}, thread};
use crate::shared;

pub fn start_server(address: &str) {
    let server = match TcpListener::bind(address) {
        Ok(_server) => _server,
        Err(err) => {
            eprintln!("Could not bind to address {}: {:?}", address, err);
            return;
        }
    };
    println!("Started listening on address {}", address);
    
    for result in server.incoming() {
        match result {
            Ok(sock) => {
                sock.set_read_timeout(Some(std::time::Duration::from_secs(shared::READ_TIMEOUT))).ok();
                thread::spawn(move || {
                    handle_connection(sock);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {:?}", err);
            }
        }
    }
}

fn handle_connection(mut sock: TcpStream) {
    let peer_addr = sock.peer_addr().ok();
    
    loop {
        match shared::read_from_socket(&mut sock) {
            Ok(message) => {
                println!(
                    "Received from {:?}: length={}, payload={:?}",
                    peer_addr, message.length, message.payload
                );
                
                // Echo back the message
                let response = match shared::TcpMessage::new(message.length, message.payload.clone()) {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Invalid message: {}", e);
                        break;
                    }
                };
                
                if let Err(e) = shared::write_to_socket(&mut sock, &response) {
                    eprintln!("Error writing response: {:?}", e);
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Client {:?} disconnected cleanly", peer_addr);
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Read timeout from {:?}", peer_addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading from {:?}: {:?}", peer_addr, e);
                break;
            }
        }
    }
}