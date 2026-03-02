## Context

Brevis Vera is a digital media authenticity attestation system that enables:
1. **Provenance Verification**: Verify C2PA-signed media provenance using official C2PA Rust SDK
2. **Image Editing**: Transform images with crop (mandatory) and other operations
3. **ZK Proof Generation**: Generate Zero-Knowledge Proofs proving edit integrity without revealing transformation details
4. **Verification**: Independent verification of edited media against ZK proofs

The system targets use cases where content creators need to prove their edits are legitimate without revealing the original assets or specific transformation parameters.

## Goals / Non-Goals

**Goals:**
- Build a functional prototype demonstrating the complete capture → edit → prove → verify flow
- Integrate C2PA Rust SDK (stable, lightweight) for real C2PA-signed media provenance verification
- Integrate image editing with crop as mandatory transformation
- Generate ZK proofs using Pico ZKVM that prove edit integrity
- Create a verification CLI that can independently verify proofs
- Provide a demo UI for user-friendly interaction

**Non-Goals:**
- Full production-readiness (prototype focus)
- X.509 certificate chain authentication (skip for prototype, verify single signature only)
- Video support
- Performance benchmarking

## Decisions

### 1. Tech Stack
- **Language**: Rust (for ZKVM integration and performance)
- **C2PA SDK**: C2PA Rust SDK (stable, lightweight) for provenance verification
- **ZKVM**: Pico ZKVM for proof generation
- **Image Processing**: Photon (Rust-based) or image crate
- **Frontend**: Web-based UI (simple HTML/JS)

**Alternative considered**: Python was considered for C2PA SDK availability, but C2PA Rust SDK provides better integration for the prototype.

### 2. Architecture
- **Modular Pipeline**: Provenance → Editing → ZK Proof → Verification
- **CLI-first**: Core functionality in CLI, UI wraps CLI operations
- **File-based**: JSON/metadata files for passing data between stages

**Alternative considered**: REST API was considered but adds complexity not needed for prototype.

### 3. ZK Proof Approach
- Prove that output derives from signed original
- Hide transformation details (e.g., original crop dimensions)
- Use circuit-based ZK proving

### 4. Image Editing
- Support crop (mandatory)
- Support at least one additional: resize, brightness adjustment, or filter
- Programmatic editing acceptable for prototype

## Risks / Trade-offs

- **ZKVM Learning Curve**: Pico ZKVM has a learning curve → Mitigation: Start with simple circuits, iterate
- **Proof Generation Time**: ZK proofs can be slow → Mitigation: Accept prototype-level performance
- **C2PA SDK Learning Curve**: Official C2PA Rust SDK has its own API → Mitigation: Start with basic verification, iterate
- **Image Format Support**: Limited format support initially → Mitigation: Focus on common formats (PNG, JPEG)

## Open Questions

- Which additional image transformation to support beyond crop?
- How to structure the ZK circuit for proving edit integrity?
- What metadata format to use for provenance?
