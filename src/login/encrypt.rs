use aes_gcm::{Aes256Gcm, aead::{Aead, KeyInit, OsRng, Nonce}};
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::Key;
use rand::{RngCore, Rng};
use std::str;

use sha2::{Sha256, Digest};

pub struct Encrypt;
pub struct Decrypt;

pub struct Keys;

impl Keys {
    pub fn new() -> [u8; 32] {
        let mut rng = rand::thread_rng(); 
        let key: [u8; 32] = rng.gen();     
        key            
    }
}

impl Encrypt {
    pub fn sha256(plaintext: &String) -> String{
        let mut hasher = Sha256::new();
        hasher.update(plaintext);
    
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn aes(key: &[u8; 32], plaintext: &String) -> Option<(String, [u8; 12])> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        match cipher.encrypt(GenericArray::from_slice(&nonce), plaintext.as_bytes()) {
            Ok(ciphertext) => Some((base64::encode(&ciphertext), nonce)),
            Err(_) => None
        }
    }
}

impl Decrypt {
    pub fn aes(key: &[u8; 32], ciphertext: &String, nonce: &[u8; 12]) -> Option<String> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

        let ciphertext = match base64::decode(ciphertext) {
            Ok(data) => data,
            Err(_) => return None
        };

        match cipher.decrypt(GenericArray::from_slice(nonce), ciphertext.as_ref()) {
            Ok(plaintext) => {
                match String::from_utf8(plaintext) {
                    Ok(string) => Some(string),
                    Err(_) => None,
                }
            }
            Err(_) => None
        }
    }
}