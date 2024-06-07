mod server;
mod client;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => server::start_server(),
        "client" => client::start_client(),
        _=> {
            eprintln!("Usage: {} [server|client]", args[0]);
            std::process::exit(1);
        }
    }
}
