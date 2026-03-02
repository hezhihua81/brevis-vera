use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofInput {
    pub raw_image_bytes: Vec<u8>,
    pub operations: Vec<EditOperation>,
    pub new_image_bytes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofOutput {
    pub raw_hash_value: [u8; 32],
    pub new_hash_value: [u8; 32],
}
