
mod minioc_service;
mod utils;

use tonic::transport::Server;
use std::env;
use tonic_reflection::server::Builder;
use crate::minioc_service::MyMiniocService;
use crate::minioc_service::minioc::minioc_service_server::MiniocServiceServer;
use dotenvy::from_path;
use std::path::Path;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    // Retrieve the necessary values from environment variables
    let ip = env::var("MINIOC_DOMAIN").expect("Missing 'domain' environment variable");
    let port = env::var("MINIOC_PORT").expect("Missing 'port' environment variable");
    let addr = format!("{}:{}", ip, port).parse().unwrap();

    let minioc_service = MyMiniocService::default();
    println!("Server listening on {}", addr);

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

