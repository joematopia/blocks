//! Bitopia: Orbital Compute Marketplace
//! Purpose: Bridges Terrestrial AI demand with Orbital E-Watt supply.

pub struct ComputeJob {
    pub job_id: String,
    pub client_id: String,
    pub complexity_index: u64, // Floating point operations required
    pub bounty_sov: f64,       // Amount of $SOV offered
}

pub struct Marketplace {
    pub min_e_watt_price_sov: f64, // Dynamic floor price
}

impl Marketplace {
    /// Matches an AI company's request with a satellite's available energy
    pub fn match_job(&self, job: ComputeJob, available_e_watts: f64) -> Result<String, &'static str> {
        // Calculate cost based on orbital physics and energy scarcity
        let required_sov = job.complexity_index as f64 * self.min_e_watt_price_sov;

        if job.bounty_sov < required_sov {
            return Err("🚫 BOUNTY TOO LOW: Orbital compute cost exceeds offer.");
        }

        if available_e_watts < (job.complexity_index as f64 * 0.1) {
            return Err("🚫 INSUFFICIENT ENERGY: Satellite in eclipse/low battery.");
        }

        Ok(format!("✅ JOB_ACCEPTED: {} allocated to Node.", job.job_id))
    }
}
