
use dotenvy::dotenv;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::StaticProvider;
use rusoto_s3::{S3Client, S3, PutObjectRequest};
use std::env;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
// use tokio::task;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from the .env file
    dotenv().ok();

    // Retrieve the necessary values from environment variables
    let s3_url = env::var("P_S3_URL")?;
    let access_key = env::var("P_S3_ACCESS_KEY")?;
    let secret_key = env::var("P_S3_SECRET_KEY")?;
    let region_name = env::var("P_S3_REGION")?;
    let bucket_name = env::var("P_S3_BUCKET")?;

    // Create a custom region for MinIO (MinIO doesn't use AWS regions directly)
    let region = Region::Custom {
        name: region_name,
        endpoint: s3_url,
    };

    // Set up the S3 client with static credentials
    let provider = StaticProvider::new_minimal(access_key, secret_key);
    let client = S3Client::new_with(HttpClient::new()?, provider, region);

    // Create a file programmatically
    let file_path = Path::new("temp_file.txt");
    let file_content = "Hello, MinIO from Rust!";
    write_to_file(file_path, file_content).await?;

    // Read the file content to upload
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // Example: Put a new object in the S3 bucket
    let request = PutObjectRequest {
        bucket: bucket_name,
        key: "uploaded_file.txt".to_string(),  // The key is the name of the file in S3
        body: Some(buffer.into()),
        ..Default::default()
    };

    client.put_object(request).await?;
    println!("File uploaded successfully!");

    // Optionally, delete the file after upload
    tokio::fs::remove_file(file_path).await?;
    println!("Temporary file deleted.");

    Ok(())
}

// Async function to write content to a file
async fn write_to_file(path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    println!("File created: {:?}", path);
    Ok(())
}

