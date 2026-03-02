# ZK Proof Verification Specification

## ADDED Requirements

### Requirement: Independent ZK proof verification
The system SHALL verify ZK proofs without requiring access to the original image, using Pico ZKVM verification.

#### Scenario: Valid proof verification
- **WHEN** user provides a valid ZK proof and edited image
- **THEN** system computes SHA256 of the edited image and compares with public commitment
- **AND** returns verification result with is_valid = true and lists attested transformations

#### Scenario: Invalid proof verification
- **WHEN** user provides an invalid or tampered ZK proof
- **THEN** system returns verification result with is_valid = false and error message

#### Scenario: Proof for different image
- **WHEN** ZK proof is verified against a different image than the one used to generate it
- **THEN** system returns verification result with is_valid = false due to commitment mismatch

### Requirement: Verification output format
The system SHALL output verification results in a standardized format.

#### Scenario: Successful verification output
- **WHEN** proof is successfully verified
- **THEN** system outputs JSON containing:
  - `is_valid`: true
  - `verified_transformations`: Array of transformation names
  - `error`: null

#### Scenario: Failed verification output
- **WHEN** proof verification fails
- **THEN** system outputs JSON containing:
  - `is_valid`: false
  - `verified_transformations`: empty array
  - `error`: description of the failure

#### Scenario: Placeholder proof detection
- **WHEN** the proof file contains "Pico ZKVM integration" marker
- **THEN** system includes a note in the error field: "Note: This is a placeholder proof - not a real Pico ZKVM proof"

### Requirement: Error handling for verification
The system SHALL provide clear error messages when verification fails due to file issues.

#### Scenario: Missing proof file
- **WHEN** proof file does not exist
- **THEN** system returns error with message "Proof file not found: <path>"

#### Scenario: Missing edited image
- **WHEN** edited image file does not exist
- **THEN** system returns error with message "Edited file not found: <path>"

#### Scenario: Malformed proof file
- **WHEN** proof file is not valid JSON
- **THEN** system returns error with message "Invalid proof format"

#### Scenario: Invalid proof commitment
- **WHEN** proof contains invalid hex-encoded commitment
- **THEN** system returns error with message "Invalid proof commitment format"
