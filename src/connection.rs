use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::messages::{
    authentication::Authentication, format::MessageFormat, startup::StartupMessage,
};

mod sasl;

pub fn establish_connection(stream: &mut TcpStream) -> io::Result<()> {
    let startup_message = StartupMessage {
        user: "postgres".to_string(),
        database: "postgres".to_string(),
        protocol_major_version: 3,
        protocol_minor_version: 0,
    };
    let mut buf = [0; 4096];

    // send connection message
    stream.write(&startup_message.to_bytes())?;
    stream.read(&mut buf)?;

    let message_format = MessageFormat::decode(&buf);

    let auth_method = match message_format {
        MessageFormat::Authentication(v) => v,
        v => panic!("Message format not supported yet: {:?}", v),
    };

    match auth_method {
        Authentication::Ok => {}
        Authentication::SASL(v) => {
            sasl::authenticate(stream, &v);
        }
        v => panic!("Not implemented. Auth method: {:?}", v),
    };

    Ok(())
}
