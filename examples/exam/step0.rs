extern crate serde_json;

use std::collections::HashMap;
use serde_json::{json, Value};

fn main() {
    // HashMap creation for key-value pairs storage
    let mut data_store: HashMap<String, Value> = HashMap::new();

    // key-value pair insertion
    let key = "user1".to_string();
    let value = json!({
        "name": "Chris",
        "age": 30,
        "email": "chris@example.com"
    });
    data_store.insert(key.clone(), value);

    // key-value pair retrieval
    if let Some(retrieved_value) = data_store.get(&key) {
        println!("The {} key value: {}", key, retrieved_value);
    } else {
        println!("The {} key not found!", key);
    }
}
