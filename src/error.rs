use thiserror::Error;

#[derive(Error, Debug)]
pub enum PigRabbitError {
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Porkbun Error!\n{0}")]
    ResponseError(serde_json::Value),
}
