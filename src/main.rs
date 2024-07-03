use std::{
    io::{Read, Write},
    net::TcpStream,
};

use messages::{authentication::Authentication, startup::StartupMessage};
use types::Connection;

mod messages;
mod types;

fn main() -> std::io::Result<()> {
    let connection = Connection::new("postgres", "postgres", 5432, "localhost", "postgres");
    let startup_message = StartupMessage {
        user: "postgres".to_string(),
        database: "postgres".to_string(),
        protocol_major_version: 3,
        protocol_minor_version: 0,
    };
    println!("Connection: {:?}", connection);
    println!("Startup message:  {:?}", startup_message);

    let mut stream = TcpStream::connect("127.0.0.1:5432")?;

    // send startup message
    let mut buf = [0; 1024];
    stream.write(&startup_message.to_bytes());
    stream.read(&mut buf);

    let auth_resp = Authentication::from(buf[1..].to_vec());
    println!("Auth resp: {:?}", auth_resp);

    Ok(())
}
