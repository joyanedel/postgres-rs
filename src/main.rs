use std::{
    io::{Read, Write},
    net::TcpStream,
};

use connection::establish_connection;
use types::Connection;

mod connection;
mod messages;
mod types;

fn main() -> std::io::Result<()> {
    let connection = Connection::new("postgres", "postgres", 5432, "localhost", "postgres");
    println!("Connection: {:?}", connection);

    let mut stream = TcpStream::connect("127.0.0.1:5432")?;

    establish_connection(&mut stream)?;

    Ok(())
}
