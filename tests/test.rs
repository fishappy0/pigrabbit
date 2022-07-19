static config: &'static str = include_str!("../config.json");
// use prclient::prclient;
use pigrabbit::{types::Keys, *};
use serde_json;
use std::fs;
use types::*;

fn init_prclient() -> PRClient {
    let key_struct = serde_json::from_str(&config).unwrap();
    let mut prclient = pigrabbit::PRClient::new(key_struct);
    prclient
}

fn block_on<T: std::future::Future>(s: T) -> T::Output {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(s)
}
#[test]
fn retreive_ssl_certs() {
    let mut prclient = init_prclient();
    block_on(async {
        prclient
            .retreive_ssl_by_domain("arvinderd.com")
            .await
            .unwrap();
    });
}

#[test]
fn retreive_by_domain_with_id() {
    let mut prclient = init_prclient();
    block_on(async {
        prclient
            .retreive_by_domain_with_id(
                "arvinderd.com",
                "", // "", // none,
            )
            .await
            .unwrap();
    });
}

#[test]
fn add_record() {
    let mut prclient = init_prclient();
    block_on(async {
        prclient
            .retreive_by_domain_with_id(
                "arvinderd.com",
                "", // "", // none,
            )
            .await
            .unwrap();
    });
}
