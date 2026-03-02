# ZK Proof Generation Specification

## Overview
This specification describes the ZK proof generation system using Pico ZKVM.
Reference: https://github.com/benhuang2025/vera-vibe/tree/main/brevis-vera-zk

## Architecture

### Components
1. **ZKVM Circuit** (`brevis-vera-zk/app`): RISC-V program that verifies image edits
2. **Prover** (`brevis-vera-zk/prover`): Uses Pico ZKVM to generate proofs
3. **Host Library**: Provides data structures and host-side proof generation

### Data Flow
```
Original Image + Edit Manifest → ZKVM Circuit → Public Values → Proof Package
```

## ADDED Requirements

### Requirement: Pico ZKVM Circuit
The system SHALL use a ZKVM circuit that verifies image edit integrity.

#### Scenario: Circuit verifies ECDSA signature
- **WHEN** the ZKVM circuit receives a SignedPhoto
- **THEN** it verifies the ECDSA P-256 signature over the metadata

#### Scenario: Circuit replays edit operations
- **WHEN** the ZKVM circuit receives an EditManifest
- **THEN** it replays each edit operation on the image pixels

#### Scenario: Circuit commits public values
- **WHEN** the ZKVM circuit completes execution
- **THEN** it commits: pub_key_hash, edit_types, output_image_hash

### Requirement: Proof Input Format
The system SHALL prepare proof inputs in bincode format for ZKVM.

#### Scenario: SignedPhoto structure
- **GIVEN** original image with C2PA metadata
- **WHEN** creating SignedPhoto
- **THEN** the structure contains: image_bytes, metadata (device_id, timestamp, width, height, image_hash), signature (r, s, public_key)

#### Scenario: EditManifest structure
- **GIVEN** list of edit operations
- **WHEN** creating EditManifest
- **THEN** the structure contains: operations (EditOperation enum)

### Requirement: Proof Generation
The system SHALL generate proofs using Pico ZKVM prover.

#### Scenario: Prover loads ELF binary
- **WHEN** generating a proof
- **THEN** the prover loads the RISC-V ELF from `app/elf/riscv32im-pico-zkvm-elf`

#### Scenario: Prover uses prove_fast
- **WHEN** generating a RISC-V proof
- **THEN** the prover uses `prove_fast` method from DefaultProverClient

#### Scenario: Placeholder proof
- **WHEN** Pico ZKVM toolchain is not available
- **THEN** generate placeholder proof with zero-filled proof bytes

### Requirement: Proof Output Format
The system SHALL output ProofPackage in JSON format.

#### Scenario: Valid proof output
- **WHEN** proof is generated
- **THEN** output JSON contains:
  - `edited_image`: bytes of the edited image
  - `proof`: proof bytes (placeholder or actual)
  - `public_values.pub_key_hash`: hash of signing key
  - `public_values.edit_types`: array of edit operation names
  - `public_values.output_image_hash`: SHA256 of edited image

### Requirement: Error handling
The system SHALL handle errors gracefully.

#### Scenario: Missing original file
- **WHEN** original file does not exist
- **THEN** return error "Original file not found: <path>"

#### Scenario: Missing edited file
- **WHEN** edited file does not exist
- **THEN** return error "Edited file not found: <path>"

#### Scenario: Pico ZKVM not available
- **WHEN** `cargo pico --version` fails
- **THEN** generate placeholder proof with note in error field
