mod server;
// mod client;
mod shared;
fn main() {
    let address = "127.0.0.1:4002";
    server::start_server(address);
}

