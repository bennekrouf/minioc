
use tonic::{Request, Response, Status, Streaming};
use tokio::sync::mpsc;
use futures::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

pub mod minioc {
    tonic::include_proto!("minioc"); // The proto package name
}

use minioc::minioc_service_server::MiniocService;
use minioc::{FileChunk, UploadResponse, DownloadRequest};

use crate::utils::handle_upload::handle_upload;
use crate::utils::handle_download::handle_download;

#[derive(Debug, Default)]
pub struct MyMiniocService;

#[tonic::async_trait]
impl MiniocService for MyMiniocService {
    async fn stream_upload(
        &self,
        request: Request<Streaming<FileChunk>>,
    ) -> Result<Response<UploadResponse>, Status> {
        let tenant = request
            .metadata()
            .get("tenant")
            .ok_or_else(|| Status::invalid_argument("Missing tenant"))?
            .to_str()
            .map_err(|_| Status::invalid_argument("Invalid tenant"))?
            .to_string();
        let filename = request
            .metadata()
            .get("filename")
            .ok_or_else(|| Status::invalid_argument("Missing filename"))?
            .to_str()
            .map_err(|_| Status::invalid_argument("Invalid filename"))?
            .to_string();

        let (file_sender, file_receiver) = mpsc::channel(100);
        let mut stream = request.into_inner();

        tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(chunk) => {
                        if let Err(_) = file_sender.send(chunk).await {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        let _ = handle_upload(tenant, filename, file_receiver).await;

        let reply = UploadResponse {
            message: "File uploaded successfully!".to_string(),
        };

        Ok(Response::new(reply))
    }

    type downloadFileStream = ReceiverStream<Result<FileChunk, Status>>;

    async fn download_file(
        &self,
        request: Request<DownloadRequest>,
    ) -> Result<Response<Self::downloadFileStream>, Status> {
        let into_inner = request.into_inner();
        let tenant = into_inner.tenant;
        let filename = into_inner.filename;

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            if let Err(e) = handle_download(tenant, filename, tx).await {
                eprintln!("Error reading and streaming file: {:?}", e);
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

