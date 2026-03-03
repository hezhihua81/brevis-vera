#![no_main]
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};
use zkapp::types::{ProofInput, ProofOutput};

pico_sdk::entrypoint!(main);

pub fn main() {
    let proof_input: ProofInput = read_as();

    let public_key = VerifyingKey::from_bytes(&proof_input.public_key_bytes).unwrap();
    let signature = Signature::from_bytes(&proof_input.signature_bytes);
    public_key
        .verify(&proof_input.raw_image_bytes, &signature)
        .expect("C2PA Signature Verification Failed");

    let mut processed_img = proof_input.raw_image_bytes.clone();
    for op in proof_input.operations {
        match op {
            zkapp::types::EditOperation::Crop {
                x,
                y,
                width,
                height,
                source_width,
                source_height,
            } => processed_img = apply_crop(&processed_img, x, y, width, height, source_width, source_height),
            zkapp::types::EditOperation::AdjustBrightness { delta } => {
                processed_img = apply_brightness(&mut processed_img, delta);
            }
        }
    }

    assert_eq!(
        processed_img, proof_input.new_image_bytes,
        "Transformation mismatch!"
    );

    let old_hash = Sha256::digest(&proof_input.raw_image_bytes);
    let new_hash = Sha256::digest(&proof_input.new_image_bytes);

    commit(&ProofOutput {
        raw_hash_value: old_hash.into(),
        new_hash_value: new_hash.into(),
    });
}

fn apply_brightness(pixels: &mut [u8], delta: i16) -> Vec<u8> {
    for pixel in pixels.iter_mut() {
        let val = *pixel as i16 + delta;
        *pixel = val.clamp(0, 255) as u8;
    }
    pixels.to_vec()
}

fn apply_crop(data: &[u8], x: u32, y: u32, w: u32, h: u32, source_width: u32, _source_height: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 3) as usize);
    let src_width = source_width as usize;

    for row in y..(y + h) {
        for col in x..(x + w) {
            let src_idx = (row as usize * src_width + col as usize) * 3;
            if src_idx + 2 < data.len() {
                out.push(data[src_idx]);     // R
                out.push(data[src_idx + 1]); // G
                out.push(data[src_idx + 2]); // B
            }
        }
    }
    out
}
