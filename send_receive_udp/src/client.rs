use std::io::{self, Write};
use std::net::UdpSocket;

pub fn start_client() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind client socket");

    println!("Client ready to send messages to 127.0.0.1:7878");

    loop {
        let mut input = String::new();
        print!("Enter message to send to server: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        socket.send_to(input.as_bytes(), "127.0.0.1:7878").expect("Failed to send data");

        let mut buffer = [0; 512];
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                println!("Received from server {}: {}", src, String::from_utf8_lossy(&buffer[..size]));
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
                break;
            }
        }
    }
}
