use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::Bytes;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EditOperation {
    Crop {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    AdjustBrightness {
        delta: i16,
    },
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofInput {
    pub raw_image_bytes: Vec<u8>,
    #[serde_as(as = "Bytes")]
    pub signature_bytes: [u8; 64],
    #[serde_as(as = "Bytes")]
    pub public_key_bytes: [u8; 32],
    pub operations: Vec<EditOperation>,
    pub new_image_bytes: Vec<u8>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofOutput {
    #[serde_as(as = "Bytes")]
    pub raw_hash_value: [u8; 32],
    #[serde_as(as = "Bytes")]
    pub new_hash_value: [u8; 32],
}
