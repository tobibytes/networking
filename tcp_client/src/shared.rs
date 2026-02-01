use std::{io::{Read, Write}, net::TcpStream};
const READ_TIMEOUT: u64 = 5;
pub struct TcpMessage {
    length: u8,
    payload: Vec<u8>
}

impl TcpMessage {
    pub fn new(length: u8, payload: Vec<u8>) -> Self {
        TcpMessage { length, payload }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut arr = vec![0; self.length as usize];
        arr[0] = self.length;
        arr[1..].copy_from_slice(&self.payload);
        arr
    }
}

fn read_first_byte(mut sock: &TcpStream) -> u8 {
    let mut buf = [0];
    match sock.read(&mut buf[..]) {
        Ok(_size) => buf[0],
        Err(err) => {
            panic!("Could not read the first byte: {:?}", err);
        }
    }
}
pub fn read_from_socket(mut sock: &TcpStream) -> TcpMessage {
    sock.set_read_timeout(Some(std::time::Duration::from_secs(READ_TIMEOUT))).unwrap();
    let length = read_first_byte(sock);
    let mut buf = vec![0; length as usize];
    match sock.read(&mut buf[..]) {
        Ok(size) => size,
        Err(err) => {
            panic!("Could not read from this source: {:?}", err);
        }
    };
    TcpMessage { length, payload: buf[1..].to_vec()}
}


pub fn write_to_socket(mut client_con: &TcpStream, message: TcpMessage) -> usize {
    let buf = &message.to_bytes();
    match client_con.write(buf) {
        Ok(bytes_written) => bytes_written,
        Err(err) => {
            panic!("Could not write bytes: {:?}", err);
        }
    }
}