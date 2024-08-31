
mod minioc_service;
mod utils;

use tonic::transport::Server;
use std::env;
use std::path::Path;
use tonic_reflection::server::Builder;
use crate::minioc_service::MyMiniocService;
use crate::minioc_service::minioc::minioc_service_server::MiniocServiceServer;
use dotenvy::from_path;
use std::sync::Arc;
use messengerc::{connect_to_messenger_service, MessagingService};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    // Retrieve the necessary values from environment variables
    let ip = env::var("MINIOC_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("MINIOC_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse().unwrap();

    let minioc_service = MyMiniocService::default();

    let mes = format!("Minioc listening on {}", &addr);
    println!("{}", &mes);

    let tag = env::var("MINIOC_TAG").expect("Missing 'port' environment variable");

    let messenger_client = connect_to_messenger_service().await
        .ok_or("Failed to connect to messenger service")?;

    let messaging_service = MessagingService::new(
        Arc::new(Mutex::new(messenger_client)),
        tag.clone(),
    );
    // Example: Publish a message (can be removed or modified as needed)
    let _ = messaging_service.publish_message(mes, Some(vec![tag])).await;

    // Include the descriptor set for reflection
    let descriptor_set = include_bytes!(concat!(env!("OUT_DIR"), "/minioc_descriptor.bin"));
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(descriptor_set)
        .build_v1()?;

    // Build and start the gRPC server
    Server::builder()
        .add_service(MiniocServiceServer::new(minioc_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}

