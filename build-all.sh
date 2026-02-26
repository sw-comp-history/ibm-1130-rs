#!/bin/bash
# Build for GitHub Pages (pages folder on main branch)
set -e

# Build release
echo "Building release to ./pages..."
trunk build --release

echo "Build complete. ./pages has been updated."
echo "Run 'git add pages && git commit' to commit the changes."
