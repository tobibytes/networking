use std::{net::{SocketAddr, TcpListener, TcpStream}, thread};
use crate::shared;
pub fn start_server(address: &str) {
    let server = match TcpListener::bind(address) {
        Ok(_server) => _server,
        Err(err) => panic!("Could not bind to address {}: {:?}", address, err)
    };
    println!("Started listening on address {}", address);
    loop {
        match server.accept() {
            Ok((sock, addr)) => {
                thread::spawn(move || {
                handle_connection(sock, addr);
                })
            }
            Err(err) => {
                close_socket(server, err);
                break;
            }
        };
    }

}
fn handle_connection(sock: TcpStream, _addr: SocketAddr) {
    let read_buffer= shared::read_from_socket(&sock);
    println!("read buffer: {:?}", read_buffer.to_bytes());
    let send_message = shared::TcpMessage::new(10, Vec::from([5, 10, 20, 50, 7, 8, 2, 4, 9]));
    let sent_data = shared::write_to_socket(&sock, send_message);
    println!("write buffer: {:?}", sent_data);

}

fn close_socket(server: TcpListener, err: std::io::Error ) {
    drop(server);
    panic!("Could not accept client connection: {:?}", err);
}