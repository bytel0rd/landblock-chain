use rand::rngs::OsRng;
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::ToRsaPrivateKey;
use serde::ser::{Serialize};

use super::super::utils::{Error, ErrorLevel};

#[derive(Debug, Clone)]
pub struct Wallet {

    private_key: RsaPrivateKey,

    public_key: RsaPublicKey,

}

impl Wallet {

    pub fn new (secret_key: String) -> Result<Self, Error> {
        let mut rng = OsRng;

        let bits = 2048;

        let generated_private_key = RsaPrivateKey::new(&mut rng, bits);

        if generated_private_key.is_err() {

          let error = Error::from_message_and_error(String::from("Failed to generate private key"), ErrorLevel::External);  

          return Err(error);
        }

        let private_key = generated_private_key.unwrap();

        let public_key = RsaPublicKey::from(&private_key);

        return Ok(Wallet {
            private_key,
            public_key,
        })
    }

    pub fn get_pem_key(&self) -> Result<zeroize::Zeroizing<String>, Error> {
        
        let pem = self.private_key.to_pkcs1_pem();

        if pem.is_err() {
           let pgk_error =  pem.unwrap_err().to_string();
           let error = Error::from_message(pgk_error);

           return Err(error);
        }

        return Ok(pem.unwrap());
    }

    pub fn get_private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    pub fn get_public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

}