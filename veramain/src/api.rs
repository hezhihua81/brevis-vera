// RESTful API module for vera
use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use crate::image_editing::{edit_image, EditResult};
use crate::provenance::{verify_c2pa_provenance, ProvenanceInfo};
use serde::Serialize;
use sha2::{Sha256, Digest};
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use zkapp::types::{ProofInput, ProofOutput};

const ELF: &[u8] = include_bytes!("../../zkapp/elf/riscv32im-pico-zkvm-elf");

// Embed the UI HTML
const UI_HTML: &str = include_str!("ui/index.html");

#[derive(Clone)]
struct AppState {
    upload_dir: PathBuf,
}

// ============ Request/Response DTOs ============

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

// Custom error type for API responses
struct ApiError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(ApiResponse::<()>::error(&self.message));
        (self.status, body).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub provenance: ProvenanceInfo,
}

#[derive(Debug, Serialize)]
pub struct EditResponse {
    pub result: EditResult,
    pub output_base64: Option<String>,
    pub output_hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProveResponse {
    pub proof_output: ProofOutput,
}

#[derive(Debug, Serialize)]
pub struct ProveMockResponse {
    pub raw_image_hash: String,
    pub new_image_hash: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

// ============ API Routes ============

/// Serve the main UI page
async fn ui_index() -> Html<&'static str> {
    Html(UI_HTML)
}

/// Health check endpoint
async fn health() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse::success(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}

/// Verify C2PA provenance of an uploaded image
async fn verify(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<VerifyResponse>>, ApiError> {
    let mut file_data: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to parse multipart: {}", e) }
    })? {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let filename = field
                .file_name()
                .unwrap_or("image.jpg")
                .to_string();
            let bytes = field.bytes().await.map_err(|e| {
                ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to read file: {}", e) }
            })?;
            file_data = Some((bytes.to_vec(), filename));
            break;
        }
    }

    let (bytes, filename) = file_data
        .ok_or_else(|| ApiError { status: StatusCode::BAD_REQUEST, message: "No file provided".to_string() })?;

    // Save uploaded file temporarily
    let file_path = state.upload_dir.join(&filename);
    tokio::fs::write(&file_path, &bytes)
        .await
        .map_err(|e| ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: format!("Failed to save file: {}", e) })?;

    // Verify provenance
    let result = verify_c2pa_provenance(file_path.to_str().unwrap_or(""));

    // Clean up temp file
    let _ = tokio::fs::remove_file(&file_path).await;

    Ok(Json(ApiResponse::success(VerifyResponse {
        provenance: result,
    })))
}

/// Edit an image with transformations
async fn edit(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<EditResponse>>, ApiError> {
    let mut file_data: Option<(Vec<u8>, String)> = None;
    let mut crop: Option<String> = None;
    let mut resize: Option<String> = None;
    let mut brightness: Option<i32> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to parse multipart: {}", e) }
    })? {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                let filename = field
                    .file_name()
                    .unwrap_or("image.jpg")
                    .to_string();
                let bytes = field.bytes().await.map_err(|e| {
                    ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to read file: {}", e) }
                })?;
                file_data = Some((bytes.to_vec(), filename));
            }
            "crop" => {
                crop = Some(field.text().await.unwrap_or_default());
            }
            "resize" => {
                resize = Some(field.text().await.unwrap_or_default());
            }
            "brightness" => {
                brightness = field.text().await.unwrap_or_default().parse().ok();
            }
            _ => {}
        }
    }

    let (bytes, filename) = file_data
        .ok_or_else(|| ApiError { status: StatusCode::BAD_REQUEST, message: "No file provided".to_string() })?;

    // Save uploaded file temporarily
    let input_filename = format!("input_{}", filename);
    let input_path = state.upload_dir.join(&input_filename);
    tokio::fs::write(&input_path, &bytes)
        .await
        .map_err(|e| ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: format!("Failed to save file: {}", e) })?;

    // Generate output filename
    let output_filename = format!("output_{}", filename);
    let output_path = state.upload_dir.join(&output_filename);

    // Clone paths for use in closure
    let input_path_clone = input_path.clone();
    let output_path_clone = output_path.clone();

    // Edit image (blocking operation)
    let result = tokio::task::spawn_blocking(move || {
        edit_image(
            input_path_clone.to_str().unwrap_or(""),
            output_path_clone.to_str().unwrap_or(""),
            crop,
            resize,
            brightness,
        )
    })
    .await
    .map_err(|e| ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: format!("Task error: {}", e) })?;

    // Read output file as base64 and compute hash if successful
    let (output_base64, output_hash) = if result.success {
        match tokio::fs::read(&output_path).await {
            Ok(bytes) => {
                use base64::Engine;
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                let hash = format!("{:x}", hasher.finalize());
                (Some(b64), Some(hash))
            }
            Err(_) => (None, None),
        }
    } else {
        (None, None)
    };

    // Clean up temp files
    let _ = tokio::fs::remove_file(&input_path).await;
    if !result.success {
        let _ = tokio::fs::remove_file(&output_path).await;
    }

    Ok(Json(ApiResponse::success(EditResponse {
        result,
        output_base64,
        output_hash,
    })))
}

