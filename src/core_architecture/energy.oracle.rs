use std::collections::HashMap;

// 1. Physical Infrastructure Classifications
#[derive(Debug, Clone, PartialEq)]
pub enum DerType {
    OrbitalSolarArray,
    TerrestrialSolarFarm,
    GeothermalTap,
    KineticCapture,
}

// 2. The Axiom IoT Sensor Registry
#[derive(Debug, Clone)]
pub struct AxiomSensor {
    pub hardware_id: String,           // The physical chip serial number
    pub der_type: DerType,             // What kind of energy is it generating?
    pub owner_identity_hash: String,   // The Citizen who owns this physical farm
    pub max_capacity_kwh: f64,         // Hardware limit to prevent "infinite energy" spoofing
    pub total_minted_ewatts: f64,
}

// 3. The Telemetry Payload (Incoming from the physical solar farm)
#[derive(Debug, Clone)]
pub struct EnergyTelemetry {
    pub hardware_id: String,
    pub timestamp: u64,
    pub generated_kwh: f64,
    pub hardware_zk_proof: String, // The chip's unforgeable cryptographic seal
}

// 4. dasOS Energy Oracle Subsystem
pub struct DasEnergyOracle {
    pub registered_sensors: HashMap<String, AxiomSensor>,
    pub global_ewatt_supply: f64,
}

impl DasEnergyOracle {
    pub fn new() -> Self {
        DasEnergyOracle {
            registered_sensors: HashMap::new(),
            global_ewatt_supply: 0.0,
        }
    }

    // A Citizen registers a new physical solar farm to the network
    pub fn register_der_sensor(
        &mut self, 
        hardware_id: &str, 
        der_type: DerType, 
        owner_hash: &str, 
        capacity: f64
    ) {
        let new_sensor = AxiomSensor {
            hardware_id: hardware_id.to_string(),
            der_type,
            owner_identity_hash: owner_hash.to_string(),
            max_capacity_kwh: capacity,
            total_minted_ewatts: 0.0,
        };
        self.registered_sensors.insert(hardware_id.to_string(), new_sensor);
    }

    // ==========================================
    // ⚡ THE E-WATT MINTING ENGINE
    // ==========================================
    pub fn process_telemetry_and_mint(&mut self, payload: EnergyTelemetry) -> Result<String, String> {
        // 1. Identify the hardware
        let sensor = self.registered_sensors.get_mut(&payload.hardware_id)
            .ok_or("ORACLE REJECTED: Unregistered hardware ID.")?;

        // 2. Cryptographic Enclave Check
        if payload.hardware_zk_proof != "VALID_AXIOM_ENCLAVE_SIGNATURE" {
            return Err("ORACLE REJECTED: Hardware signature invalid. Possible spoofing detected.".to_string());
        }

        // 3. Physics Check: Did they claim to produce more energy than the hardware is capable of?
        if payload.generated_kwh > sensor.max_capacity_kwh {
            return Err("ORACLE REJECTED: Generation exceeds physical hardware constraints.".to_string());
        }

        // 4. Tokenization (1 kWh = 1 E-Watt)
        let newly_minted_ewatts = payload.generated_kwh;
        
        sensor.total_minted_ewatts += newly_minted_ewatts;
        self.global_ewatt_supply += newly_minted_ewatts;

        // In a full implementation, this would trigger a transaction to send `newly_minted_ewatts` 
        // to `sensor.owner_identity_hash` on the main blockchain ledger.

        Ok(format!(
            "SUCCESS: Verified {} kWh from {}. Minted {} $E-WATT to Citizen {}.",
            payload.generated_kwh, 
            payload.hardware_id, 
            newly_minted_ewatts, 
            sensor.owner_identity_hash
        ))
    }
}
