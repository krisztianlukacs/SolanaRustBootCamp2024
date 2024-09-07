let state = State {
    key: "example_key".to_string(),
    value: json!({"example_field": "example_value"}),
};

let serialized = serde_json::to_string(&state)?;
println!("Serialized: {}", serialized);
