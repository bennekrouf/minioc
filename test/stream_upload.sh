#!/bin/bash

# Replace these with appropriate values
FILE="toto.txt"
TENANT="mayorana1"
FILENAME="toto.txt"

# Stream file chunks to grpcurl
while IFS= read -r -n 1024 chunk; do
    echo "{\"data\": \"$(echo -n "$chunk" | base64)\"}"
done < "$FILE" | grpcurl -plaintext \
    -rpc-header "tenant:$TENANT" \
    -rpc-header "filename:$FILENAME" \
    -d @ \
    0.0.0.0:50051 minioc.MiniocService/streamUpload