/// Generate ZK proof for edited media
async fn prove(Json(payload): Json<ProofInput>) -> Result<Json<ApiResponse<ProveResponse>>, ApiError> {
    let proof_input = payload;

    let proof_output = tokio::task::spawn_blocking(move || {
        let client = pico_sdk::client::DefaultProverClient::new(ELF);
        let encoded_input = bincode::serialize(&proof_input).unwrap();
        let mut stdin_builder = client.new_stdin_builder();
        stdin_builder.write_slice(&encoded_input);

        let pv_stream = {
            let proof = client.prove_fast(stdin_builder).unwrap();
            proof.pv_stream.unwrap()
        };

        bincode::deserialize::<ProofOutput>(&pv_stream).unwrap()
    })
    .await
    .map_err(|e| ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: format!("Task error: {}", e) })?;

    Ok(Json(ApiResponse::success(ProveResponse { proof_output })))
}

/// Mock prove - returns SHA-256 hashes of raw and new images
async fn mock_prove(
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<ProveMockResponse>>, ApiError> {
    let mut raw_image_bytes: Option<Vec<u8>> = None;
    let mut new_image_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to parse multipart: {}", e) }
    })? {
        let name = field.name().unwrap_or("").to_string();
        let bytes = field.bytes().await.map_err(|e| {
            ApiError { status: StatusCode::BAD_REQUEST, message: format!("Failed to read file: {}", e) }
        })?;

        match name.as_str() {
            "raw_image" => raw_image_bytes = Some(bytes.to_vec()),
            "new_image" => new_image_bytes = Some(bytes.to_vec()),
            _ => {}
        }
    }

    let raw_bytes = raw_image_bytes
        .ok_or_else(|| ApiError { status: StatusCode::BAD_REQUEST, message: "No raw_image provided".to_string() })?;
    let new_bytes = new_image_bytes
        .ok_or_else(|| ApiError { status: StatusCode::BAD_REQUEST, message: "No new_image provided".to_string() })?;

    let mut raw_hasher = Sha256::new();
    raw_hasher.update(&raw_bytes);
    let raw_image_hash = format!("{:x}", raw_hasher.finalize());

    let mut new_hasher = Sha256::new();
    new_hasher.update(&new_bytes);
    let new_image_hash = format!("{:x}", new_hasher.finalize());

    Ok(Json(ApiResponse::success(ProveMockResponse {
        raw_image_hash,
        new_image_hash,
    })))
}

/// Verify ZK proof (placeholder)
async fn verify_proof() -> Json<ApiResponse<String>> {
    Json(ApiResponse::error("Proof verification not yet implemented"))
}

// ============ Server Setup ============

pub async fn run_server(host: &str, port: u16) {
    let upload_dir = std::env::temp_dir().join("vera_uploads");
    std::fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

    let state = AppState {
        upload_dir: upload_dir.clone(),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router with increased body limit (20MB for file uploads)
    let app = Router::new()
        .route("/", get(ui_index))
        .route("/health", get(health))
        .route("/api/verify", post(verify).layer(DefaultBodyLimit::max(20 * 1024 * 1024)))
        .route("/api/edit", post(edit).layer(DefaultBodyLimit::max(20 * 1024 * 1024)))
        .route("/api/prove", post(prove).layer(DefaultBodyLimit::max(20 * 1024 * 1024)))
        .route("/api/mock-prove", post(mock_prove).layer(DefaultBodyLimit::max(20 * 1024 * 1024)))
        .route("/api/verify-proof", post(verify_proof).layer(DefaultBodyLimit::max(20 * 1024 * 1024)))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);
    println!("📁 Upload directory: {:?}", upload_dir);
    println!("📋 API endpoints:");
    println!("   GET  /health          - Health check");
    println!("   POST /api/verify       - Verify C2PA provenance");
    println!("   POST /api/edit        - Edit image");
    println!("   POST /api/prove       - Generate ZK proof");
    println!("   POST /api/mock-prove  - Mock prove (returns hashes)");
    println!("   POST /api/verify-proof - Verify ZK proof");

    axum::serve(listener, app).await.unwrap();
}
