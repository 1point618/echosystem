use serde_json::{Deserializer, Value};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn listen_to_connection(stream: TcpStream) {
    dbg!(&stream);

    let json_objs = Deserializer::from_reader(stream).into_iter::<Value>();

    for value in json_objs {
        println!("{}", value.unwrap());
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    for a in &args[1..] {
        println!("Connecting to {}", a);
        let mut stream = TcpStream::connect(a)?;
        dbg!(&stream);
        let packet = br#"{"foo": "bar"}"#;
        loop {
            stream.write_all(packet)?;
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        listen_to_connection(stream?);
    }
    Ok(())
}
