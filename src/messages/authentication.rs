use core::panic;

#[derive(Debug, PartialEq)]
pub enum Authentication {
    Ok,
    KerberosV5,
    CleartextPassword,
    // Salt Bytes 4
    MD5Password([u8; 4]),
    GSS,
    SSPI,
    // Bytes n
    GSSContinue(Vec<u8>),
    // Name of SASL auth mech
    SASL(Vec<String>),
    // Bytes n
    SASLContinue(Vec<u8>),
    // Bytes n
    SASLFinal(Vec<u8>),
    NegotiateProtocolVersion,
}

impl From<Vec<u8>> for Authentication {
    fn from(value: Vec<u8>) -> Self {
        let length = u32::from_be_bytes(value[..4].try_into().unwrap()) as usize;
        let auth_type: i32 = i32::from_be_bytes(value[4..8].try_into().unwrap());
        match auth_type {
            0 => Self::Ok,
            2 => Self::KerberosV5,
            3 => Self::CleartextPassword,
            5 => {
                let salt: [u8; 4] = value[8..12].try_into().unwrap();
                Self::MD5Password(salt)
            }
            7 => Self::GSS,
            8 => {
                let gssapi = value[8..].to_vec();
                Self::GSSContinue(gssapi)
            }
            9 => Self::SSPI,
            10 => {
                // Self::SASL
                let offset: usize = 8;
                let methods = String::from_utf8(value[offset..length - 2].to_vec()).unwrap();
                let methods = methods.split('\0').map(|v| v.to_owned()).collect();

                Self::SASL(methods)
            }
            11 => {
                // Self::SASLContinue
                unimplemented!("SASLContinue")
            }
            12 => {
                // Self::SASLFinal
                unimplemented!("SASLFinal")
            }
            _ => panic!("Error"),
        }
    }
}
