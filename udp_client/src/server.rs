use std::net::UdpSocket;

pub fn run_server(bind_addr: &str) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(bind_addr)?;
    println!("Server listening on {bind_addr}");

    let mut buf = [0u8; 1024];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let mut data = buf[..amt].to_vec();
        data.reverse();
        socket.send_to(&data, src)?;
    }
}
