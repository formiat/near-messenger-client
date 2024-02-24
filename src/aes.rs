use std::fs;

use aes::Aes256;
use anyhow::{anyhow, Result};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::encrypted_msg::EncryptedMsg;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Initialization Vector
///
/// Notes:
/// - You must use the same IV when you encrypt and when you decrypt
/// - Since the IV doesn't need to be kept secret, it can be safely transmitted with the ciphertext
/// - You should generate a new IV for each new encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iv(Vec<u8>);

impl Iv {
    fn generate() -> Self {
        let mut rng = thread_rng();

        // AES block size is 16 bytes
        let mut iv = vec![0u8; 16];
        rng.fill(&mut iv[..]);

        Self(iv)
    }

    fn save_to_file(&self, path: &str) -> Result<()> {
        let iv_hex = hex::encode(&self.0);

        Ok(fs::write(path, iv_hex)?)
    }

    fn load_from_file(path: &str) -> Result<Self> {
        let iv_hex = fs::read_to_string(path)?;
        let iv = hex::decode(iv_hex.trim())?;

        Ok(Self(iv))
    }
}

#[derive(Debug, Clone)]
pub struct Key(Vec<u8>);

impl Key {
    pub fn load_from_hex_string(hex_string: &str) -> Result<Self> {
        let key = hex::decode(hex_string.trim())?;

        Ok(Self(key))
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let key_hex = fs::read_to_string(path)?;

        Self::load_from_hex_string(&key_hex)
    }
}

#[allow(unused)]
fn example() {
    let key = Key::load_from_file("path/to/your/aes-256.key").expect("Failed to read key");
    let data = "Hello, world!";

    // Path to save or load the IV
    let iv_path = "path/to/your/iv.txt";

    // Generate IV
    let iv = Iv::generate();
    iv.save_to_file(iv_path).expect("Failed to save IV");

    // Encrypt
    let encrypted_data = encrypt(data.as_bytes(), &key, &iv).expect("Encryption failed");
    println!("Encrypted data: {:?}", encrypted_data);

    // Load IV from file for decryption
    let iv_loaded = Iv::load_from_file(iv_path).expect("Failed to load IV");

    // Decrypt
    let decrypted_data = decrypt_str(&encrypted_data, &key, &iv_loaded).expect("Decryption failed");
    println!("Decrypted data: {:?}", decrypted_data);
}

fn encrypt(data: &[u8], key: &Key, iv: &Iv) -> Result<EncryptedMsg> {
    let cipher = Aes256Cbc::new_from_slices(&key.0, &iv.0)?;
    let encrypted_data = cipher.encrypt_vec(data);

    Ok(EncryptedMsg(encrypted_data))
}

pub fn decrypt(encrypted_data: &EncryptedMsg, key: &Key, iv: &Iv) -> Result<Vec<u8>> {
    let cipher = Aes256Cbc::new_from_slices(&key.0, &iv.0)?;
    let decrypted_data = cipher.decrypt_vec(&encrypted_data.0)?;

    Ok(decrypted_data)
}

fn decrypt_str(encrypted_data: &EncryptedMsg, key: &Key, iv: &Iv) -> Result<String> {
    let decrypted_data = decrypt(encrypted_data, key, iv).map_err(|e| anyhow!(e))?;
    let decrypted_str = String::from_utf8(decrypted_data)?;

    Ok(decrypted_str)
}
