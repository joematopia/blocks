/*
 * REPOSITORY: bitaris-hardware-core
 * MODULE: axiom-c-firmware (Commercial / DER)
 * DESCRIPTION: Secure energy metrology, hardware attestation, and LTE-M broadcast.
 */

#include <zephyr/kernel.h>
#include <zephyr/net/socket.h>
#include <modem/lte_lc.h>
#include <cryptoauthlib.h> // Microchip ATECC608B Library
#include <cJSON.h>         // Payload formatting

// --- CONFIGURATION ---
#define BLOCKCHAIN_RPC_URL "http://node-01.blocks-network.bitaris.io/submitEnergyProof"
#define AXIOM_CHIP_ID "0x7F8C...A1B2" // Hardcoded public address of this specific chip
#define PRIVATE_KEY_SLOT 0            // Secure Enclave slot holding the immutable private key
#define BROADCAST_INTERVAL_MS 600000  // 10 minutes (Enforces the Time-Lock / TTL)

// --- PROTOTYPE FUNCTIONS ---
extern double read_metrology_ic_spi(); // Interfaces with Analog Devices ADE9153A
extern uint32_t get_network_time();    // Fetches verified Unix timestamp via cellular NTP

void main(void) {
    printk("Bitaris Axiom-C Boot Sequence Initiated...\n");

    // 1. Initialize Cellular LTE-M Modem
    if (lte_lc_init_and_connect() != 0) {
        printk("FATAL: Failed to connect to LTE-M network.\n");
        return;
    }
    printk("Cellular Network: CONNECTED.\n");

    // 2. Initialize Microchip ATECC608B Secure Enclave
    ATCAIfaceCfg cfg = cfg_ateccx08a_i2c_default;
    if (atcab_init(&cfg) != ATCA_SUCCESS) {
        printk("FATAL: Secure Enclave Tamper Detected or Unreachable.\n");
        return;
    }
    printk("Secure Enclave: ONLINE & LOCKED.\n");

    // --- MAIN ATTESTATION LOOP ---
    while (1) {
        // A. Read pure physics (Energy generated in MWh)
        double energy_mwh = read_metrology_ic_spi();
        uint32_t current_timestamp = get_network_time();

        // B. Construct the raw payload to match the Solidity Smart Contract
        char payload_string[128];
        snprintf(payload_string, sizeof(payload_string), "%s|%f|%u", 
                 AXIOM_CHIP_ID, energy_mwh, current_timestamp);

        // C. Hash the payload (Keccak256 equivalent preparation)
        uint8_t payload_hash[32];
        atcab_sha(strlen(payload_string), (uint8_t*)payload_string, payload_hash);

        // D. ZERO-TRUST SIGNING: The Secure Enclave signs the hash internally.
        // The private key NEVER leaves the ATECC608B chip.
        uint8_t signature[64];
        if (atcab_sign(PRIVATE_KEY_SLOT, payload_hash, signature) != ATCA_SUCCESS) {
            printk("ERROR: Cryptographic Signing Failed.\n");
            continue;
        }

        // E. Package the final JSON RPC for the blocks network
        cJSON *rpc_post = cJSON_CreateObject();
        cJSON_AddStringToObject(rpc_post, "chipId", AXIOM_CHIP_ID);
        cJSON_AddNumberToObject(rpc_post, "energyAmount", energy_mwh);
        cJSON_AddNumberToObject(rpc_post, "timestamp", current_timestamp);
        
        // Convert signature array to hex string (pseudo-function for brevity)
        char hex_sig[129];
        bytes_to_hex_string(signature, 64, hex_sig); 
        cJSON_AddStringToObject(rpc_post, "signature", hex_sig);

        char *json_string = cJSON_PrintUnformatted(rpc_post);

        // F. Broadcast directly to decentralized nodes
        printk("Broadcasting Attestation Payload: %s\n", json_string);
        send_http_post(BLOCKCHAIN_RPC_URL, json_string);

        // Clean up memory
        cJSON_Delete(rpc_post);
        free(json_string);

        // G. Sleep until the next TTL cycle
        k_msleep(BROADCAST_INTERVAL_MS);
    }
}
