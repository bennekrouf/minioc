
# S3 Connector - Minioc for Minio connector

This Rust-based gRPC service allows you to stream a file to an S3-compatible storage, such as MinIO, by specifying a tenant, which maps to an S3 bucket. The service only allows you to upload one file at a time.

## Features
- **File Upload**: Stream a file to the service using gRPC. The file is uploaded to the specified S3 bucket (identified by the `tenant` metadata).
- **gRPC Service**: Uses `tonic` for the gRPC implementation.
- **S3 Integration**: Integrates with S3-compatible storage services using `rusoto_s3`.

## Setup

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/s3-connector.git
    cd s3-connector
    ```

2. Set up the necessary environment variables in a `.env` file:

    ```env
    P_S3_URL=http://localhost:9000
    P_S3_ACCESS_KEY=minioadmin
    P_S3_SECRET_KEY=minioadmin
    P_S3_REGION=us-east-1
    ```

3. Run the server:

    ```bash
    cargo run
    ```

   The server will start listening on `[::1]:50051`.

## gRPC Metadata Requirements

- **tenant**: The tenant identifier (maps to the S3 bucket).
- **filename**: The name of the file being uploaded.

## Using grpcurl

You can use `grpcurl` to interact with the gRPC service. Here's an example of uploading a file using `grpcurl`:

1. Create a file to upload:

    ```bash
    echo "Hello, gRPC!" > test_file.txt
    ```

2. Use `grpcurl` to upload the file in a streaming manner:

    ```bash
    grpcurl -plaintext \
      -d @ \
      -rpc-header "tenant:minio-bucket" \
      -rpc-header "filename:test_file.txt" \
      0.0.0.0:50051 minioc.Minioc/streamUpload < test_file.txt
    ```

This will stream the contents of `test_file.txt` to the gRPC server, and the file will be uploaded to the S3 bucket named `minio-bucket`.

## Environment Variables

Ensure the following environment variables are set in your `.env` file:

- `P_S3_URL`: The URL of your S3-compatible service (e.g., MinIO).
- `P_S3_ACCESS_KEY`: The access key for S3.
- `P_S3_SECRET_KEY`: The secret key for S3.
- `P_S3_REGION`: The region for S3 (for MinIO, this is usually `us-east-1`).

## License

This project is licensed under the MIT License.
