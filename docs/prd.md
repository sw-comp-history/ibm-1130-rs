# Product Requirements Document (PRD)

## Product Overview

**Product Name**: IBM 1130 Assembly Emulator
**Version**: 0.1.0
**Target Audience**: Students, educators, computer history enthusiasts, retro computing hobbyists

## Problem Statement

Learning assembly language and computer architecture concepts is challenging because:
1. Access to historical hardware is limited
2. Traditional emulators are complex to set up
3. Educational materials lack interactivity
4. Modern development environments don't teach fundamental concepts

## Solution

A browser-based, interactive educational tool that simulates the IBM 1130 minicomputer, allowing users to write, assemble, and execute programs while observing CPU state changes in real-time.

## Goals

1. **Accessibility**: Zero-install, runs in any modern browser
2. **Education**: Progressive learning through examples and challenges
3. **Historical Accuracy**: Faithful representation of IBM 1130 architecture
4. **Engagement**: Interactive visualization of CPU operations

## User Stories

### As a Student
- I want to write assembly programs and see them execute step-by-step
- I want to understand how memory addressing works
- I want challenges to test my understanding

### As an Educator
- I want a tool to demonstrate CPU concepts visually
- I want examples that illustrate key architectural features
- I want challenges I can assign to students

### As a Hobbyist
- I want to experience historical computing
- I want to understand how 1960s computers worked
- I want accurate ISA implementation

## Features

### Core Features (MVP)

| Feature | Priority | Status |
|---------|----------|--------|
| Assembly editor | P0 | ✅ Complete |
| Assembler with ORG support | P0 | ✅ Complete |
| CPU execution (step/run) | P0 | ✅ Complete |
| Register display with change highlighting | P0 | ✅ Complete |
| Memory viewer with special location highlighting | P0 | ✅ Complete |
| Full IBM 1130 instruction set | P0 | ✅ Complete |
| 5 educational examples | P1 | ✅ Complete |
| 3 progressive challenges | P1 | ✅ Complete |
| Tutorial modal | P1 | ✅ Complete |
| ISA reference modal | P1 | ✅ Complete |

### Future Features

| Feature | Priority | Status |
|---------|----------|--------|
| More challenges (10+) | P2 | Planned |
| Code save/load to local storage | P2 | Planned |
| Share programs via URL | P3 | Planned |
| Breakpoints | P3 | Planned |
| Memory watches | P3 | Planned |
| Achievement system | P3 | Planned |

### Future Peripherals

| Peripheral | Model | Priority | Status |
|------------|-------|----------|--------|
| Internal Disk Drive | 2310/2311 | P2 | Planned |
| Card Reader/Punch | 1442 | P2 | Planned |
| Line Printer | 1132 | P2 | Planned |
| Multiplexor | 1133 | P3 | Planned |
| Drum Plotter | 1627 | P3 | Planned |
| Vector Graphics Display | 2250 | P3 | Planned |
| Tape Drives | TBD | P3 | Planned |

### Programming Language Support

| Language | Priority | Status |
|----------|----------|--------|
| Assembler | P0 | ✅ Complete |
| FORTH | P2 | Planned |
| APL | P3 | Planned |

## "Day in the Life" Educational Scenarios

The emulator aims to recreate authentic 1130 programming workflows:

### Scenario 1: Assembler Programmer
1. **Keypunch** source code onto cards
2. **Load** card deck into card reader
3. **Assemble** source program
4. **Save** object deck to disk or punch cards
5. **Execute** program with console interaction
6. **Print** results on line printer

### Scenario 2: FORTH Programmer
1. **Load** FORTH system from card deck or disk
2. **Boot** into FORTH interpreter
3. **Define** words interactively via console
4. **Save** dictionary to disk
5. **Load/execute** programs from cards

### Scenario 3: APL Programmer
1. **Load** APL interpreter from disk
2. **Type** APL expressions using console keyboard
3. **Save** workspaces to disk
4. **Load** workspaces from disk
5. **Print** output on console printer or line printer

### Data Flow Capabilities
- **Cards → Memory**: Load programs/data from 1442
- **Memory → Cards**: Punch output decks
- **Cards → Disk**: Copy card images to disk files
- **Disk → Cards**: Reproduce card decks from disk
- **Memory → Tape**: Archive data to tape drives
- **Tape → Memory**: Restore data from tape

### Special Scenario: IBM 360/370 Graphics Terminal
The IBM 1130 was used as a "smart terminal controller" for IBM 360/370 mainframes, driving the 2250 vector graphics display. This configuration allowed:
- Mainframe applications to render vector graphics
- 1130 handling display list management and refresh
- Interactive graphics applications (CAD, scientific visualization)
- Channel-to-channel communication between 1130 and 360/370

This advanced scenario demonstrates the 1130's role in early computer graphics history.

## Success Metrics

1. **Engagement**: Average session duration > 5 minutes
2. **Completion**: > 50% of users complete at least one challenge
3. **Adoption**: Used by at least 3 educational institutions

## Technical Requirements

- Load time < 3 seconds on broadband
- Works on Chrome, Firefox, Safari, Edge
- Mobile-responsive (tablet minimum)
- WASM binary < 1MB

## Non-Functional Requirements

1. **Performance**: 60fps UI updates during execution
2. **Reliability**: No crashes during normal operation
3. **Accessibility**: Keyboard navigation support
4. **Documentation**: Comprehensive in-app help
