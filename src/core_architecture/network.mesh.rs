use std::collections::HashMap;

// 1. Post-Quantum Cryptographic Primitives
type PQHash = String;       // Keccak-512 / SHA-3 Quantum-resistant hash
type PQSignature = Vec<u8>; // Dilithium/Falcon lattice signature

// 2. The Nerve Impulse (A Single Transaction)
#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: PQHash,
    pub sender_hash: String,
    pub receiver_hash: String,
    pub ewatt_amount: f64,
    pub timestamp: u64,
    pub signature: PQSignature, // Proves the sender actually authorized this
}

// 3. Proof of Space-Time + Energy (PoST + PoE)
#[derive(Debug, Clone)]
pub struct AxiomTelemetry {
    pub hardware_id: String,
    pub spatial_coordinates: (f64, f64, f64), // X, Y, Z Orbital position
    pub timestamp: u64,                      // The "Time" in Space-Time
    pub energy_burned_ewatts: f64,           // The "Energy" in Proof of Energy
    pub hardware_zk_seal: String,            // Unforgeable silicon signature
}

// 4. The Nervous System (The Mesh Network State)
pub struct SwarmMesh {
    pub mempool: Vec<Transaction>, // Pending transactions waiting to be forged into a block
    pub active_telemetry: HashMap<String, AxiomTelemetry>, // Live feed of all satellites
}

impl SwarmMesh {
    pub fn new() -> Self {
        SwarmMesh {
            mempool: Vec::new(),
            active_telemetry: HashMap::new(),
        }
    }

    // ---------------------------------------------------------
    // 📡 SYNAPSE 1: Receiving a Transaction from a Citizen
    // ---------------------------------------------------------
    pub fn broadcast_transaction(&mut self, tx: Transaction) -> Result<String, String> {
        // In a real system, we would mathematically verify tx.signature against tx.sender_hash here.
        let is_valid_quantum_sig = true; 

        if is_valid_quantum_sig {
            self.mempool.push(tx.clone());
            Ok(format!("TX {} accepted into swarm mempool.", tx.tx_id))
        } else {
            Err("SYNAPSE REJECTED: Invalid Quantum Signature.".to_string())
        }
    }

    // ---------------------------------------------------------
    // 📡 SYNAPSE 2: Verifying Space-Time Physics
    // ---------------------------------------------------------
    pub fn ping_node_telemetry(&mut self, telemetry: AxiomTelemetry) -> Result<(), String> {
        // Validate that the hardware signature matches the physical telemetry
        if telemetry.hardware_zk_seal != "VALID_AXIOM_ENCLAVE_SIGNATURE" {
            return Err("MESH ALERT: Hardware spoofing detected. Node quarantined.".to_string());
        }

        // Update the nervous system with the node's current physical state
        self.active_telemetry.insert(telemetry.hardware_id.clone(), telemetry);
        Ok(())
    }

    // ---------------------------------------------------------
    // 🧠 THE CORTEX: Compressing Data via Merkle Root
    // ---------------------------------------------------------
    pub fn calculate_merkle_root(&self) -> PQHash {
        if self.mempool.is_empty() {
            return "EMPTY_MERKLE_ROOT_000000".to_string();
        }

        // In a live environment, this recursively hashes transactions pairwise.
        // For the architecture blueprint, we simulate the root generation:
        let combined_data: String = self.mempool.iter()
            .map(|tx| tx.tx_id.clone())
            .collect();
        
        format!("MERKLE_ROOT_OF_{}_TXS", self.mempool.len())
    }

    // This function clears the mempool after the governance_engine successfully mines the block
    pub fn clear_forged_transactions(&mut self) {
        self.mempool.clear();
    }
}
