#!/bin/bash
# Serve the IBM 1130 Emulator locally for preview
# Always uses port 9352 - do not change

set -e

cd "$(dirname "$0")"

if [ ! -d "pages" ]; then
    echo "Error: pages directory not found. Run ./build-all.sh first."
    exit 1
fi

# Create symlink to match GitHub Pages URL structure
# The build uses public_url="/ibm-1130-rs/" so we need that path locally too
if [ ! -L "ibm-1130-rs" ]; then
    ln -s pages ibm-1130-rs
fi

echo "Serving IBM 1130 Emulator at http://localhost:9352/ibm-1130-rs/"
echo "Press Ctrl+C to stop"

exec basic-http-server -a 0.0.0.0:9352
