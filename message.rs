use std::time::{SystemTime, UNIX_EPOCH};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1v15, pkcs8::DecodePublicKey};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct Message {
    message_id: u64,
    channel_id: u64,
    is_response: bool,
    is_response_to: Option<u64>,
    is_broadcast: bool,
    body: String,
    attachments: Vec<Vec<u8>>,
    timestamp: u64,
    author: String,
    signature: Vec<u8>,
}

impl Message {
    pub fn new(
        channel_id: u64,
        is_response: bool,
        is_response_to: Option<u64>,
        is_broadcast: bool,
        body: String,
        attachments: Vec<Vec<u8>>,
        author: String,
        private_key: &RsaPrivateKey
    ) -> Result<Self, String> {
        // Validation code remains the same...
        
        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        
        let message_id = 0;

        // Create message data to sign
        let message_data = Self::create_message_data(
            message_id,
            channel_id,
            is_response,
            is_response_to,
            is_broadcast,
            &body,
            &attachments,
            timestamp,
            author
        );
        
        // Sign the message using pkcs1v15 padding
        let mut hasher = Sha256::new();
        hasher.update(&message_data);
        let hashed_data = hasher.finalize().to_vec();
        
        let signature = private_key.sign(
            pkcs1v15::Pkcs1v15Sign::new::<Sha256>(),
            &hashed_data
        ).map_err(|e| e.to_string())?;
        
        Ok(Self {
            message_id,
            channel_id,
            is_response,
            is_response_to,
            is_broadcast,
            body,
            attachments,
            timestamp,
            author,
            signature,
        })
    }
    
    // Rest of create_message_data remains the same...

    pub fn create_message_data(
        message_id: u64,
        channel_id: u64,
        is_response: bool,
        is_response_to: Option<u64>,
        is_broadcast: bool,
        body: &str,
        attachments: &[Vec<u8>],
        timestamp: u64,
        author: String,) -> Vec<u8> {
        let mut message_data = Vec::new();
        message_data.extend_from_slice(&message_id.to_be_bytes());
        message_data.extend_from_slice(&channel_id.to_be_bytes());
        message_data.push(is_response as u8);
        if let Some(response_to) = is_response_to {
            message_data.extend_from_slice(&response_to.to_be_bytes());
        } else {
            message_data.extend_from_slice(&0u64.to_be_bytes());
        }
        message_data.push(is_broadcast as u8);
        message_data.extend_from_slice(body.as_bytes());
        for attachment in attachments {
            message_data.extend_from_slice(attachment);
        }
        message_data.extend_from_slice(&timestamp.to_be_bytes());
        message_data.extend_from_slice(author.as_bytes());
        message_data
    }
    
    pub fn verify(&self, public_key: &RsaPublicKey) -> bool {
        let message_data = Self::create_message_data(
            self.message_id,
            self.channel_id,
            self.is_response,
            self.is_response_to,
            self.is_broadcast,
            &self.body,
            &self.attachments,
            self.timestamp,
            self.author
        );
        
        let mut hasher = Sha256::new();
        hasher.update(&message_data);
        let hashed_data = hasher.finalize().to_vec();
        
        public_key.verify(
            pkcs1v15::Pkcs1v15Sign::new::<Sha256>(),
            &hashed_data,
            &self.signature
        ).is_ok()
    }
    
    // Getters remain the same...

    pub fn encrypt_for_sending(&self, public_key: &RsaPublicKey, author_private_key: &RsaPrivateKey) -> String {
        // Encrypt body with PKCS1v15 padding
        let encrypted_body = public_key.encrypt(
            &mut OsRng,
            pkcs1v15::Pkcs1v15Encrypt,
            self.body.as_bytes()
        ).unwrap_or_default();
        
        // Encrypt attachments
        let encrypted_attachments: Vec<Vec<u8>> = self.attachments.iter()
            .map(|attachment| public_key.encrypt(
                &mut OsRng,
                pkcs1v15::Pkcs1v15Encrypt,
                attachment
            ).unwrap_or_default())
            .collect();
        
        // Encrypt timestamp
        let encrypted_timestamp = public_key.encrypt(
            &mut OsRng,
            pkcs1v15::Pkcs1v15Encrypt,
            &self.timestamp.to_be_bytes()
        ).unwrap_or_default();
        
        // Sign the message
        let message_bytes = self.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&message_bytes);
        let hashed_data = hasher.finalize().to_vec();
        
        let signature = author_private_key.sign(
            pkcs1v15::Pkcs1v15Sign::new::<Sha256>(),
            &hashed_data
        ).unwrap_or_default();
        
        // Format JSON output
        format!(
            "{{\"message_id\":{},\"channel_id\":{},\"is_response\":{},\"is_response_to\":{:?},\"is_broadcast\":{},\"body\":\"{}\",\"attachments\":{},\"timestamp\":\"{}\",\"author\":\"{}\",\"signature\":\"{}\"}}",
            self.message_id,
            self.channel_id,
            self.is_response,
            self.is_response_to,
            self.is_broadcast,
            hex::encode(encrypted_body),
            encrypted_attachments.len(),
            hex::encode(encrypted_timestamp),
            self.author,
            hex::encode(signature)
        )
    }
}
