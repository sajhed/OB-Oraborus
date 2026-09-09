use crate::error::{ObError, ObResult};
use keyring::Entry;
use zeroize::Zeroizing;

#[derive(Clone, Default)]
pub struct SecretVault;
impl SecretVault {
    fn entry(provider_id: &str) -> ObResult<Entry> { Entry::new("OB Oraborus", provider_id).map_err(|error| ObError::Internal(format!("Credential store initialization failed: {error}"))) }
    pub fn set(&self, provider_id: &str, value: &str) -> ObResult<()> {
        if value.trim().is_empty() { return Err(ObError::Validation("Secret cannot be empty".into())); }
        Self::entry(provider_id)?.set_password(value).map_err(|error| ObError::Internal(format!("Windows credential storage failed: {error}")))
    }
    pub fn get(&self, provider_id: &str) -> ObResult<Zeroizing<String>> {
        let secret = Self::entry(provider_id)?.get_password().map_err(|_| ObError::NotConnected(format!("No credential stored for {provider_id}")))?;
        Ok(Zeroizing::new(secret))
    }
    pub fn delete(&self, provider_id: &str) -> ObResult<()> { Self::entry(provider_id)?.delete_credential().map_err(|error| ObError::Internal(format!("Credential deletion failed: {error}"))) }
}
