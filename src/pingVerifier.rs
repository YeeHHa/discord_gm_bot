use hex;
use core::sync;
use std::env;
use std::env::VarError;
use std::error::Error;
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
use serde_json;
use super::discord_data_structs;
pub struct PingVerifier {
    pub verifying_key: VerifyingKey
}

impl PingVerifier{
    pub fn new() -> PingVerifier {
        let discord_key = env::var("DISCORD_API_KEY")
            .expect("DISCORD_API_KEY env variable must be set for discord signature verification");

        let discord_key = discord_key.trim();

        let mut byte_array: [u8; PUBLIC_KEY_LENGTH] = [0; PUBLIC_KEY_LENGTH];
        
        hex::decode_to_slice(&discord_key, &mut byte_array)
            .expect("COULD NOT CONVERT API KEY TO BYTE ARRAY");

        let v_key: VerifyingKey = VerifyingKey::from_bytes(&byte_array)
            .expect("COULD NOT CONVERT BYTE ARRAY TO VERIFYING KEY");
        
        PingVerifier { 
            verifying_key: v_key 
        } 
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

        log::debug!("signature\n{}\npayload\n{}\n\t", sig, payload);

        match self.verifying_key.verify_strict(&payload.as_bytes(), &sig) {
            Ok(_) => true,
            Err(e) => {
                log::warn!("could not verify signature from ping request\n{:?}", e);
                false
            }
        }
    }

    pub fn prepare(&self, headers: &HeaderMap, body: &discord_data_structs::Interaction) -> Result<(String, String), Box<dyn Error + Send + Sync>> {

        let sig = match headers.get("X-Signature-Ed25519") {
            Some(s) => match s.to_str() {
                Ok(ss) => ss,
                Err(e) => {
                    log::error!("could not convert X-Signature-Ed25519 to String\n{}", e);
                    return Err("X-Signature-Ed25519 header convertion failure".into()) 
                }
            },
            None => {
                log::debug!("sig sign not found");
                return Err("X-Signature-Ed25519 header required".into()) 
            }
        };

        let time_stamp = match headers.get("X-Signature-Timestamp") {
            Some(t) => match t.to_str() {
                Ok(ts) => ts,
                Err(e) => {
                    log::error!("could not convert X-Signature-Timestamp to String\n{}", e);
                    return Err("X-Signature-Timestamp convertion failure".into()) 
                }
            },
            None => {
                log::debug!("time time not found");
                return Err("X-Signature-Timestamp header required".into())
            }
        };

        let body_string: String = match serde_json::to_string(body) {
            Ok(val) => val,
            Err(e) => {
                log::debug!("coudln't convert json body to string"); 
                return Err("couldn't convert json body to string".into()); 
            }
        };

        let payload = format!("{}{}",time_stamp, body_string );
        let sig = sig.to_string();

        Ok((payload, sig))
    }
}