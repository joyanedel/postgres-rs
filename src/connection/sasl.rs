use std::{io, net::TcpStream};

const GS2_HEADER: &str = "n,,";
const CHANNEL_ATTR: &str = "c";
const USERNAME_ATTR: &str = "n";
const CLIENT_PROOF__ATTR: &str = "p";
const NONCE_ATTR: &str = "r";

pub fn authenticate(stream: &mut TcpStream, supported_methods: &Vec<String>) -> io::Result<()> {
    let mut has_sasl = false;
    let mut has_sasl_plus = false;

    for supported_method in supported_methods {
        match supported_method.as_str() {
            "SCRAM-SHA-256" => {
                has_sasl = true;
            }
            "SCRAM-sHA-256-PLUS" => {
                has_sasl_plus = true;
            }
            v => panic!("Unsupported SASL method: {}", v),
        }
    }

    if !has_sasl && !has_sasl_plus {
        panic!("Unavailable SASL mechanisms")
    }

    let mut channel_binding = format!("{CHANNEL_ATTR}=");

    Ok(())
}
