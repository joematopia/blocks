pub struct Treasury {
    pub fiat_reserve: f64,    // Layer 1: Shock Absorber
    pub e_watt_minted: f64,   // Layer 2: Productive Truth
    pub btc_reserve: f64,     // Layer 3: Apex Collateral
    pub sov_circulating: f64, // The Liquid Currency
}

impl Treasury {
    pub fn calculate_collateral_ratio(&self, btc_price: f64, e_watt_price: f64) -> f64 {
        let total_value = self.fiat_reserve 
                        + (self.btc_reserve * btc_price) 
                        + (self.e_watt_minted * e_watt_price);
        
        if self.sov_circulating == 0.0 {
            return 1.0; // Fully backed if no $SOV exists yet
        }
        
        total_value / self.sov_circulating
    }

    pub fn mint_sov(&mut self, amount: f64, btc_price: f64, e_watt_price: f64) -> Result<(), &'static str> {
        let projected_sov = self.sov_circulating + amount;
        let total_value = self.fiat_reserve + (self.btc_reserve * btc_price) + (self.e_watt_minted * e_watt_price);
        
        if (total_value / projected_sov) >= 1.0 {
            self.sov_circulating += amount;
            Ok(())
        } else {
            Err("CRITICAL HALT: Minting failed. Collateral ratio would drop below 100%.")
        }
    }
}

fn main() {
    println!(">>> INITIALIZING BITOPIA TRI-ASSET TREASURY...");
    
    // Simulating a Treasury with $100k Fiat, 50k E-Watts, and 2 BTC
    let mut bitopia_treasury = Treasury {
        fiat_reserve: 100_000.0,  
        e_watt_minted: 50_000.0,  
        btc_reserve: 2.0,         
        sov_circulating: 0.0,
    };

    let current_btc_price = 65_000.0;
    let current_ewatt_price = 1.0;

    println!(">>> ATTEMPTING TO MINT 200,000 $SOV...");
    
    match bitopia_treasury.mint_sov(200_000.0, current_btc_price, current_ewatt_price) {
        Ok(_) => println!("SUCCESS: 200,000 $SOV minted. Peg is secure."),
        Err(e) => println!("ERROR: {}", e),
    }
}
