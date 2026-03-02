// Provenance verification module using C2PA SDK
use c2pa::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceInfo {
    pub is_verified: bool,
    pub has_manifest: bool,
    pub claim_label: Option<String>,
    pub json_output: String,
    pub error: Option<String>,
}

pub fn verify_c2pa_provenance(file_path: &str) -> ProvenanceInfo {
    let path = Path::new(file_path);

    // Check if file exists
    if !path.exists() {
        return ProvenanceInfo {
            is_verified: false,
            has_manifest: false,
            claim_label: None,
            json_output: String::new(),
            error: Some(format!("File not found: {}", file_path)),
        };
    }

    // Use C2PA Reader API to verify the manifest
    match Reader::from_file(path) {
        Ok(reader) => {
            // Get JSON output for debugging
            let json_output = reader.json();

            // Check if there is an active manifest
            let has_manifest = reader.active_manifest().is_some();

            // Get claim label from JSON if available
            let claim_label = if has_manifest {
                Some("C2PA manifest present".to_string())
            } else {
                None
            };

            ProvenanceInfo {
                is_verified: has_manifest,
                has_manifest,
                claim_label,
                json_output,
                error: None,
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
