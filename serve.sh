#!/bin/bash
# Serve the IBM 1130 Emulator locally for preview
# Always uses port 9352 - do not change

set -e

cd "$(dirname "$0")"

if [ ! -d "pages" ]; then
    echo "Error: pages directory not found. Run ./build-all.sh first."
    exit 1
fi

echo "Serving IBM 1130 Emulator at http://localhost:9352/"
echo "Press Ctrl+C to stop"

cd pages
exec basic-http-server -a 0.0.0.0:9352
