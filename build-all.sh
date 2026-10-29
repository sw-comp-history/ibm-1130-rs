#!/bin/bash
# Build for GitHub Pages (pages folder on main branch)
set -e

# Fresh timestamp makes build.rs re-run, so the footer's build host, git SHA
# and timestamp are updated on every deploy build
export IBM1130_BUILD_TIMESTAMP="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

# Build release
echo "Building release to ./pages..."
trunk build --release

echo "Build complete. ./pages has been updated."
echo "Run 'git add pages && git commit' to commit the changes."
