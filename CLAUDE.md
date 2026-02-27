# Claude Code Instructions for IBM 1130 Emulator

## Code Quality Standards

**Zero tolerance for tech debt. No shortcuts. No laziness.**

Before every commit:
1. Run `cargo clippy --all-targets` and fix ALL warnings
2. Run `cargo fmt` to format code
3. Run `cargo test` to verify tests pass

Do not:
- Ignore clippy warnings (they are not "just suggestions")
- Declare victory prematurely
- Leave code in a broken state
- Skip verification steps

## Local Development Server

**ALWAYS use port 9352 for local previews. Do not use random ports.**

```bash
# Build and serve locally
./build-all.sh
./serve.sh
```

The serve.sh script uses `basic-http-server` on port 9352. Do not change this.

To preview changes:
1. Run `./build-all.sh` to rebuild
2. Run `./serve.sh` to start the server
3. Navigate to http://localhost:9352/ibm-1130-rs/
4. Use Playwright to take screenshots if needed

The serve.sh script creates a symlink `ibm-1130-rs -> pages` to match
the GitHub Pages URL structure (public_url="/ibm-1130-rs/" in Trunk.toml).

## Build Commands

- `./build-all.sh` - Build release to `./pages/` directory
- `./serve.sh` - Serve `./pages/` on http://localhost:9352/
- `cargo build --package components` - Quick compile check
- `trunk serve` - Development server with hot reload (uses different port)

## Project Structure

- `components/src/components/` - Yew UI components (console_panel.rs, indicator_lights.rs, etc.)
- `static/styles/` - CSS files for components
- `src/` - Main emulator application
- `pages/` - Production build output (committed for GitHub Pages deployment)
- `docs/` - Documentation files

## Key Files

- `components/src/components/console_panel.rs` - IBM 1130 console panel layout
- `components/src/components/indicator_lights.rs` - Register and control indicator displays
- `static/styles/console_panel.css` - Console panel styling
- `static/styles/indicator_lights.css` - Indicator light styling

## Deployment

Changes to `./pages/` on main branch are automatically deployed via GitHub Actions.

```bash
./build-all.sh
git add pages
git commit -m "Build for deploy"
git push
```

## Reference Documentation

- `docs/console-lights-schematic.txt` - Gemini documentation on IBM 1130 console lights
- `docs/console-lights-research.txt` - ChatGPT research on console layout
- Screenshot references in `~/Desktop/Screenshot*.png` for visual accuracy
