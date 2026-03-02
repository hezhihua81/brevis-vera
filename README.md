# Brevis Vera

Digital Media Authenticity Attestation System

## Overview

Brevis Vera is a prototype system for verifying the authenticity of digitally edited media. It enables:

1. **C2PA Provenance Verification**: Verify that media was signed using the C2PA (Coalition for Content Provenance and Authenticity) standard
2. **Image Editing**: Transform images with crop, resize, and brightness adjustments
3. **ZK Proof Generation**: Generate Zero-Knowledge Proofs proving edit integrity without revealing transformation details
4. **Independent Verification**: Verify proofs without needing access to the original media

## Quick Start

### Prerequisites

- Rust 1.75+ (nightly recommended)
- Cargo

### Build

```bash
cargo build --release
```

### Start REST API Server

```bash
# Default: 127.0.0.1:3000
cargo run --package veramain -- serve

# Custom host and port
cargo run --package veramain -- serve --host 0.0.0.0 --port 8080
```

### CLI Commands

#### Start API Server

```bash
# Default: 127.0.0.1:3000
cargo run --package veramain -- serve

# Custom port
cargo run --package veramain -- serve --port 8080
```

#### Verify C2PA Provenance

```bash
cargo run --package veramain -- verify <image_file>
```

Example:
```bash
cargo run --package veramain -- verify photo.jpg
```

Output:
```json
{
  "is_verified": true,
  "has_manifest": true,
  "claim_label": "C2PA manifest present",
  "json_output": "{...}",
  "error": null
}
```

#### Edit Image

```bash
cargo run --package veramain -- edit <input> <output> [options]
```

Options:
- `--crop <x,y,width,height>` - Crop region
- `--resize <width,height>` - Resize to dimensions
- `--brightness <-100 to 100>` - Adjust brightness

Example:
```bash
cargo run --package veramain -- edit input.jpg output.jpg --crop "10,10,200,200" --brightness 20
```

#### Generate ZK Proof

```bash
cargo run --package veramain -- prove <proof_input_file>
```

Example:
```bash
cargo run --package veramain -- prove proof_input.json
```

#### Verify Proof

```bash
cargo run --package veramain -- verify-proof
```

Note: Currently a placeholder implementation.

## RESTful API

The system provides a RESTful API for frontend integration.

### Base URL

```
http://localhost:3000
```

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Web UI (index.html) |
| GET | `/health` | Health check |
| POST | `/api/verify` | Verify C2PA provenance |
| POST | `/api/edit` | Edit image |
| POST | `/api/prove` | Generate ZK proof |
| POST | `/api/verify-proof` | Verify ZK proof |

### API Reference

#### Health Check

```bash
GET /health
```

Response:
```json
{
  "success": true,
  "data": {
    "status": "ok",
    "version": "0.1.0"
  },
  "error": null
}
```

#### Verify Provenance

```bash
POST /api/verify
Content-Type: multipart/form-data

# Body: file (image file)
```

Response:
```json
{
  "success": true,
  "data": {
    "provenance": {
      "is_verified": true,
      "has_manifest": true,
      "claim_label": "C2PA manifest present",
      "json_output": "...",
      "error": null,
      "validation_errors": []
    }
  },
  "error": null
}
```

#### Edit Image

```bash
POST /api/edit
Content-Type: multipart/form-data

# Body:
#   - file: image file (required)
#   - crop: "x,y,width,height" (optional)
#   - resize: "width,height" (optional)
#   - brightness: -100 to 100 (optional)
```

Response:
```json
{
  "success": true,
  "data": {
    "result": {
      "success": true,
      "output_path": "...",
      "params": {
        "crop": { "x": 0, "y": 0, "width": 800, "height": 600 },
        "resize": null,
        "brightness": 20
      },
      "error": null
    },
    "output_base64": "..."
  },
  "error": null
}
```

#### Generate ZK Proof

```bash
POST /api/prove
Content-Type: application/json

# Body: ProofInput JSON object
```

Response:
```json
{
  "success": true,
  "data": {
    "proof_output": { ... }
  },
  "error": null
}
```

### cURL Examples

```bash
# Health check
curl http://127.0.0.1:3000/health

# Verify image provenance
curl -X POST -F "file=@image.jpg" http://127.0.0.1:3000/api/verify

# Edit image with resize
curl -X POST -F "file=@input.jpg" -F "resize=800,600" http://127.0.0.1:3000/api/edit

# Edit image with brightness
curl -X POST -F "file=@input.jpg" -F "brightness=20" http://127.0.0.1:3000/api/edit
```

## Demo UI

A web-based demo is served automatically when the API server starts. Simply open your browser and navigate to:

```
http://127.0.0.1:3000
```

The demo UI provides an interactive 4-step workflow:

1. **Upload** - Upload a C2PA-signed image to verify its provenance
2. **Edit** - Apply transformations (crop, resize, brightness)
3. **Prove** - Generate a ZK proof attesting to the edit integrity
4. **Verify** - Verify the generated proof

All operations use the REST API backend.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Pipeline                              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────┐    ┌──────────────┐    ┌───────────┐    │
│  │ C2PA     │───▶│ Image        │───▶│ ZK Proof  │    │
│  │ Verify   │    │ Editing      │    │ Generate  │    │
│  └──────────┘    └──────────────┘    └───────────┘    │
│                                                  │      │
│                                                  ▼      │
│                                        ┌─────────────┐  │
│                                        │ Verification│  │
│                                        └─────────────┘  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

## Tech Stack

- **Language**: Rust
- **C2PA SDK**: c2pa (v0.76)
- **Image Processing**: image crate
- **CLI**: clap
- **HTTP Server**: Axum
- **Async Runtime**: Tokio
- **ZK Proof**: Pico ZKVM

## Limitations

- ZK proof verification is not fully implemented
- No actual C2PA test files included (obtain from C2PA tools)
- Video support not implemented
- Performance not optimized

## License

Apache 2.0 / MIT
