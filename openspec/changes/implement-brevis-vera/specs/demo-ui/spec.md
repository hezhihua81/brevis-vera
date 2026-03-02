## ADDED Requirements

### Requirement: RESTful API Integration
The frontend SHALL communicate with the backend via RESTful API endpoints.

#### Scenario: Connect to API server
- **GIVEN** the API server is running
- **WHEN** frontend loads
- **THEN** it SHALL connect to `/health` endpoint to verify connectivity

#### Scenario: Verify provenance via API
- **WHEN** user uploads an image file
- **THEN** frontend SHALL send POST request to `/api/verify` with multipart/form-data
- **AND** display the provenance verification result from the response

#### Scenario: Edit image via API
- **WHEN** user applies transformations (crop/resize/brightness)
- **THEN** frontend SHALL send POST request to `/api/edit` with image and parameters
- **AND** display the edited image from the base64 response

#### Scenario: Generate proof via API
- **WHEN** user initiates proof generation
- **THEN** frontend SHALL send POST request to `/api/prove` with proof input
- **AND** display the generated proof

### Requirement: Upload media for processing
The system SHALL provide a user interface for uploading media files.

#### Scenario: Successful upload
- **WHEN** user uploads an image file
- **THEN** system displays the image and proceeds to provenance verification

### Requirement: Display provenance status
The system SHALL display the provenance verification result to the user.

#### Scenario: Provenance verified
- **WHEN** uploaded media has valid signature
- **THEN** system displays "Provenance Verified" status

#### Scenario: Provenance failed
- **WHEN** uploaded media has invalid signature
- **THEN** system displays "Provenance Verification Failed" with reason

### Requirement: Apply transformations
The system SHALL provide controls for applying image transformations.

#### Scenario: Crop applied
- **WHEN** user specifies crop region and applies it
- **THEN** system displays the cropped image and records parameters

### Requirement: Generate and display proof
The system SHALL generate ZK proof after transformations and display the result.

#### Scenario: Proof generated
- **WHEN** user initiates proof generation after editing
- **THEN** system displays "Proof Generated" with ability to download

### Requirement: Verify authenticity
The system SHALL provide a verification view where users can verify edited media.

#### Scenario: Verification result displayed
- **WHEN** user provides edited media and proof for verification
- **THEN** system displays the final authenticity verdict

## API Contract

| Method | Endpoint | Request | Response |
|--------|----------|---------|----------|
| GET | `/health` | - | `{success, data: {status, version}, error}` |
| POST | `/api/verify` | `multipart: file` | `{success, data: {provenance: {...}}, error}` |
| POST | `/api/edit` | `multipart: file, crop?, resize?, brightness?` | `{success, data: {result: {...}, output_base64}, error}` |
| POST | `/api/prove` | `json: ProofInput` | `{success, data: {proof_output}, error}` |
| POST | `/api/verify-proof` | - | `{success, data: {message}, error}` (placeholder) |
