use super::authentication::Authentication;

#[derive(Debug)]
pub enum MessageFormat {
    Authentication(Authentication),
}

impl MessageFormat {
    pub fn decode(buf: &[u8]) -> Self {
        let message_type = *buf.first().unwrap() as char;

        match message_type {
            'R' => {
                let auth_buf = buf[1..].to_vec();
                MessageFormat::Authentication(Authentication::from(auth_buf))
            }
            v => panic!("Unknown message. Char value: {}", v),
        }
    }
}
