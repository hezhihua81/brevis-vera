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

### CLI Commands

#### Verify C2PA Provenance

```bash
brevis-vera verify <image_file>
```

Example:
```bash
brevis-vera verify photo.jpg
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
brevis-vera edit <input> <output> [options]
```

Options:
- `--crop <x,y,width,height>` - Crop region
- `--resize <width,height>` - Resize to dimensions
- `--brightness <-100 to 100>` - Adjust brightness

Example:
```bash
brevis-vera edit input.jpg output.jpg --crop "10,10,200,200" --brightness 20
```

#### Generate ZK Proof

```bash
brevis-vera prove <original> <edited> <output>
```

Example:
```bash
brevis-vera prove original.jpg edited.jpg proof.json
```

#### Verify Proof

```bash
brevis-vera verify-proof <proof_file> <edited_image>
```

Example:
```bash
brevis-vera verify-proof proof.json edited.jpg
```

## Demo UI

A web-based demo is available at `src/ui/index.html`. Open it in a browser to interactively test the full flow.

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
- **ZK Proof**: Placeholder (Pico ZKVM integration pending)

## Limitations

- ZK proof generation is currently a placeholder
- No actual C2PA test files included (obtain from C2PA tools)
- Video support not implemented
- Performance not optimized

## License

Apache 2.0 / MIT
