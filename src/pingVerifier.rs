use hex;
use std::env;
use std::env::VarError;
use axum::http::{
    HeaderName,
    HeaderValue,
    HeaderMap
};
use ed25519_dalek::{
    Verifier, 
    VerifyingKey,
    Signature,
    SIGNATURE_LENGTH, 
    PUBLIC_KEY_LENGTH
};


pub struct PingVerifier {
    pub verifying_key: VerifyingKey
}

impl PingVerifier{
    pub fn new() -> PingVerifier {
        let discord_Key = env::var("DISCORD_API_KEY")
            .expect("DISCORD_API_KEY env variable must be set for discord signature verification");

        let mut byte_array: [u8; PUBLIC_KEY_LENGTH] = [0; PUBLIC_KEY_LENGTH];
        
        hex::decode_to_slice(&discord_Key, &mut byte_array)
            .expect("COULD NOT CONVERT API KEY TO BYTE ARRAY");

        let v_key: VerifyingKey = VerifyingKey::from_bytes(&byte_array)
            .expect("COULD NOT CONVERT BYTE ARRAY TO VERIFYING KEY");
        PingVerifier { verifying_key: v_key } 
    }

    pub fn verify(&self, payload: &str, signature: &str) -> bool {
        let mut sig_byte_array: [u8; SIGNATURE_LENGTH] = [0; SIGNATURE_LENGTH]; 
        match hex::decode_to_slice(&signature, &mut sig_byte_array) {
            Ok(()) => log::info!("successfully decoded provided X-Signature-Ed25519"),
            Err(e) => {
                log::error!("could not decode provide X-Signature-Ed25519\n{}\n{}", signature, e);
                return false
            }
        }
        
        let sig: Signature =  Signature::from_bytes(&sig_byte_array); 

        match self.verifying_key.verify(&payload.as_bytes(), &sig) {
            Ok(verified) => true,
            Err(e) => {
                log::warn!("could not verify signature from ping request\n{}", e);
                false
            }
        }
    }
}