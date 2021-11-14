use clap::{App, Arg};
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
    let args = App::new("echosystem")
        .version("1")
        .author("Adrian M Ryan")
        .arg(Arg::new("echoer")
            .short('e')
            .long("echo_port")
            .takes_value(true),
        )
        .arg(Arg::new("yodelers")
            .short('y')
            .long("yodel_ips")
            .multiple_values(true),
        )
        .get_matches();

    let echo_port: i64 = args.value_of_t("echoer").unwrap_or(8000);

    let yodel_ips: Vec<&str> = args.values_of_t::<&str>("yodelers").unwrap_or(vec!("localhost:8000"));

    for a in yodel_ips {
        let a = a.clone();
        std::thread::spawn(move || {
            println!("Connecting to {}", a);
            let mut stream = TcpStream::connect(a).unwrap();
            dbg!(&stream);
            let packet = br#"{"foo": "bar"}"#;
            loop {
                stream.write_all(packet).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    let listener = TcpListener::bind(format!("0.0.0.0:{}", echo_port))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        std::thread::spawn(move || {
            listen_to_connection(stream.unwrap());
        });
    }
    Ok(())
}
