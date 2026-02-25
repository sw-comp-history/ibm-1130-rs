# Design Decisions

## Architecture Decisions

### Decision 1: Rust + WebAssembly

**Choice**: Use Rust compiled to WebAssembly instead of JavaScript

**Rationale**:
- Type safety prevents runtime errors in CPU emulation
- Performance-critical instruction execution benefits from compiled code
- Rust's enum system perfectly models instruction sets
- Easy to test CPU logic independently of UI

**Trade-offs**:
- Longer compile times
- Larger initial bundle size
- More complex build toolchain

### Decision 2: Yew Framework

**Choice**: Use Yew for the UI instead of raw web-sys or JavaScript framework

**Rationale**:
- Component-based architecture matches UI structure
- Reactive state management simplifies updates
- Rust end-to-end reduces context switching
- Strong typing for component props

**Trade-offs**:
- Smaller ecosystem than React/Vue
- Steeper learning curve
- Slower development iteration

### Decision 3: Memory-Mapped Index Registers

**Choice**: Implement XR1-XR3 as actual memory locations (1, 2, 3)

**Rationale**:
- Historically accurate to real IBM 1130
- Enables visualization of register changes in memory viewer
- Simplifies implementation (no separate register storage)

**Trade-offs**:
- Users must understand memory-mapping concept
- Potential confusion between registers and memory

### Decision 4: Word Addressing

**Choice**: Use word addresses (not byte addresses) throughout

**Rationale**:
- IBM 1130 was word-addressed machine
- Simplifies memory viewer display
- More authentic experience

**Trade-offs**:
- Different from modern byte-addressed systems
- May confuse users familiar with x86/ARM

## UI Design Decisions

### Decision 5: Single-Page Layout

**Choice**: All controls visible on one screen without scrolling

**Rationale**:
- See code, registers, and memory simultaneously
- No context loss during stepping
- Immediate feedback on all state changes

**Trade-offs**:
- Dense interface
- Challenging on smaller screens

### Decision 6: Modal-Based Documentation

**Choice**: Use modals for tutorial, examples, challenges instead of separate pages

**Rationale**:
- No navigation away from editor
- Quick access and dismissal
- Consistent with single-page design

**Trade-offs**:
- Limited space for content
- Can feel cramped for long documentation

### Decision 7: Change Highlighting

**Choice**: Highlight registers/memory that changed on last instruction

**Rationale**:
- Visual feedback on instruction effects
- Aids debugging
- Educational value in seeing data flow

**Trade-offs**:
- Additional state tracking
- Could be visually distracting

### Decision 8: Special Location Coloring

**Choice**: Color-code memory locations 0-13 (trap, index regs, interrupts)

**Rationale**:
- Teaches architecture through visualization
- Prevents accidental writes to system areas
- Matches historical documentation conventions

**Trade-offs**:
- Color choice may not be accessible
- Adds visual complexity

## Assembler Design

### Decision 9: Simplified Syntax

**Choice**: Use simple "MNEMONIC mode address" format

**Rationale**:
- Easy to learn
- Unambiguous parsing
- Similar to original IBM 1130 assembler

**Trade-offs**:
- Less flexible than modern assemblers
- No macros or complex expressions

### Decision 10: DATA Directive

**Choice**: Support DATA directive for memory initialization

**Rationale**:
- Essential for practical programs
- Avoids need for self-modifying code
- Common in educational assemblers

**Trade-offs**:
- Not pure IBM 1130 assembler syntax
- Requires pre-processing before assembly

## Challenge System Design

### Decision 11: State-Based Validation

**Choice**: Validate solutions by checking CPU state after execution

**Rationale**:
- Allows multiple correct solutions
- Tests understanding, not specific syntax
- Easy to implement and extend

**Trade-offs**:
- Cannot verify code structure
- May miss inefficient solutions

### Decision 12: Progressive Difficulty

**Choice**: Three difficulty levels with hints available

**Rationale**:
- Scaffolded learning experience
- Prevents frustration
- Encourages experimentation

**Trade-offs**:
- Limited challenge count per level
- Hints may reduce learning

## Technology Choices

| Component | Choice | Alternative Considered |
|-----------|--------|----------------------|
| Language | Rust | TypeScript, AssemblyScript |
| UI Framework | Yew | Leptos, Dioxus, vanilla |
| Build Tool | Trunk | wasm-pack + manual |
| Hosting | GitHub Pages | Vercel, Netlify |
| CSS | Custom | Tailwind, CSS frameworks |
