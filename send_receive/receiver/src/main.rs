use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 512];
    loop{
        match stream.read(&mut buffer){
            Ok(0) => {
                println!("Connection closed by client");
                break;
            }

            Ok(n) => {
                //Display
                let received_message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", received_message);

                //write a response back
                //TODO
            }
            
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
    }
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    println!("Litening on 127.0.0.1:7070!");
    
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New client connected");
                thread::spawn(||{
                    handle_client(stream);
                });
            }

            Err(e) => {
                println!("Error! {}", e);
            }
        }
    }
}
