use crate::encryption::PacketEncrypter;
use std::error::Error;

pub struct EncNone;

impl EncNone {
    pub fn new(_key: &str) -> Result<Box<dyn PacketEncrypter>, Box<dyn Error>> {
        Ok(Box::new(EncNone))
    }
}

impl PacketEncrypter for EncNone {
    fn encrypt(&self, input: &[u8], output: &mut [u8], _iv: &[u8]) -> usize {
        output.copy_from_slice(input);
        input.len()
    }

    fn decrypt(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        output.copy_from_slice(input);
        Ok(input.len())
    }

    fn check_size(&self, _size: usize) -> bool {
        true
    }

    fn adjust_input_size(&self, size: usize) -> usize {
        size
    }

    fn output_add(&self) -> usize {
        0
    }

    fn iv_len(&self) -> usize {
        0
    }
}

