use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::aes::{self, Iv};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMsg(pub Vec<u8>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMsgWrapper {
    iv: Iv,
    encrypted_msg: EncryptedMsg,
}

impl EncryptedMsgWrapper {
    pub fn decrypt(&self, aes_key: &aes::Key) -> Result<Vec<u8>> {
        aes::decrypt(&self.encrypted_msg, &aes_key, &self.iv)
    }

    pub fn decrypt_str(&self, aes_key: &aes::Key) -> Result<String> {
        let decrypted_data = self.decrypt(aes_key)?;
        let decrypted_str = String::from_utf8(decrypted_data)?;

        Ok(decrypted_str)
    }
}
