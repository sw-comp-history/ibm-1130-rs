# IBM 1130 System Emulator

An interactive browser-based educational platform for learning the IBM 1130 computer system. The IBM 1130 was a 16-bit minicomputer introduced by IBM in 1965, widely used in scientific and educational institutions.

## Live Demo

**[Try it online](https://sw-comp-history.github.io/ibm-1130-rs/)**

## Screenshots

### Assembler Game
Write and execute IBM 1130 assembly code with real-time register and memory visualization.

![Assembler Game](images/assembler.png)

### Console Panel
Authentic IBM 1130 console with indicator lights, toggle switches, and speed control knob.

![Console Panel](images/console.png)

### IBM 029 Keypunch
Punch cards with Hollerith encoding - type text and see the punch pattern.

![Keypunch](images/keypunch.png)

### Console Printer
IBM 1131 console printer with typewriter-style output and greenbar paper.

![Printer](images/printer.png)

## Features

### Assembler Game
- **Historical IBM 1130 CPU emulation** - 4K words of 16-bit memory
- **Memory-mapped index registers** - XR1, XR2, XR3 at addresses 1, 2, 3
- **Complete instruction set** - LD, STO, LDX, STX, A, S, AND, OR, SLA, SRA, BSC, BSI, WAIT
- **Interactive examples** covering arithmetic, indexing, shifts, and memory-mapped registers
- **Progressive challenges** with validation
- **Assembly parser** with ORG directive support
- **Real-time visualization** of CPU state, registers, and memory

### Console Panel
- **6 rows × 16 register indicator lights** - IAR, SAR, SBR, AFR, ACC, EXT
- **8 status indicator lights** - DISK UNLOCK, FILE READY, RUN, PARITY CHECK, etc.
- **16-bit toggle switches** for data entry
- **7-position speed control knob** - SS, SMC, INT RUN, RUN, SI, DISP, LOAD
- **Power switch and control buttons**
- **Lamp test** for verifying all indicators

### Keypunch
- **IBM 029 Keypunch simulation**
- **Hollerith punch card encoding**
- **Visual punch pattern display**
- **Multi-card deck management**

### Printer
- **IBM 1131 console printer simulation**
- **Greenbar paper rendering**
- **Typewriter-style character output**

## Documentation

- [Architecture](docs/architecture.md) - System design and component structure
- [PRD](docs/prd.md) - Product requirements and user stories
- [Plan](docs/plan.md) - Implementation roadmap and milestones
- [Design](docs/design.md) - Design decisions and rationale
- [Status](docs/status.md) - Current project status and changelog

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
# Build for production
./build-all.sh

# Serve locally for preview (always uses port 9352)
./serve.sh
# Then open http://localhost:9352/ibm-1130-rs/

# Run development server with hot reload (for active development)
trunk serve
```

The production build outputs to `./pages/`.

### Deploying to GitHub Pages

Build and commit:
```bash
./build-all.sh
git add pages
git commit -m "Build for deploy"
git push
```

GitHub Actions automatically deploys from `./pages` when changes are pushed to main.

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
├── docs/                  # Documentation
├── images/                # Screenshots
├── index.html             # HTML entry point
├── Trunk.toml             # Trunk configuration
└── Cargo.toml             # Workspace configuration
```

## References

### IBM Documentation

- **[IBM 1130 Functional Characteristics (A26-5881-2)](https://bitsavers.org/pdf/ibm/1130/functional_characteristics/A26-5881-2_1130_Functional_Characteristics_1966.pdf)** - The definitive technical reference for the IBM 1130 architecture, instruction set, and hardware specifications (1966)
- **[IBM 1130 Assembler Language (C26-5927-2)](https://bitsavers.org/pdf/ibm/1130/lang/C26-5927-2_1130_Assembler_Language_1966.pdf)** - Complete assembler language reference including directives, macros, and programming techniques (1966)
- **[IBM 1130 Manual Library](http://ibm1130.org/lib/manuals/)** - Comprehensive collection of IBM 1130 documentation at ibm1130.org

### Online Resources

- **[IBM 1130 - Wikipedia](https://en.wikipedia.org/wiki/IBM_1130)** - Overview of the IBM 1130's history and significance
- **[IBM 1130 Functional Characteristics (HTML)](https://ibm1130.net/functional/Introduction.html)** - Hyperlinked HTML version of the Functional Characteristics manual
- **[Bitsavers IBM 1130 Archive](https://bitsavers.org/pdf/ibm/1130/)** - Complete archive of IBM 1130 documentation PDFs

### Historical Context

The IBM 1130 was introduced in 1965 as an affordable scientific computer. It featured:
- 16-bit word size with 15-bit addressing (32K words max)
- Memory-mapped index registers at locations 1, 2, 3
- Single-address instruction format with indirect and indexed addressing modes
- Popular in universities and small businesses throughout the late 1960s and 1970s

## License

MIT
