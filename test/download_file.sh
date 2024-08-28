
#!/bin/bash

# Replace these with appropriate values
TENANT="mayorana"
FILENAME="toto.txt"
OUTPUT_FILE="downloaded_file.txt"

# Clear the output file if it already exists
> "$OUTPUT_FILE"

# Run grpcurl to call the downloadFile RPC and process the raw output
grpcurl -plaintext \
    -rpc-header "tenant:$TENANT" \
    -rpc-header "filename:$FILENAME" \
    0.0.0.0:50051 minioc.MiniocService/downloadFile \
| while IFS= read -r chunk; do
    # Append each chunk to the output file
    echo -n "$chunk" >> "$OUTPUT_FILE"
done

echo "Download complete. File saved as $OUTPUT_FILE"

