use serde_json::{json, Value};

fn main() {
    // JSON object creation
    let data: Value = json!({
        "name": "Chris",
        "age": 30,
        "city": "Budapest"
    });

    // Print Json data
    println!("JSON data: {}", data);

    // Acces Json data
    println!("Name: {}", data["name"]);
    println!("Age: {}", data["age"]);

    // Check if key exists
    if let Some(city) = data.get("city") {
        println!("City: {}", city);
    } else {
        println!("The 'city' not found!");
    }
}