use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::packet::IPPacket;

mod encryption_cbc;
mod encryption_none;

#[derive(Debug, Clone)]
pub struct PacketTooSmallError;
#[derive(Debug, Clone)]
pub struct NonIPv4PacketError;
#[derive(Debug, Clone)]
pub struct PacketInvalidSizeError;

impl Error for PacketTooSmallError {}
impl Error for NonIPv4PacketError {}
impl Error for PacketInvalidSizeError {}

impl fmt::Display for PacketTooSmallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Packet too small")
    }
}

impl fmt::Display for NonIPv4PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Non IPv4 packet")
    }
}

impl fmt::Display for PacketInvalidSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stored packet size bigger than packet itself")
    }
}

pub trait PacketEncrypter {
    fn encrypt(&self, input: &[u8], output: &mut [u8], iv: &[u8]) -> usize;
    fn decrypt(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Box<dyn Error>>;
    fn check_size(&self, size: usize) -> bool;
    fn adjust_input_size(&self, size: usize) -> usize;
    fn output_add(&self) -> usize;
    fn iv_len(&self) -> usize;
}

type NewEncrypterFunc = fn(String) -> Result<Box<dyn PacketEncrypter>, Box<dyn Error>>;

pub struct EncryptionRegistry {
    registered_encrypters: HashMap<String, NewEncrypterFunc>,
}

impl EncryptionRegistry {
    fn new() -> Self {
        let mut registry = EncryptionRegistry {
            registered_encrypters: HashMap::new(),
        };
        // registry.register("none", encryption_none::EncNone::new);
        registry.register("aescbc", encryption_cbc::AesCbc::new);
        registry
    }

    pub fn decrypt_v4_chk(
        &self,
        e: &dyn PacketEncrypter,
        src: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, Box<dyn Error>> {
        let num = e.decrypt(src, dst)?;

        if num < 22 {
            return Err(Box::new(PacketTooSmallError));
        }

        let packet = IPPacket::new(dst);
        if packet.ip_ver() != 4 {
            return Err(Box::new(NonIPv4PacketError));
        }

        let size = packet.get_size();
        if size > num {
            return Err(Box::new(PacketInvalidSizeError));
        }

        Ok(size)
    }
}

