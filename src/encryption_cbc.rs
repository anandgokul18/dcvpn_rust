use crate::encryption::PacketEncrypter;
use aes_soft::block_cipher::generic_array::GenericArray;
use aes_soft::block_cipher::{BlockCipher, NewBlockCipher};
use aes_soft::Aes128, Aes192, Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::FromHex;
use std::error::Error;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;
type Aes192Cbc = Cbc<Aes192, Pkcs7>;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub enum AesCbc {
    Aes128(Aes128Cbc),
    Aes192(Aes192Cbc),
    Aes256(Aes256Cbc),
}

impl AesCbc {
    pub fn new(key: &str) -> Result<Box<dyn PacketEncrypter>, Box<dyn Error>> {
        if key.is_empty() {
            return Err("key is empty".into());
        }

        let bkey = Vec::from_hex(key)?;

        match bkey.len() {
            16 => {
                let cipher = Aes128::new(GenericArray::from_slice(&bkey));
                Ok(Box::new(AesCbc::Aes128(Aes128Cbc::new(cipher, GenericArray::default()))))
            }
            24 => {
                let cipher = Aes192::new(GenericArray::from_slice(&bkey));
                Ok(Box::new(AesCbc::Aes192(Aes192Cbc::new(cipher, GenericArray::default()))))
            }
            32 => {
                let cipher = Aes256::new(GenericArray::from_slice(&bkey));
                Ok(Box::new(AesCbc::Aes256(Aes256Cbc::new(cipher, GenericArray::default()))))
            }
            _ => Err("Length of key must be 16, 24 or 32 bytes
                    (32, 48 or 64 hex symbols)
                    to select AES-128, AES-192 or AES-256"
                .into()),
        }
    }
}

impl PacketEncrypter for AesCbc {
    fn encrypt(&self, input: &[u8], output: &mut [u8], iv: &[u8]) -> usize {
        match self {
            AesCbc::Aes128(cipher) => cipher.encrypt(iv.into(), output, input).len(),
            AesCbc::Aes192(cipher) => cipher.encrypt(iv.into(), output, input).len(),
            AesCbc::Aes256(cipher) => cipher.encrypt(iv.into(), output, input).len(),
        }
    }

    fn decrypt(&self, input: &[u8], output: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        let result = match self {
            AesCbc::Aes128(cipher) => cipher.decrypt(output, input),
            AesCbc::Aes192(cipher) => cipher.decrypt(output, input),
            AesCbc::Aes256(cipher) => cipher.decrypt(output, input),
        }?;
        Ok(result.len())
    }

   
    fn check_size(&self, size: usize) -> bool {
        size > 16 && size % 16 == 0
    }

    fn adjust_input_size(&self, size: usize) -> usize {
        if size % 16 != 0 {
            size + (16 - (size % 16))
        } else {
            size
        }
    }

    fn output_add(&self) -> usize {
        // adding IV to each message
        16
    }

    fn iv_len(&self) -> usize {
        16
    }
}

