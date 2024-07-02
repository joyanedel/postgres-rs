use std::str::Bytes;

#[derive(Debug)]
pub struct Connection {
    username: String,
    password: String,
    port: u32,
    host: String,
    database: String,
}

impl Connection {
    pub fn new(username: &str, password: &str, port: u32, host: &str, database: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            port,
            host: host.to_string(),
            database: database.to_string(),
        }
    }
}
