# PigRabbit

A rust wrapper for porkbun's api

# Example

Adding a record to the domain and view all the records:

```rust
let keys_file = std::fs::read_to_string("keys.json").expect("File not found!");
let keys = serde_json::from_str(&keys_file).unwrap();
let mut client = pigrabbit::PRClient::new(keys);

let record = pigrabbit::types::Record{
    name: "internal".to_owned(),
    dtype: "A".to_owned(),
    content: "1.1.1.1".to_owned(),
    ttl: 300
};

client.add_record("Example.com",&record).await.unwrap();
let records = client.retreive_by_domain_with_id("Example.com", "").await.unwrap();
println!("{:?}", records);

```
