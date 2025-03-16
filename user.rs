use std::fmt;
use rsa::RsaPublicKey;

/// User represents a user in an online chat server
#[derive(Debug, Clone)]
pub struct User {
    /// Username, limited to 64 characters
    username: String,
    // Canonical Name, a randomly generated unicode string that is unique to each user. Exactly 64 characters long.
    canonical_name: String,
    /// Permission level (4-bit number, 0-15)
    permission_level: u8,
    /// RSA public key for secure communication
    public_key: RsaPublicKey,
}

// Constants for common permission levels
pub mod permissions {
    pub const GUEST: u8 = 0; //can only ask for permission to join
    pub const REGULAR: u8 = 1; //can send and retrieve messages
    pub const PREMIUM: u8 = 2; //can send and retrieve messages, and can send files
    pub const MODERATOR: u8 = 8; //can send and retrieve messages, can send files, and can ban, mute, or kick users
    pub const ADMIN: u8 = 15; //can do anything
}

impl User {
    /// Creates a new user
    pub fn new(username: &str, canonical_name: &str, permission_level: u8, public_key: RsaPublicKey) -> Result<Self, &'static str> {
        if username.len() > 64 {
            return Err("Username too long, maximum length is 64 characters");
        }

        if canonical_name.len() != 64 {
            return Err("Canonical Name must be exactly 64 characters long");
        }
        
        if permission_level > 15 {
            return Err("Permission level must be a 4-bit number (0-15)");
        }
        
        Ok(User {
            username: username.to_string(),
            canonical_name: canonical_name.to_string(),
            permission_level,
            public_key,
        })
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }
    
    pub fn permission_level(&self) -> u8 {
        self.permission_level
    }
    
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }
    
    pub fn set_username(&mut self, username: &str) -> Result<(), &'static str> {
        if username.len() > 64 {
            return Err("Username too long, maximum length is 64 characters");
        } else if username.is_empty() {
            return Err("Username cannot be empty");
        }
        self.username = username.to_string();
        Ok(())
    }
    
    pub fn set_permission_level(&mut self, permission_level: u8) -> Result<(), &'static str> {
        if permission_level > 15 {
            return Err("Permission level must be a 4-bit number (0-15)");
        }
        self.permission_level = permission_level;
        Ok(())
    }
    
    pub fn has_permission(&self, required_level: u8) -> bool {
        self.permission_level >= required_level
    }

}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_name = match self.permission_level {
            permissions::GUEST => "Guest",
            permissions::REGULAR => "Regular",
            permissions::PREMIUM => "Premium",
            permissions::MODERATOR => "Moderator",
            permissions::ADMIN => "Admin",
            _ => "Custom",
        };
        
        write!(f, "User '{}' with permission level {} ({})", 
               self.username, self.permission_level, level_name)
    }
}