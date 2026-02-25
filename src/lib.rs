//! IBM 1130 Educational Game
//!
//! An interactive browser-based game for learning the IBM 1130 instruction set architecture.
//! This educational tool simulates the IBM 1130 minicomputer from 1965, allowing users
//! to write and execute assembly programs through a web interface.
//!
//! ## Architecture
//!
//! The IBM 1130 is a 16-bit word-addressed machine with:
//! - Accumulator (ACC) - main register for arithmetic
//! - Extension register (EXT) - used for double-precision operations
//! - Instruction Address Register (IAR) - program counter
//! - Index registers (XR1-XR3) - memory-mapped at addresses 1, 2, 3
//! - 256 words of memory (16-bit words)
//!
//! ## Instruction Set
//!
//! The simplified instruction set includes:
//! - **Load/Store**: LD, STO, LDX, STX
//! - **Arithmetic**: A (add), S (subtract)
//! - **Logical**: AND, OR
//! - **Shift**: SLA (shift left), SRA (shift right arithmetic)
//! - **Branch**: BSC (branch on condition), BSI (branch and store IAR)
//! - **Control**: WAIT (halt), NOP
//!
//! ## Usage
//!
//! This crate is designed to be compiled to WebAssembly and used in a browser:
//!
//! ```bash
//! trunk build --release
//! ```

pub mod assembler;
pub mod challenge;
pub mod cpu;

#[cfg(target_arch = "wasm32")]
pub mod app;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use assembler::{Assembler, AssemblerError, decode_instruction, encode_instruction};
pub use challenge::{
    Challenge, Difficulty, TestCase, TestResult, ValidationResult, get_all_challenges,
};
pub use cpu::{AddressingMode, BranchCondition, CpuError, CpuState, Instruction};
pub use cpu::{MEMORY_SIZE, PROGRAM_START, XR1_ADDR};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
