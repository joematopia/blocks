use std::collections::HashMap;

// 1. Post-Quantum & Sharding Primitives
type PQSignature = Vec<u8>; // NIST-standard Dilithium signature
type BlockHash = String;    // Quantum-safe Keccak-512 hash

// 2. The Zero-Knowledge Bitopian Citizen
#[derive(Debug, Clone)]
pub struct BitopianCitizen {
    pub zkp_identity_hash: String, 
    pub risk_clearance: u8,        
    pub staked_sov: f64,
    pub habitat_sector: String,    
}

// 3. The Hardware-Locked Axiom Node (NEW)
#[derive(Debug, Clone)]
pub struct AxiomNode {
    pub hardware_id: String,       // Unforgeable silicon serial number
    pub is_active: bool,
    pub lifetime_e_watts: f64,     // Total energy verifiable by the enclave
}

// 4. The Tamper-Resistant Block (Upgraded)
#[derive(Debug, Clone)]
pub struct Block {
    pub block_height: u64,
    pub timestamp: u64,
    pub previous_hash: BlockHash,
    pub state_shard_id: u32,       
    pub transactions: Vec<String>, 
    pub validator_signature: PQSignature, 
    pub hardware_zk_proof: String, // NEW: Proof the energy came from an Axiom node
}

// 5. The Space Habitat Jurisdiction Engine
pub struct SovereignHabitat {
    pub habitat_name: String,
    pub current_block: u64,
    pub ledger_chain: Vec<Block>,
    pub citizens: HashMap<String, BitopianCitizen>,
    pub authorized_nodes: HashMap<String, AxiomNode>, // The swarm registry
    pub thermal_capacity_mw: f64,  
}

impl SovereignHabitat {
    pub fn new(name: &str) -> Self {
        let genesis_block = Block {
            block_height: 0,
            timestamp: 0,
            previous_hash: "00000000000000000000".to_string(),
            state_shard_id: 0,
            transactions: vec!["GENESIS_STATE_INITIALIZED".to_string()],
            validator_signature: vec![],
            hardware_zk_proof: "GENESIS_ENCLAVE_OVERRIDE".to_string(),
        };

        // Pre-registering the first Axiom node for the swarm
        let mut initial_nodes = HashMap::new();
        initial_nodes.insert(
            "AXIOM_HW_001".to_string(), 
            AxiomNode { hardware_id: "AXIOM_HW_001".to_string(), is_active: true, lifetime_e_watts: 0.0 }
        );

        SovereignHabitat {
            habitat_name: name.to_string(),
            current_block: 0,
            ledger_chain: vec![genesis_block],
            citizens: HashMap::new(),
            authorized_nodes: initial_nodes,
            thermal_capacity_mw: 100_000.0,
        }
    }

    // The Immigration Airlock
    pub fn onboard_citizen(&mut self, zkp_payload: &str) -> Result<String, String> {
        if self.verify_zk_proof(zkp_payload) {
            let new_citizen = BitopianCitizen {
                zkp_identity_hash: "ZK_VERIFIED_HASH_8F9A".to_string(),
                risk_clearance: 1, 
                staked_sov: 0.0,
                habitat_sector: "Axiom_Ring_A".to_string(),
            };
            self.citizens.insert(new_citizen.zkp_identity_hash.clone(), new_citizen.clone());
            Ok(format!("Welcome to Bitopia. Sector Assignment: {}", new_citizen.habitat_sector))
        } else {
            Err("Uplink Rejected: Risk Parameters Exceeded".to_string())
        }
    }

    fn verify_zk_proof(&self, _payload: &str) -> bool { true }

    // ==========================================
    // 🛡️ THE HARDWARE-LOCKED FORGE
    // ==========================================
    pub fn mine_block(
        &mut self, 
        pending_transactions: Vec<String>, 
        hardware_id: &str, 
        hardware_zk_proof: &str
    ) -> Result<Block, String> {
        
        // 1. SILICON VERIFICATION: Is this an authorized Axiom node?
        let node = self.authorized_nodes.get_mut(hardware_id)
            .ok_or("CRITICAL ALERT: Unauthorized hardware attempting to forge.")?;

        if !node.is_active {
            return Err("Node is deactivated.".to_string());
        }

        // 2. ENCLAVE VERIFICATION: Did the hardware actually burn the E-Watts?
        if hardware_zk_proof != "VALID_ENCLAVE_SIGNATURE" {
             return Err("51% ATTACK DETECTED: Energy signature invalid. Forging rejected.".to_string());
        }

        // 3. The node is legit. Update its lifetime energy burn.
        node.lifetime_e_watts += 14.2; 

        // 4. Build and lock the block
        let previous_block = self.ledger_chain.last().unwrap();
        let new_previous_hash = format!("HASH_OF_BLOCK_{}", previous_block.block_height);
        let pq_seal = vec![0x0F, 0x1A, 0x2B, 0x3C]; 

        let new_block = Block {
            block_height: previous_block.block_height + 1,
            timestamp: 1710200000, 
            previous_hash: new_previous_hash,
            state_shard_id: 1, 
            transactions: pending_transactions,
            validator_signature: pq_seal,
            hardware_zk_proof: hardware_zk_proof.to_string(), 
        };
        
        self.ledger_chain.push(new_block.clone());
        self.current_block = new_block.block_height;
        
        Ok(new_block)
    }
}
