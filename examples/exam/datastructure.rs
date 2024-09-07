use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub key: String,
    pub value: serde_json::Value, // Stores the current value in JSON format
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    pub key: String,
    pub previous_values: Vec<(u64, serde_json::Value)>, // Timestamp and value
}
