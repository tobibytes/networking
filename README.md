# networking

Two small networking exercises in Rust, written against the standard library only — no async runtime, no third-party deps. Focus is on framing and the OS-level socket APIs.

## `tcp_client/` — length-prefixed TCP echo server + client

A multi-threaded TCP server that speaks a tiny length-prefixed protocol:

```
| length: u8 | payload: length bytes |
```

- `server` accepts connections, spawns a thread per client, reads the 1-byte length, then exactly that many payload bytes, and echoes the frame back.
- `client` connects, sends a sequence of frames (including an empty payload), and reads each echo back.
- 5-second read timeout per connection; clean disconnect handling for `UnexpectedEof` and `TimedOut`.
- Framing logic lives in `shared.rs` (`TcpMessage::new` validates that the declared length matches the payload).

```sh
cd tcp_client
cargo run --bin tcp_client      # starts server on 127.0.0.1:4002
# in another shell, switch the mod in main.rs to use client::, or run the client binary
```

## `udp_client/` — UDP reverse-echo server + client

A datagram server that receives a message, reverses the bytes, and sends them back.

```sh
cd udp_client
cargo run -- server     # bind 127.0.0.1:4003
cargo run -- client     # send "hello", receive "olleh"
```

## What this is

Hands-on practice with:

- `TcpListener` / `TcpStream` and `UdpSocket` from `std::net`
- Manual length-prefix framing over a byte stream
- OS-thread-per-connection concurrency
- I/O error handling (`io::ErrorKind` matching, read timeouts)

## Stack

Rust 2024 · `std::net` · `std::thread` · zero deps
