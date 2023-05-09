use openssl::symm::{Cipher, Crypter, Mode};

pub struct Encryption {
    cipher: Cipher,
    key: Vec<u8>,
}

impl Encryption {
    pub fn new(key: Vec<u8>) -> Self {
        Encryption {
            cipher: Cipher::aes_256_cbc(),
            key,
        }
    }

    pub fn encrypt(&self, data: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut crypter = Crypter::new(self.cipher, Mode::Encrypt, &self.key, Some(iv))?;
        let mut encrypted_data = vec![0; data.len() + self.cipher.block_size()];
        let mut count = crypter.update(data, &mut encrypted_data)?;
        count += crypter.finalize(&mut encrypted_data[count..])?;
        encrypted_data.truncate(count);
        Ok(encrypted_data)
    }

    pub fn decrypt(&self, data: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut crypter = Crypter::new(self.cipher, Mode::Decrypt, &self.key, Some(iv))?;
        let mut decrypted_data = vec![0; data.len() + self.cipher.block_size()];
        let mut count = crypter.update(data, &mut decrypted_data)?;
        count += crypter.finalize(&mut decrypted_data[count..])?;
        decrypted_data.truncate(count);
        Ok(decrypted_data)
    }
}

