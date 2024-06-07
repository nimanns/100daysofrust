use std::net::UdpSocket;

pub fn start_server() {
    let socket = UdpSocket::bind("127.0.0.1:7878").expect("Could not bind to address");

    println!("Server listening on 127.0.0.1:7878");

    let mut buffer = [0; 512];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                println!("Received message from {}: {}", src, String::from_utf8_lossy(&buffer[..size]));
                socket.send_to(&buffer[..size], &src).expect("Failed to send response");
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}
