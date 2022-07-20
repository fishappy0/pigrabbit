static CONFIG: &'static str = include_str!("../config.json");
// use prclient::prclient;
use pigrabbit::{types::Record, PRClient};
use serde_json;

fn init_prclient() -> PRClient {
    let key_struct = serde_json::from_str(&CONFIG).unwrap();
    let prclient = pigrabbit::PRClient::new(key_struct);
    prclient
}

fn block_on<T: std::future::Future>(s: T) -> T::Output {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(s)
}
#[test]
fn ping_test() {
    let mut prclient = init_prclient();
    block_on(async {
        prclient.ping_test().await.unwrap();
    });
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
fn add_and_remove_record() {
    let mut prclient = init_prclient();
    let record = Record {
        name: "test_record".to_owned(),
        dtype: "TXT".to_owned(),
        content: "test!".to_owned(),
        ttl: "300".to_owned(),
    };
    block_on(async {
        let id = prclient
            .add_record(
                "arvinderd.com",
                &record, // "", // none,
            )
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        prclient
            .del_by_id("arvinderd.com", &id.to_string())
            .await
            .unwrap();
    });
}

#[test]
fn add_edit_and_remove_record() {
    let mut prclient = init_prclient();
    let record = Record {
        name: "test_record_for_edit".to_owned(),
        dtype: "TXT".to_owned(),
        content: "test!".to_owned(),
        ttl: "300".to_owned(),
    };
    let edited_record = Record {
        name: "edited_test_record".to_owned(),
        dtype: "TXT".to_owned(),
        content: "edited test!".to_owned(),
        ttl: "420".to_owned(),
    };
    block_on(async {
        let id = prclient
            .add_record(
                "arvinderd.com",
                &record, // "", // none,
            )
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        prclient
            .edit_by_domain_and_id("arvinderd.com", &id.to_string(), &edited_record)
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        prclient
            .del_by_id("arvinderd.com", &id.to_string())
            .await
            .unwrap();
    });
}
