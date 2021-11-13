use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    dbg!(stream);
}

fn main() -> std::io::Result<()> {
    let args:Vec<String> = std::env::args().collect();

    for a in &args[1..] {
        println!("Connecting to {}", a);
        let stream = TcpStream::connect(a);
        dbg!(stream);
    }

    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
