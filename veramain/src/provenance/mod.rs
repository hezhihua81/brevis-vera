// Provenance verification module using C2PA SDK
use c2pa::Reader;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceInfo {
    pub is_verified: bool,
    pub has_manifest: bool,
    pub claim_label: Option<String>,
    pub json_output: String,
    pub error: Option<String>,
    pub validation_errors: Vec<String>,
    pub file_hash: Option<String>,
}

pub fn verify_c2pa_provenance(file_path: &str) -> ProvenanceInfo {
    let path = Path::new(file_path);

    // Check if file exists and compute hash
    let file_hash = if path.exists() {
        match std::fs::read(path) {
            Ok(data) => {
                let mut hasher = Sha256::new();
                hasher.update(&data);
                Some(format!("{:x}", hasher.finalize()))
            }
            Err(_) => None,
        }
    } else {
        return ProvenanceInfo {
            is_verified: false,
            has_manifest: false,
            claim_label: None,
            json_output: String::new(),
            error: Some(format!("File not found: {}", file_path)),
            validation_errors: vec![],
            file_hash: None,
        };
    };

    // Use C2PA Reader API to verify the manifest and signature
    match Reader::from_file(path) {
        Ok(reader) => {
            // Get JSON output for debugging
            let json_output = reader.json();

            // Check if there is an active manifest
            let has_manifest = reader.active_manifest().is_some();

            // Get claim label from manifest if available
            let claim_label = reader
                .active_manifest()
                .map(|_m| "C2PA manifest present".to_string());

            // Verify signature by checking validation status
            // Only collect actual failures, not informational or success statuses
            let validation_results = reader.validation_results();

            // Collect failures from active manifest
            let validation_errors: Vec<String> = validation_results
                .as_ref()
                .and_then(|vr| vr.active_manifest())
                .map(|am| {
                    am.failure()
                        .iter()
                        .map(|status| format!("{:?}", status))
                        .collect()
                })
                .unwrap_or_default();

            // Also check ingredient deltas for failures
            let mut all_errors = validation_errors;
            if let Some(vr) = validation_results.as_ref() {
                if let Some(deltas) = vr.ingredient_deltas() {
                    for delta in deltas {
                        let delta_errors: Vec<String> = delta
                            .validation_deltas()
                            .failure()
                            .iter()
                            .map(|status| format!("{:?}", status))
                            .collect();
                        all_errors.extend(delta_errors);
                    }
                }
            }

            // Determine if verification succeeded: has manifest AND no failures
            let is_verified = all_errors.is_empty() && has_manifest;

            ProvenanceInfo {
                is_verified,
                has_manifest,
                claim_label,
                json_output,
                error: None,
                validation_errors: all_errors,
                file_hash,
            }
        }
        Err(e) => {
            // No C2PA manifest found or verification failed
            ProvenanceInfo {
                is_verified: false,
                has_manifest: false,
                claim_label: None,
                json_output: String::new(),
                error: Some(format!("C2PA error: {}", e)),
                validation_errors: vec![],
                file_hash,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_info_creation() {
        let info = ProvenanceInfo {
            is_verified: true,
            has_manifest: true,
            claim_label: Some("test_claim".to_string()),
            json_output: "{}".to_string(),
            error: None,
            validation_errors: vec![],
            file_hash: None,
        };

        assert!(info.is_verified);
        assert!(info.has_manifest);
    }

    #[test]
    fn test_nonexistent_file() {
        let result = verify_c2pa_provenance("/nonexistent/file.jpg");
        assert!(!result.is_verified);
        assert!(result.error.is_some());
    }
}
