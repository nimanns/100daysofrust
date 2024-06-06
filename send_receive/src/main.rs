use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {


    let mut stream = TcpStream::connect("127.0.0.1:7070")?;
    
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    loop{

        let mut input = String::new();
        handle.read_line(&mut input)?;
        
        input.to_string().push_str("\n");
        let input = input.trim();

        if input == "exit" {
            println!("Exiting");
            break;
        }

        stream.write_all(input.as_bytes())?;

        println!("Sent");

        //Read the response
        //TODO

    }    

    Ok(())
}
