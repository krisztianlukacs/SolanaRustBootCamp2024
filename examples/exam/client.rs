use serde_json::json;
use solana_sdk::signature::{Keypair, Signer};

fn set_value(key: &str, value: &serde_json::Value, program_id: &Pubkey, payer: &Keypair) {
    let transaction = json!({
        "key": key,
        "value": value
    });

    // Create and send Solana transaction
}
