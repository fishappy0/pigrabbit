# PigRabbit

[![github](https://shields.io/badge/github-green?logo=github&color=informational&style=for-the-badge)](https://github.com/fishappy0/pigrabbit)
[![docs](https://shields.io/badge/Docs.rs-green?logo=rust&color=green&style=for-the-badge)](https://docs.rs/pigrabbit/latest/pigrabbit/)
[![cratesio](https://shields.io/badge/Crates.io-green?logo=rust&color=orange&style=for-the-badge)](https://crates.io/crates/pigrabbit)
</br><p style="font-size:23px"> A rust wrapper for porkbun's api<p>

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
    ttl: "300".to_owned()
};

client.add_record("Example.com",&record).await.unwrap();
//Wait for the remote to catch up
tokio::time::sleep(std::time::Duration::from_secs(1)).await;
let records = client.retreive_by_domain_with_id("Example.com", "").await.unwrap();
println!("{:?}", records);

```
