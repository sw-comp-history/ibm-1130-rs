# IBM 1130 Assembly Emulator

An interactive browser-based educational game for learning the IBM 1130 instruction set architecture. The IBM 1130 was a 16-bit minicomputer introduced by IBM in 1965, widely used in scientific and educational institutions.

## Live Demo

**[Try it online](https://softwarewrighter.github.io/ibm-1130-rs/)**

## Features

- **Historical IBM 1130 CPU emulation** - 4K words of 16-bit memory
- **Memory-mapped index registers** - XR1, XR2, XR3 at addresses 1, 2, 3
- **Complete instruction set** - LD, STO, LDX, STX, A, S, AND, OR, SLA, SRA, BSC, BSI, WAIT
- **Interactive examples** covering arithmetic, indexing, shifts, and memory-mapped registers
- **Progressive challenges** with validation
- **Assembly parser** with ORG directive support
- **Real-time visualization** of CPU state, registers, and memory

## Architecture

The IBM 1130 is a word-addressed machine with:

- **ACC** - Accumulator: main register for arithmetic
- **EXT** - Extension register: for double-precision operations
- **IAR** - Instruction Address Register: program counter
- **XR1-XR3** - Index Registers: for indexed addressing (memory-mapped at locations 1, 2, 3)
- **Flags** - C (Carry), V (Overflow), P (Positive), Z (Zero)

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (with wasm32-unknown-unknown target)
- [Trunk](https://trunkrs.dev/) - `cargo install trunk`

### Development

```bash
# Run development server with hot reload
trunk serve

# Build for production
trunk build --release
```

The production build outputs to `./pages/`.

### Deploying to GitHub Pages

1. Build locally:
   ```bash
   trunk build --release
   ```

2. Create and push gh-pages branch:
   ```bash
   git checkout --orphan gh-pages
   cp -r pages/* .
   git add .
   git commit -m "Deploy"
   git push -u origin gh-pages
   ```

3. Enable GitHub Pages in repository settings, selecting the gh-pages branch.

## Project Structure

```
ibm-1130-rs/
├── src/                    # Main application
│   ├── app.rs             # Yew application component
│   ├── assembler.rs       # Assembly parser
│   ├── challenge.rs       # Challenge system
│   ├── cpu/               # CPU emulation
│   │   ├── executor.rs    # Instruction execution
│   │   ├── instruction.rs # Instruction definitions
│   │   └── state.rs       # CPU state management
│   ├── lib.rs             # Library root
│   └── wasm.rs            # WASM bindings
├── components/            # Shared Yew UI components
│   └── src/
│       ├── components/    # UI components (header, sidebar, etc.)
│       └── lib.rs
├── styles/                # CSS stylesheets
├── index.html             # HTML entry point
├── Trunk.toml             # Trunk configuration
└── Cargo.toml             # Workspace configuration
```

## License

MIT
