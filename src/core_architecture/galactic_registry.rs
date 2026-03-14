//! Bitopia: Galactic Registry & Taxation Engine
//! Logic: Handles DAO Formation, IP, and RWA Tokenization (Celestial Assets)

pub enum EntityType {
    SovereignDAO,
    IntellectualProperty,
    CelestialClaim,           // Staking the physical claim
    RWA_Tokenization(u64),    // Fractionalizing the claim (Number of tokens minted)
}

pub struct RegistrationRequest {
    pub client_zk_id: String,
    pub entity_type: EntityType,
    pub years_prepaid: u32,
}

pub struct RegistryTaxEngine {
    pub base_dao_fee: f64,    // 500 $SOV
    pub base_ip_fee: f64,     // 50 $SOV
    pub base_claim_fee: f64,  // 1000 $SOV
    pub annual_tax_rate: f64, // 10 $SOV per year
}

impl RegistryTaxEngine {
    pub fn process_registration(&self, req: RegistrationRequest) -> String {
        let mut total_fee = 0.0;

        match req.entity_type {
            EntityType::SovereignDAO => {
                total_fee += self.base_dao_fee;
                println!("🏛️ DAO FORMATION: Wrapper generated for {}.", req.client_zk_id);
            },
            EntityType::IntellectualProperty => {
                total_fee += self.base_ip_fee;
                println!("💡 IP HASHED: Immutable rights secured.");
            },
            EntityType::CelestialClaim => {
                total_fee += self.base_claim_fee;
                println!("🪐 CELESTIAL CLAIM: Orbital/Lunar coordinates locked.");
            },
            EntityType::RWA_Tokenization(supply) => {
                // Charge a micro-fee for every fractional token generated
                let minting_tax = supply as f64 * 0.01; 
                total_fee += minting_tax;
                println!("⛏️ RWA TOKENIZATION: Minting {} fractional shares of space asset.", supply);
            }
        }

        // Add Recurring Anti-Bloat Tax
        let tax_burden = self.annual_tax_rate * req.years_prepaid as f64;
        total_fee += tax_burden;

        format!("✅ SUCCESS: Total Treasury Fee: {} $SOV", total_fee)
    }
}

fn main() {
    let bitopia_registry = RegistryTaxEngine {
        base_dao_fee: 500.0, base_ip_fee: 50.0, base_claim_fee: 1000.0, annual_tax_rate: 10.0,
    };

    // Scenario: A Mining DAO registers a Lunar Claim, prepays 10 years, and mints 100k tokens
    let claim_req = RegistrationRequest {
        client_zk_id: String::from("0xLUNAR_MINERS"),
        entity_type: EntityType::CelestialClaim,
        years_prepaid: 10,
    };

    let token_req = RegistrationRequest {
        client_zk_id: String::from("0xLUNAR_MINERS"),
        entity_type: EntityType::RWA_Tokenization(100_000), // Minting 100k shares
        years_prepaid: 0, // One-time minting event
    };

    println!("--- Step 1: Staking the Physical Claim ---");
    println!("{}\n", bitopia_registry.process_registration(claim_req));

    println!("--- Step 2: RWA Fractionalization ---");
    println!("{}", bitopia_registry.process_registration(token_req));
}
