# Architecture

## Overview

The IBM 1130 Emulator is a browser-based educational application built with Rust compiled to WebAssembly. It uses the Yew framework for the reactive UI and implements a historically accurate simulation of the IBM 1130 minicomputer.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Browser                               │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │                  Yew Application                     │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │    │
│  │  │   Sidebar   │  │  Program    │  │   Memory    │  │    │
│  │  │  Component  │  │    Area     │  │   Viewer    │  │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │    │
│  │  │  Register   │  │   Modal     │  │   Header    │  │    │
│  │  │   Panel     │  │  Component  │  │  Component  │  │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │    │
│  └─────────────────────────────────────────────────────┘    │
│                              │                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   Core Library                       │    │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │    │
│  │  │  Assembler  │  │     CPU     │  │  Challenge  │  │    │
│  │  │   Parser    │  │   State     │  │   System    │  │    │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  │    │
│  │                   ┌─────────────┐                    │    │
│  │                   │  Executor   │                    │    │
│  │                   │  (ISA Impl) │                    │    │
│  │                   └─────────────┘                    │    │
│  └─────────────────────────────────────────────────────┘    │
│                              │                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 WebAssembly Runtime                  │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Component Structure

### UI Layer (`components/`)

Reusable Yew components providing consistent UI across assembly game variants:

| Component | Purpose |
|-----------|---------|
| `Header` | Application title and subtitle |
| `Sidebar` | Navigation buttons for modals |
| `ProgramArea` | Code editor with assemble/step/run controls |
| `RegisterPanel` | Display CPU registers with change highlighting |
| `MemoryViewer` | Scrollable memory display with special location highlighting |
| `Modal` | Overlay dialogs for tutorials, examples, challenges |

### Core Library (`src/`)

| Module | Purpose |
|--------|---------|
| `cpu/state.rs` | CPU state: registers, memory, flags |
| `cpu/instruction.rs` | Instruction definitions and addressing modes |
| `cpu/executor.rs` | Instruction execution logic |
| `assembler.rs` | Assembly language parser with ORG support |
| `challenge.rs` | Challenge definitions and validation |
| `wasm.rs` | WASM bindings for browser integration |
| `app.rs` | Main Yew application component |

## Data Flow

```
User Input → Editor → Assembler → CPU State → UI Update
                         ↓
                    Machine Code
                         ↓
     Step/Run → Executor → CPU State Update → UI Refresh
```

## Memory Model

The IBM 1130 uses a **word-addressed** architecture:

- **Memory Size**: 4,096 words (16-bit each)
- **Address Space**: 0x0000 - 0x0FFF

### Special Memory Locations

| Address | Purpose |
|---------|---------|
| 0x0000 | Safety trap (infinite loop) |
| 0x0001 | Index Register 1 (XR1) |
| 0x0002 | Index Register 2 (XR2) |
| 0x0003 | Index Register 3 (XR3) |
| 0x0008-0x000D | Interrupt vectors |
| 0x0010+ | Program area |

## Build Pipeline

```
Rust Source → wasm-pack → WASM Binary → Trunk → Distribution
                              ↓
                        JavaScript Glue
                              ↓
                         index.html
```

## Deployment

- **Source branch**: `main`
- **Deployment branch**: `gh-pages`
- **Build tool**: Trunk
- **Hosting**: GitHub Pages
- **CI/CD**: GitHub Actions (deploy on push to gh-pages)
