# Console with Emulator Plan

The Console tab is a front panel for the IBM 1130. The long-term goal is to
connect it to a full emulator so programs can be entered, run and debugged
from the panel, as an operator would on the real machine. This document
describes what the panel can demo today and the steps to get there.

## What can be demoed now

The console is connected to the same CPU memory the Assembler Game uses
(`WasmCpu` in `src/app.rs`). Words stored from the console appear in the
Assembler Game's **Memory (4K Words)** view, highlighted as changed.

### Storing words in memory

1. Click **POWER** to turn the system on.
2. Turn the mode switch (the knob) to **LOAD**.
3. Set the starting address on the 16 console entry switches and press
   **LOAD IAR**. The INSTRUCTION ADDRESS lights show the address.
4. For each word:
   - Set the word on the entry switches.
   - Press **PROG START**. The word is stored at the IAR address, then IAR
     advances to the next word. STORAGE ADDRESS shows where the word went,
     and STORAGE BUFFER shows the word stored.
5. Open the Assembler Game tab to see the words in the memory view.

### Reading words back

1. Set the starting address on the entry switches and press **LOAD IAR**.
2. Turn the mode switch to **DISP**.
3. Press **PROG START** repeatedly. Each press shows the word at IAR in
   STORAGE BUFFER (its address in STORAGE ADDRESS) and advances IAR.

### Other controls

- **RESET** clears the CPU registers. Words in memory are kept.
- Hovering over any lamp, button or switch shows a description. Lamps show
  it quickly; buttons and switches wait longer, so the tooltip does not get
  in the way while operating them.

### Limits of the demo

- **No program execution from the panel.** PROG START in RUN mode only
  lights the RUN lamp. IMM STOP and PROG STOP only turn it off. Programs
  stored from the console can be run from the Assembler Game's Step/Run
  buttons, but that CPU uses the game's simplified instruction encoding, not
  real 1130 opcodes.
- **SS, SMC, INT RUN and SI** modes do nothing yet.
- **PROG LOAD** (boot from a card) is not emulated.
- **4K words of memory.** Addresses wrap at 4096, as on a minimum 1130, so
  LOAD IAR with 0x1234 sets IAR to 0x0234.
- **LOAD IAR works in any mode.** Check against the 1130 operating
  procedures whether it should require LOAD mode.
- **ARITHMETIC FACTOR** and the right-hand OP/TAG/CYC/STS lamps are not
  driven by the CPU. SAR and SBR show the last console LOAD/DISP operation,
  not CPU memory cycles.
- **The status lamps** (DISK UNLOCK, FILE READY, PARITY CHECK, K.B. SELECT,
  FORMS CHECK) and the CONSOLE/KEYBOARD switch are cosmetic. The switch
  flips between CON (up) and INT (down), but nothing reads it yet.

## Current design

| Piece | Location | Role |
|---|---|---|
| `ConsolePanel` | `components/src/components/console_panel.rs` | Panel UI. Holds switches, mode, power and lamp test. Displays `external_registers` when provided. |
| Panel callbacks | `on_load`, `on_deposit`, `on_examine`, `on_reset` props | Report operator actions. PROG START in LOAD mode sends `on_deposit(addr, word)`; in DISP mode, `on_examine(addr)`. |
| App wiring | `console_content_html` in `src/app.rs` | Applies the actions to the shared `WasmCpu`. Keeps SAR/SBR (`console_storage`) for the lamps. |
| CPU | `src/cpu/` | Simplified 16-bit CPU used by the Assembler Game. |

The panel reports intents and the app owns the machine state. That split
should stay when the full emulator replaces the simplified CPU.

## Plan for connecting a full emulator

### 1. Authentic CPU core

Implement the real 1130 instruction set in its own crate (for example
`crates/ibm1130-cpu`), independent of Yew and WASM:

- Short (one-word) and long (two-word) instruction formats, tag bits for
  XR1-XR3 (memory locations 1-3), and indirect addressing.
- The full opcode set (LD, LDD, STO, STD, A, AD, S, SD, M, D, AND, OR, EOR,
  shifts, BSC/BOSC, BSI, MDX, LDX/STX, LDS/STS, WAIT, XIO).
- Carry and overflow indicators, WAIT state, and memory sizes of 4K-32K
  words.
- Expose the panel registers: IAR, SAR, SBR, AFR, ACC, EXT, plus OP code,
  format, tag, indirect and the current cycle (T0-T7) for the right-hand
  lamps.

Validate against known programs and, where possible, traces from the SIMH
IBM 1130 simulator.

### 2. One machine shared by all tabs

Replace the cloned `WasmCpu` state with a single machine object (CPU, memory
and devices) shared through a Yew context, for example
`Rc<RefCell<Machine>>` plus a version counter that triggers re-renders.
The Console, Assembler Game, Keypunch and Printer tabs would all operate on
the same machine.

### 3. Execution loop and console modes

- Run the CPU in bounded batches per animation frame
  (`requestAnimationFrame` or a `gloo` interval) so the UI stays responsive.
  Update the lamps from a snapshot after each batch.
- **RUN / INT RUN**: PROG START runs from IAR until WAIT, PROG STOP (end of
  instruction) or IMM STOP (immediately).
- **SI**: each PROG START executes one instruction.
- **SMC / SS**: step one memory cycle or one clock step. This needs a
  cycle-level CPU model; start with SI and add these later.
- **DISP / LOAD**: keep today's behavior, driven by the real CPU.

### 4. Devices

- **Console keyboard and 1053 console printer** through XIO, connected to
  the on-screen keyboard and the Printer tab. The CONSOLE/KEYBOARD switch
  selects whether a keyboard read returns the entry switches.
- **1442 card reader** reading decks from the Keypunch tab. PROG LOAD reads
  the first card into memory and starts it (cold start).
- **2310 disk** later, for DMS and FORTRAN.
- Drive the status lamps from device state (FILE READY, FORMS CHECK,
  K.B. SELECT, PARITY CHECK).

### 5. Interrupts

Six interrupt levels (0-5) with the vectors at locations 8-13, the
interrupt level lamps, and INT RUN mode.

### 6. Testing

- Unit tests for each panel operation against the CPU (LOAD IAR,
  LOAD-mode store, DISP, RESET).
- Golden tests that enter a program from the panel, run it and check
  memory and registers.
- Browser checks of the panel lamps for a few known register values.
