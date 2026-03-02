#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};
use tiny_keccak::{Hasher, Keccak};
use zkapp::types::{ProofInput, ProofOutput};

pub fn main() {
    let proof_input: ProofInput = read_as();

    let mut raw_hash_value = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&proof_input.raw_image_bytes);
    hasher.finalize(&mut raw_hash_value);

    // TODO: replay edits and compare with new hash value

    let mut new_hash_value = [0; 32];
    let mut hasher = Keccak::v256();
    hasher.update(&proof_input.new_image_bytes);
    hasher.finalize(&mut new_hash_value);

    commit(&ProofOutput {
        raw_hash_value,
        new_hash_value,
    });
}
