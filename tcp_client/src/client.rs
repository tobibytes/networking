use std::{net::TcpStream, io};
mod shared;

fn start_connect(address: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(address)?;
    handle_connection(&mut stream)?;
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> io::Result<()> {
    let messages = vec![
        (5, vec![0, 1, 2, 3, 4]),
        (0, vec![]),  // Empty payload
        (3, vec![10, 20, 30]),
    ];
    
    for (length, payload) in messages {
        let msg = shared::TcpMessage::new(length, payload)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        let bytes_written = shared::write_to_socket(stream, &msg)?;
        println!("Sent {} bytes", bytes_written);
        
        let reply = shared::read_from_socket(stream)?;
        println!("Received: length={}, payload={:?}", reply.length, reply.payload);
    }
    
    Ok(())
}

fn main() {
    let address = "127.0.0.1:4002";
    
    match start_connect(address) {
        Ok(_) => println!("Connection successful"),
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}


