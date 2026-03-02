## ADDED Requirements

### Requirement: Generate ZK proof using Pico ZKVM
The system SHALL generate a Zero-Knowledge Proof using Pico ZKVM that proves the edited output derives from the original image.

#### Scenario: Successful proof generation
- **WHEN** valid ProofInput containing raw image bytes, operations, and new image bytes are provided
- **THEN** system generates a ZK proof that can be verified independently

### Requirement: Read proof inputs from JSON file
The system SHALL read proof inputs from a JSON file and serialize them for Pico ZKVM.

#### Scenario: Read proof input file
- **WHEN** user provides a JSON file path containing ProofInput
- **THEN** system reads and parses the JSON, serializes using bincode, and feeds to ZKVM

#### Scenario: Invalid proof input file
- **WHEN** specified file does not exist or contains invalid JSON
- **THEN** system returns error

### Requirement: Define proof input/output types
The system SHALL define ProofInput and ProofOutput types for ZK proof generation.

#### Scenario: ProofInput structure
- **WHEN** system accepts proof input
- **THEN** ProofInput contains: raw_image_bytes (Vec<u8>), operations (Vec<EditOperation>), new_image_bytes (Vec<u8>)

#### Scenario: ProofOutput structure
- **WHEN** proof is generated
- **THEN** ProofOutput contains: raw_hash_value ([u8; 32]), new_hash_value ([u8; 32])

### Requirement: Support edit operations
The system SHALL support EditOperation enum for different image transformations.

#### Scenario: Crop operation
- **WHEN** crop operation is included
- **THEN** EditOperation::Crop contains x, y, width, height fields

#### Scenario: Brightness adjustment operation
- **WHEN** brightness adjustment is included
- **THEN** EditOperation::AdjustBrightness contains delta (i16) field

### Requirement: Integrate with Pico SDK
The system SHALL integrate with pico-sdk for prover client and ELF execution.

#### Scenario: ZKVM execution
- **WHEN** proof generation is requested
- **THEN** system uses DefaultProverClient with included ELF binary and executes proof generation

### Requirement: Hide transformation details
The system SHALL generate proofs that do not reveal specific transformation details (e.g., original crop dimensions, exact parameter values).

#### Scenario: Proof hides crop details
- **WHEN** user crops an image
- **THEN** proof attests that cropping occurred without revealing original dimensions or position

### Requirement: Prove no additional modifications
The system SHALL generate proofs that verify no modifications beyond the declared transformations occurred.

#### Scenario: No unauthorized changes
- **WHEN** proof is generated for edited media
- **THEN** verification confirms only declared transformations were applied

### Requirement: Produce verifiable proof
The system SHALL output a proof that can be verified independently without requiring access to the original media or transformation process.

#### Scenario: Independent verification
- **WHEN** proof and public verification key are available
- **THEN** any party can verify the proof without trusting the editor
