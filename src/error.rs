use crate::types::JsonRpcError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    /// SerdeJson
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    /// ErrorResponse
    #[error("{0:?}")]
    ErrorResponse(JsonRpcError),
}
