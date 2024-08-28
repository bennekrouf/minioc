
#!/bin/bash

# Replace these with appropriate values
TENANT="mayorana"
FILENAME="test_file3.txt"
OUTPUT_FILE="downloaded_file.txt"

# Run grpcurl to call the downloadFile RPC
grpcurl -plaintext \
    -rpc-header "tenant:$TENANT" \
    -rpc-header "filename:$FILENAME" \
    0.0.0.0:50051 minioc.MiniocService/downloadFile \
    | while IFS= read -r line; do
        # Extract the base64-encoded data from the JSON response
        echo "$line" | jq -r '.data' | base64 -d
    done > "$OUTPUT_FILE"
