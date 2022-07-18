#![doc = include_str!("../README.md")]
/// A PRClient stands for PigRabbitClient is
/// a web client with authentication keys that is required by PorkBun API.
///
/// Therefore, to construct the client, the apikey and secretapikey should be provided
/// to the PRClient in the Keys struct format before performing any interaction to the API.
///
/// An usage example of the PRClient:
/// ```
///
/// let keys_file = std::fs::read_to_string("keys.json").expect("File not found!");
/// let keys = serde_json::from_str(&keys_file).unwrap();
/// let mut client = pigrabbit::PRClient::new(keys);
///
/// client.retreive_by_domain_with_id("example.com", "1234567").await.unwrap();
/// client.del_by_id("example.com","1234567").await.unwrap();
///
/// ```
pub mod prclient;
pub use error::PigRabbitError;
pub use prclient::PRClient;
/// Possible errors for the library.
pub mod error;
/// All the simple types for the api.
pub mod types;
