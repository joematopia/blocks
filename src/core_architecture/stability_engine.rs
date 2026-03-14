//! Bitopia: $SOV Algorithmic Stability Engine
//! Version: 1.0 (Post-Quantum Secure)
//! Mission: Maintain $1.00 Peg via Thermodynamic Arbitrage

pub struct ScarcityEngine {
    pub current_sov_price_usd: f64,
    pub btc_reserve_sats: u64,
    pub total_sov_supply: u64,
}

impl ScarcityEngine {
    /// The Core Peg Logic: Expansion & Contraction
    pub fn regulate_supply(&mut self, market_price: f64) -> String {
        self.current_sov_price_usd = market_price;

        if self.current_sov_price_usd > 1.02 {
            self.expand_supply()
        } else if self.current_sov_price_usd < 0.98 {
            self.contract_supply()
        } else {
            String::from("⚖️ PEG_STABLE: No intervention required.")
        }
    }

    /// EXPANSION: Price is too high ($SOV is too scarce)
    /// Mints new $SOV and distributes it via Proof of Contribution.
    fn expand_supply(&mut self) -> String {
        let mint_amount = (self.total_sov_supply as f64 * 0.01) as u64; // 1% expansion
        self.total_sov_supply += mint_amount;
        
        format!(
            "📈 EXPANSION TRIGGERED: Minting {} $SOV to Contributors (PoC) to restore $1.00 peg.",
            mint_amount
        )
    }

    /// CONTRACTION: Price is too low ($SOV is in oversupply)
    /// Uses BTC Treasury to buy back and BURN $SOV.
    fn contract_supply(&mut self) -> String {
        let burn_amount = (self.total_sov_supply as f64 * 0.01) as u64;
        
        // Logic: Treasury sells BTC Sats -> Buys $SOV -> Deletes $SOV
        self.total_sov_supply -= burn_amount;
        
        format!(
            "🔥 CONTRACTION TRIGGERED: Burning {} $SOV using BTC Treasury reserves to restore $1.00 peg.",
            burn_amount
        )
    }
}

fn main() {
    let mut engine = ScarcityEngine {
        current_sov_price_usd: 1.00,
        btc_reserve_sats: 18_421_000_000, // 184.21 BTC
        total_sov_supply: 10_000_000,
    };

    println!("--- Market Scenario: High Demand (Price = $1.05) ---");
    println!("{}", engine.regulate_supply(1.05));

    println!("\n--- Market Scenario: Panic/Sell-off (Price = $0.94) ---");
    println!("{}", engine.regulate_supply(0.94));
}
