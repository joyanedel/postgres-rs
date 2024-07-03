use std::io::Read;

#[derive(Debug)]
pub struct StartupMessage {
    pub protocol_major_version: i16, // 16 most significant bits for major; 16 less significants for minor
    pub protocol_minor_version: i16,
    pub user: String,
    pub database: String,
}

// for 3 would be 0000_0000_0000_0011 0000_0000_0000_0000

impl StartupMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(40);

        // reserve space for prefix
        let offset = buf.len();
        buf.extend(&[0; 4]);

        // add version to buffer
        let protocol_version =
            protocol_version_to_i32(self.protocol_major_version, self.protocol_minor_version);
        buf.extend(protocol_version.to_be_bytes());

        // add username and database
        write_parameter(&mut buf, "user", &self.user);
        write_parameter(&mut buf, "database", &self.database);
        buf.push(0);

        // calculate the written size and wrote it in the protocol
        let size = (buf.len() - offset) as i32;
        buf[offset..(offset + 4)].copy_from_slice(&size.to_be_bytes());

        buf
    }
}

fn protocol_version_to_i32(major: i16, minor: i16) -> i32 {
    let major_bits = major as u16;
    let minor_bits = minor as u32;

    let combined = ((major_bits as u32) << 16) | (minor_bits as u32);

    combined as i32
}

fn write_parameter(buf: &mut Vec<u8>, name: &str, value: &str) {
    buf.extend(name.as_bytes());
    buf.push(0);
    buf.extend(value.as_bytes());
    buf.push(0);
}

#[cfg(test)]
mod tests {
    use super::protocol_version_to_i32;

    #[test]
    fn protocol_version_to_i32_add_correctly() {
        let major_version = 3_i16;
        let minor_version = 0_i16;

        let result = protocol_version_to_i32(major_version, minor_version);

        assert_eq!(result, 196608);
    }

    #[test]
    fn protocol_version_0_results_in_0() {
        let major_version = 0_i16;
        let minor_version = 0_i16;

        let result = protocol_version_to_i32(major_version, minor_version);

        assert_eq!(result, 0);
    }
}
