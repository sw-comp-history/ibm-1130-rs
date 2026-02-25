//! IBM 1130 CPU implementation
//!
//! This module contains the complete CPU emulation for the IBM 1130 minicomputer.

pub mod executor;
pub mod instruction;
pub mod state;

pub use instruction::{AddressingMode, BranchCondition, Instruction};
pub use state::{CpuError, CpuState, MEMORY_SIZE, PROGRAM_START, XR1_ADDR};
