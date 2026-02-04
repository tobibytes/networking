mod client;
mod server;

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "server".to_string());

    match mode.as_str() {
        "server" => {
            let address = "127.0.0.1:4003".to_string();
            if let Err(err) = server::run_server(&address) {
                eprintln!("Server error: {err}");
            }
        }
        "client" => {
            let address = "127.0.0.1:4003".to_string();
            let message =  "hello".to_string();
            if let Err(err) = client::run_client(&address, message.as_bytes()) {
                eprintln!("Client error: {err}");
            }
        }
        _ => {
            eprintln!(
                "Usage:\n  udp_client server [bind_addr]\n  udp_client client [server_addr] [message]"
            );
        }
    }
}
