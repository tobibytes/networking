use std::net::UdpSocket;

pub fn run_client(server_addr: &str, message: &[u8]) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;
    socket.send(message)?;
    let mut buf = [0u8; 1024];
    let amt = socket.recv(&mut buf)?;
    let reply = &buf[..amt];
    println!("Received: {}", String::from_utf8_lossy(reply));
    Ok(())
}