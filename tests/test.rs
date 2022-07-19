// use prclient::prclient;
use pigrabbit::{types::Keys, *};
use serde_json;
use std::fs;
use types::*;

fn block_on<T: std::future::Future>(s: T) -> T::Output {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(s)
}
#[test]
fn retreive_ssl_certs() {
    let config_file = fs::read_to_string("config.json").expect("[pigrabbit] file does not exist!");
    let key_struct = serde_json::from_str(&config_file).unwrap();
    let mut prclient = pigrabbit::PRClient::new(key_struct);

    // let record = pigrabbit::types::record {
    //     domain = "arvinderd.com"
    // };
    // print!("{:?}", key_struct);
    block_on(async {
        let result = prclient
            .retreive_ssl_by_domain(
                // "a".to_owned(),
                "arvinderd.com",
                // "", // none,
            )
            .await
            .unwrap();
    });
    // println!("{:?}", result);
}

#[test]
fn retreive_by_domain_with_id() {
    let config_file = fs::read_to_string("config.json").expect("[pigrabbit] file does not exist!");
    let key_struct = serde_json::from_str(&config_file).unwrap();
    let mut prclient = pigrabbit::PRClient::new(key_struct);

    // let record = pigrabbit::types::record {
    //     domain = "arvinderd.com"
    // };
    // print!("{:?}", key_struct);
    block_on(async {
        let result = prclient
            .retreive_by_domain_with_id(
                // "a".to_owned(),
                "arvinderd.com",
                "", // "", // none,
            )
            .await
            .unwrap();
    });
    // println!("{:?}", result);
}
