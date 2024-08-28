
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;

use crate::minioc_service::minioc::FileChunk;
use crate::utils::upload_to_s3::upload_to_s3;

pub async fn handle_upload(
    tenant: String,
    filename: String,
    mut file_receiver: mpsc::Receiver<FileChunk>,
) -> Result<(), Box<dyn std::error::Error>> {

    let file_path = format!("/tmp/{}", filename);
    let mut file = tokio::fs::File::create(&file_path).await?;

    while let Some(chunk) = file_receiver.recv().await {
        file.write_all(&chunk.data).await?;
    }

    // Upload the file to S3 after fully receiving it
    upload_to_s3(&tenant, &filename, &file_path).await?;

    // Clean up the temporary file
    tokio::fs::remove_file(file_path).await?;
    Ok(())
}


