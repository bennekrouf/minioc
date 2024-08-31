
use rusoto_s3::{S3, S3Client, HeadBucketRequest, PutObjectRequest, CreateBucketRequest};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;
use dotenvy::dotenv;
use tracing::{info, error};

pub async fn upload_to_s3(
    tenant: &str,
    filename: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    info!("Starting S3 upload for tenant: {}, filename: {}", tenant, filename);

    // Load the environment variables
    dotenv().ok();
    info!("Loaded environment variables.");

    // Retrieve the necessary values from environment variables
    let s3_url = env::var("P_S3_URL")?;
    let access_key = env::var("P_S3_ACCESS_KEY")?;
    let secret_key = env::var("P_S3_SECRET_KEY")?;
    let region_name = env::var("P_S3_REGION")?;
    info!("S3 configuration loaded: URL: {}, Region: {}", s3_url, region_name);

    // Create a custom region for MinIO
    let region = rusoto_core::Region::Custom {
        name: region_name,
        endpoint: s3_url,
    };
    info!("S3 region initialized.");

    // Set up the S3 client with static credentials
    let provider = rusoto_credential::StaticProvider::new_minimal(access_key, secret_key);
    let client = S3Client::new_with(rusoto_core::HttpClient::new()?, provider, region);
    info!("S3 client initialized.");

    // Check if the bucket (tenant) exists
    let head_bucket_request = HeadBucketRequest {
        bucket: tenant.to_string(),
        expected_bucket_owner: None,
    };

    info!("Checking if bucket '{}' exists.", tenant);
    match client.head_bucket(head_bucket_request).await {
        Ok(_) => {
            info!("Bucket '{}' exists.", tenant);
        }
        Err(_) => {
            info!("Bucket '{}' does not exist. Creating it...", tenant);

            // Create the bucket if it doesn't exist
            let create_bucket_request = CreateBucketRequest {
                bucket: tenant.to_string(),
                ..Default::default() // You can set additional options if needed
            };

            match client.create_bucket(create_bucket_request).await {
                Ok(_) => info!("Bucket '{}' created successfully.", tenant),
                Err(e) => {
                    error!("Failed to create bucket '{}': {}", tenant, e);
                    return Err(format!("Failed to create bucket '{}': {}", tenant, e).into());
                }
            }
        }
    }

    // Read the file content to upload
    info!("Reading file from path: {}", file_path);
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    info!("File read successfully. Size: {} bytes", buffer.len());

    // Upload to S3
    info!("Uploading file '{}' to bucket '{}'.", filename, tenant);
    let request = PutObjectRequest {
        bucket: tenant.to_string(),
        key: filename.to_string(),
        body: Some(buffer.into()),
        ..Default::default()
    };

    match client.put_object(request).await {
        Ok(_) => info!("File uploaded to S3 successfully."),
        Err(e) => {
            error!("Failed to upload file to S3: {}", e);
            return Err(format!("Failed to upload file to S3: {}", e).into());
        }
    }

    Ok(())
}

