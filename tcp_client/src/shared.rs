use std::{io::{Read, Write}, net::TcpStream};
pub const READ_TIMEOUT: u64 = 5;

pub struct TcpMessage {
    pub length: u8,
    pub payload: Vec<u8>
}

impl TcpMessage {
    pub fn new(length: u8, payload: Vec<u8>) -> Result<Self, String> {
        if payload.len() != length as usize {
            return Err(format!(
                "Payload length mismatch: declared {}, got {}",
                length,
                payload.len()
            ));
        }
        Ok(TcpMessage { length, payload })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut arr = vec![0; self.length as usize + 1];
        arr[0] = self.length;
        if !self.payload.is_empty() {
            arr[1..].copy_from_slice(&self.payload);
        }
        arr
    }
}

fn read_first_byte(sock: &mut TcpStream) -> std::io::Result<u8> {
    let mut buf = [0];
    sock.read_exact(&mut buf[..])?;
    Ok(buf[0])
}

pub fn read_from_socket(sock: &mut TcpStream) -> std::io::Result<TcpMessage> {
    let length = read_first_byte(sock)?;
    
    let payload = if length == 0 {
        Vec::new()
    } else {
        let mut buf = vec![0; length as usize];
        sock.read_exact(&mut buf[..])?;
        buf
    };
    
    Ok(TcpMessage { length, payload })
}

pub fn write_to_socket(sock: &mut TcpStream, message: &TcpMessage) -> std::io::Result<usize> {
    let buf = message.to_bytes();
    sock.write_all(&buf)?;
    Ok(buf.len())
}