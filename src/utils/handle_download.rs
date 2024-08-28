
use tokio::sync::mpsc::Sender;
use tokio::io::AsyncReadExt;
use rusoto_s3::{S3, GetObjectRequest};
use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use std::env;

use crate::minioc_service::minioc::FileChunk;

pub async fn handle_download(
    tenant: String,
    filename: String,
    file_sender: Sender<Result<FileChunk, tonic::Status>>,
) -> Result<(), Box<dyn std::error::Error>> {

    // Load environment variables
    dotenvy::dotenv().ok();

    let s3_url = env::var("P_S3_URL")?;
    let access_key = env::var("P_S3_ACCESS_KEY")?;
    let secret_key = env::var("P_S3_SECRET_KEY")?;
    let region_name = env::var("P_S3_REGION")?;

    // Create a custom region for S3 (or MinIO)
    let region = Region::Custom {
        name: region_name,
        endpoint: s3_url,
    };

    // Set up the S3 client with static credentials
    let provider = StaticProvider::new_minimal(access_key, secret_key);
    let client = rusoto_s3::S3Client::new_with(rusoto_core::HttpClient::new()?, provider, region);

    // Prepare the request to fetch the file
    let request = GetObjectRequest {
        bucket: tenant.clone(),
        key: filename.clone(),
        ..Default::default()
    };

    // Fetch the file from S3
    let result = client.get_object(request).await?;

    // Get the file stream from the response
    let stream = result.body.unwrap();

    // Stream the file data back to the client in chunks
    let mut stream = stream.into_async_read();
    let mut buffer = [0u8; 1024]; // Read in 1KB chunks

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        let chunk = FileChunk {
            data: buffer[..n].to_vec(),
        };

        // Send the chunk to the client
        file_sender.send(Ok(chunk)).await.map_err(|_| {
            tonic::Status::internal("Failed to send file chunk")
        })?;
    }

    Ok(())
}
