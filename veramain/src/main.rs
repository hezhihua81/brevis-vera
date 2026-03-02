mod api;
mod image_editing;
mod provenance;

use clap::{Parser, Subcommand};
use image_editing::edit_image;
use pico_sdk::client::DefaultProverClient;
use provenance::verify_c2pa_provenance;
use serde_json;
use std::{fs::File, io::Read};
use zkapp::types::{ProofInput, ProofOutput};

#[derive(Parser)]
#[command(name = "brevis-vera")]
#[command(about = "Digital media authenticity attestation system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start RESTful API server
    Serve {
        /// Host to bind to (default: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind to (default: 3000)
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    /// Verify C2PA provenance of a media file
    Verify {
        /// Path to the media file (JPEG/PNG)
        file: String,
    },
    /// Edit an image with transformations
    Edit {
        /// Path to the input image
        input: String,
        /// Path to the output image
        output: String,
        /// Crop region (x,y,width,height)
        #[arg(long)]
        crop: Option<String>,
        /// Resize to specified dimensions (width,height)
        #[arg(long)]
        resize: Option<String>,
        /// Adjust brightness (-100 to 100)
        #[arg(long)]
        brightness: Option<i32>,
    },
    /// Generate ZK proof for edited media
    Prove { proof_input_file: String },
    /// Verify ZK proof
    VerifyProof {},
}

const ELF: &[u8] = include_bytes!("../../zkapp/elf/riscv32im-pico-zkvm-elf");

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve { host, port } => {
            api::run_server(host, *port).await;
        }
        Commands::Verify { file } => {
            let result = verify_c2pa_provenance(file);
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Edit {
            input,
            output,
            crop,
            resize,
            brightness,
        } => {
            let result = edit_image(input, output, crop.clone(), resize.clone(), *brightness);
            if result.success {
                println!("Image edited successfully!");
                println!("Output: {}", result.output_path);
                println!("Transformations applied:");
                if result.params.crop.is_some() {
                    println!("  - Crop");
                }
                if result.params.resize.is_some() {
                    println!("  - Resize");
                }
                if result.params.brightness.is_some() {
                    println!("  - Brightness adjustment");
                }
            } else {
                eprintln!(
                    "Error: {}",
                    result.error.unwrap_or("Unknown error".to_string())
                );
                std::process::exit(1);
            }
        }
        Commands::Prove { proof_input_file } => {
            let proof_input: ProofInput = read_proof_inputs_from_file(proof_input_file).unwrap();
            let encoded_input = bincode::serialize(&proof_input).unwrap();
            let client = DefaultProverClient::new(ELF);
            let mut stdin_builder = client.new_stdin_builder();
            stdin_builder.write_slice(&encoded_input);

            let pv_stream = {
                let proof = client.prove_fast(stdin_builder).unwrap();
                proof.pv_stream.unwrap()
            };

            println!(
                "verified: {:?}",
                bincode::deserialize::<ProofOutput>(&pv_stream).unwrap()
            );
        }
        Commands::VerifyProof {} => {
            // TODO
        }
    }
}

fn read_proof_inputs_from_file(file_path: &str) -> Result<ProofInput, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let proof_input: ProofInput = serde_json::from_str(&json)?;
    Ok(proof_input)
}
