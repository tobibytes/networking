use std::{net::TcpStream};
mod shared;
// use crate::shared;
// use shared;

fn start_connect(address: &str) {
    match TcpStream::connect(address) {
        Ok(_client_con) => {
            handle_connection(_client_con);
        },
        Err(err) => {
            panic!("Could not connect to socket: {:?}", err);
        }
    }
}
fn handle_connection(client_con: TcpStream) {
    println!("Connected to socket, Connection: {:?}", client_con);
    let first_message = shared::TcpMessage::new(5, Vec::from([0, 1, 2, 3]));
    let bytes_written = shared::write_to_socket(&client_con, first_message);
    println!("Bytes written: {}", bytes_written);
    let read_data = shared::read_from_socket(&client_con);
    println!("read data: {:?}", read_data.to_bytes());
    
}

fn main() {
    let address = "127.0.0.1:4002";
    let mut c = 100;
    while c > 0 {
        start_connect(&address);
        c -= 1
    }
}


