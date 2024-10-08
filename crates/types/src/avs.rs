use alloy_primitives::FixedBytes;
use eigen_crypto_bls::Signature;
use thiserror::Error;
use tokio::sync::mpsc::Sender;
pub type TaskIndex = u32;

pub type TaskResponseDigest = FixedBytes<32>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SignatureVerificationError {
    #[error("incorrect signature error")]
    IncorrectSignature,
    #[error("operator public key not found")]
    OperatorPublicKeyNotFound,
    #[error("operator not found")]
    OperatorNotFound,
}

#[derive(Debug, Clone)]
pub struct SignedTaskResponseDigest {
    pub task_response_digest: TaskResponseDigest,

    pub bls_signature: Signature,

    pub operator_id: FixedBytes<32>,

    pub signature_verification_channel: Sender<Result<(), SignatureVerificationError>>,
}
