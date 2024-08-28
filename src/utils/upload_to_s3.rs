
use rusoto_s3::S3;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;
use dotenvy::dotenv;

pub async fn upload_to_s3(
    tenant: &str,
    filename: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    // Load the environment variables
    // dotenvy::dotenv().ok();
    dotenv().ok();
    // Retrieve the necessary values from environment variables
    let s3_url = env::var("P_S3_URL")?;
    let access_key = env::var("P_S3_ACCESS_KEY")?;
    let secret_key = env::var("P_S3_SECRET_KEY")?;
    let region_name = env::var("P_S3_REGION")?;

    // Create a custom region for MinIO
    let region = rusoto_core::Region::Custom {
        name: region_name,
        endpoint: s3_url,
    };

    // Set up the S3 client with static credentials
    let provider = rusoto_credential::StaticProvider::new_minimal(access_key, secret_key);
    let client = rusoto_s3::S3Client::new_with(rusoto_core::HttpClient::new()?, provider, region);

    // Read the file content to upload
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;


    println!("TENANT !!!!!! {} KEY : {}", &tenant, &key);

    // Upload to S3
    let request = rusoto_s3::PutObjectRequest {
        bucket: tenant.to_string(),
        key: filename.to_string(),
        body: Some(buffer.into()),
        ..Default::default()
    };

    client.put_object(request).await?;
    println!("File uploaded to S3 successfully!");

    Ok(())
}


