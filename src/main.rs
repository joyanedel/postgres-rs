use std::{io::Write, net::TcpStream};

use messages::startup::StartupMessage;
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
    println!("Bytes startup: {:?}", startup_message.to_bytes());

    let mut stream = TcpStream::connect("127.0.0.1:5432")?;
    let result = stream.write(&startup_message.to_bytes());

    println!("Result: {:?}", result);

    Ok(())
}
