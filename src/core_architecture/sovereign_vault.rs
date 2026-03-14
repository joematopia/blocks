//! Bitopia: Sovereign Cold Storage (Digital Gold Vault)
//! Logic: Low-Volume, High-Value Data Custody in MEO

pub enum DataProfile {
    Commodity(u64),      // Standard files (Size in MB)
    HighValueAsset(u64), // Seed phrases, AI Weights, IP (Size in MB)
}

pub struct StorageRequest {
    pub client_zk_id: String,
    pub profile: DataProfile,
}

pub struct OrbitalVault {
    pub base_price_per_mb: f64, // $SOV
    pub vault_premium_multiplier: f64, 
}

impl OrbitalVault {
    /// Calculates the storage custody fee based on Data Profile, NOT just size.
    pub fn quote_storage(&self, request: StorageRequest) -> Result<f64, &'static str> {
        match request.profile {
            DataProfile::Commodity(size_mb) => {
                // Reject high-volume commodity storage to save physical launch mass
                if size_mb > 100 {
                    return Err("🚫 REJECTED: Orbital mass limits exceeded. Bitopia does not host commodity data.");
                }
                Ok(size_mb as f64 * self.base_price_per_mb)
            },
            DataProfile::HighValueAsset(size_mb) => {
                // Apply the "Swiss Bank" Premium for Sovereign Air-Gapping
                let fee = size_mb as f64 * self.base_price_per_mb * self.vault_premium_multiplier;
                // Note: In real production, this would use a logging framework
                Ok(fee)
            }
        }
    }
}
