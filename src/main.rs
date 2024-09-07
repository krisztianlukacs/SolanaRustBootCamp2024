use std::collections::HashMap;
use serde_json::{json, Value};

fn main() {

    // HashMap creation for key-value pairs storage
    let mut data_store: HashMap<String, Value> = HashMap::new();

    // key-value pair insertion
    let key = "user1".to_string();
    
    // JSON object creation
    let value: Value = json!({
        "name": "Chris",
        "age": 30,
        "city": "Budapest"
    });

    data_store.insert(key.clone(), value.clone());

    // Print Json data
    println!("JSON data: {}", value);

    // Acces Json data
    println!("Name: {}", value["name"]);
    println!("Age: {}", value["age"]);

    // Check if key exists
    if let Some(city) = value.get("city") {
        println!("City: {}", city);
    } else {
        println!("The 'city' not found!");
    }

     // key-value pair retrieval
     if let Some(retrieved_value) = data_store.get(&key) {
        println!("The {} key value: {}", key, retrieved_value);
    } else {
        println!("The {} key not found!", key);
    }
}