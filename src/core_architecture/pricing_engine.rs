//! Bitopia Layer 1: Thermodynamic Pricing & Burn Engine (v2.0)
//! Logic: N-Dimensional E-Watt pricing, Exponential Thermal Penalties, and Deflationary Burn

pub struct ComputeRequest {
    pub client_zk_id: String,
    pub base_compute_watts: f64, // W_c
    pub local_thermal_load: f64, // T(x,t) in percentage (0.0 to 1.0)
}

pub struct PaymentDistribution {
    pub total_sov_cost: f64,
    pub burned_sov: f64,
    pub hardware_fee_sov: f64,
}

pub struct PricingEngine {
    pub e_watt_usd_peg: f64,       // Base parameter: $0.05
    pub oracle_sov_usd_price: f64, // Phi(t): Current price of $SOV in USD (e.g., $1.25)
    
    // Tensor Constants
    pub alpha: f64, // Compute baseline weight
    pub beta: f64,  // Thermal penalty multiplier
    pub gamma: f64, // Exponential severity constant
}

impl PricingEngine {
    /// Calculates C_SOV(t) using the exponential thermodynamic manifold
    pub fn calculate_compute_cost(&self, req: &ComputeRequest) -> f64 {
        // Base physical cost of the compute
        let base_cost = self.alpha * req.base_compute_watts;
        
        // Exponential thermal penalty: e^(gamma * T(x,t))
        let thermal_exponent = self.gamma * req.local_thermal_load;
        let thermal_penalty = self.beta * thermal_exponent.exp();
        
        // Total cost in USD (E-Watts pegged)
        let total_usd_cost = (base_cost + thermal_penalty) * self.e_watt_usd_peg;
        
        // Convert to $SOV using the real-time Oracle price
        total_usd_cost / self.oracle_sov_usd_price
    }

    /// Executes the Deflationary Supply Calculus (80% Burn / 20% Hardware)
    pub fn process_payment(&self, req: ComputeRequest) -> PaymentDistribution {
        let total_sov = self.calculate_compute_cost(&req);
        
        // The Burn Equilibrium
        let burned = total_sov * 0.80; // Destroyed forever
        let hardware_fee = total_sov * 0.20; // Paid to Bitaris/Node Operator

        println!("⚡ E-WATT ALLOCATION: Processing {} base watts for {}.", req.base_compute_watts, req.client_zk_id);
        println!("🔥 DEFLATIONARY BURN: {:.2} $SOV permanently removed from circulation.", burned);
        println!("🛰️ HARDWARE YIELD: {:.2} $SOV routed to Bitaris.", hardware_fee);

        PaymentDistribution {
            total_sov_cost: total_sov,
            burned_sov: burned,
            hardware_fee_sov: hardware_fee,
        }
    }
}

fn main() {
    let engine = PricingEngine {
        e_watt_usd_peg: 0.05,
        oracle_sov_usd_price: 1.25, // Assuming $SOV is currently trading at $1.25
        alpha: 1.0,
        beta: 2.5,
        gamma: 4.0, 
    };

    // Scenario: AI company requesting 10,000 watts. The satellite is running hot (85% thermal load).
    let req = ComputeRequest {
        client_zk_id: String::from("0xAI_CORP_OMEGA"),
        base_compute_watts: 10000.0,
        local_thermal_load: 0.85, 
    };

    let result = engine.process_payment(req);
    println!("✅ TRANSACTION COMPLETE: Total Cost: {:.2} $SOV", result.total_sov_cost);
}
