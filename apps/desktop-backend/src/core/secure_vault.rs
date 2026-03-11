use keyring::Entry;
use std::fs;
use std::path::PathBuf;

pub struct SecureVault {
    entry: Option<Entry>,
    fallback_path: Option<PathBuf>,
}

impl SecureVault {
    pub fn new(service: &str, username: &str) -> Result<Self, String> {
        // Try to create keyring entry
        let entry = Entry::new(service, username).ok();
        
        // Set up fallback file path
        let fallback_path = dirs::data_dir()
            .map(|d| d.join("OpenMonkey").join(format!("{}.key", username)));
        
        Ok(Self { entry, fallback_path })
    }

    pub fn set_api_key(&self, api_key: &str) -> Result<(), String> {
        // Try keyring first
        if let Some(ref entry) = self.entry {
            match entry.set_password(api_key) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("Keyring set failed: {}, trying fallback", e);
                }
            }
        }
        
        // Fallback to file storage
        if let Some(ref path) = self.fallback_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            fs::write(path, api_key).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No storage method available".to_string())
        }
    }

    pub fn get_api_key(&self) -> Result<String, String> {
        // Try keyring first
        if let Some(ref entry) = self.entry {
            match entry.get_password() {
                Ok(key) => return Ok(key),
                Err(e) => {
                    eprintln!("Keyring get failed: {}, trying fallback", e);
                }
            }
        }
        
        // Fallback to file storage
        if let Some(ref path) = self.fallback_path {
            match fs::read_to_string(path) {
                Ok(key) => Ok(key),
                Err(_) => Err("API key not found".to_string()),
            }
        } else {
            Err("No storage method available".to_string())
        }
    }

    pub fn delete_api_key(&self) -> Result<(), String> {
        let mut success = true;
        
        // Try keyring
        if let Some(ref entry) = self.entry {
            if let Err(e) = entry.delete_credential() {
                eprintln!("Keyring delete failed: {}", e);
                success = false;
            }
        }
        
        // Also delete fallback file
        if let Some(ref path) = self.fallback_path {
            if path.exists() {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("Fallback file delete failed: {}", e);
                    success = false;
                }
            }
        }
        
        if success {
            Ok(())
        } else {
            Err("Failed to delete API key from all storage methods".to_string())
        }
    }
}
