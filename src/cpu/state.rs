//! IBM 1130 CPU state management
//!
//! This module implements the CPU state including registers, memory, and flags.

use thiserror::Error;

/// Number of 16-bit words in memory (4K words = 4096)
/// IBM 1130 could have up to 32K words, but we use 4K for this emulator
pub const MEMORY_SIZE: usize = 4096;

/// Reserved memory locations
pub const XR1_ADDR: u16 = 0x0001; // Index Register 1
pub const XR2_ADDR: u16 = 0x0002; // Index Register 2
pub const XR3_ADDR: u16 = 0x0003; // Index Register 3

/// Program start address (after reserved locations)
/// Address 0x0000-0x0003 are reserved for system use (infinite loop trap and index registers)
pub const PROGRAM_START: u16 = 0x0010;

/// CPU execution errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CpuError {
    #[error("Memory access out of bounds: 0x{0:04X}")]
    MemoryOutOfBounds(u16),

    #[error("CPU is halted")]
    Halted,

    #[error("Invalid instruction at 0x{0:04X}")]
    InvalidInstruction(u16),

    #[error("Program counter out of bounds: 0x{0:04X}")]
    IarOutOfBounds(u16),
}

/// IBM 1130 CPU state
#[derive(Debug, Clone)]
pub struct CpuState {
    /// Accumulator (16-bit)
    acc: u16,

    /// Extension register (16-bit) - used for double-precision operations
    ext: u16,

    /// Instruction Address Register (program counter)
    iar: u16,

    /// Carry flag
    carry: bool,

    /// Overflow flag
    overflow: bool,

    /// Memory (4096 16-bit words = 4K)
    /// Note: Index registers XR1-XR3 are stored at memory[1], memory[2], memory[3]
    memory: [u16; MEMORY_SIZE],

    /// Execution state
    halted: bool,

    /// Cycle counter
    cycle_count: u64,

    /// Instruction counter
    instruction_count: u64,
}

impl Default for CpuState {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuState {
    /// Create a new CPU with default state
    pub fn new() -> Self {
        Self {
            acc: 0,
            ext: 0,
            iar: PROGRAM_START,
            carry: false,
            overflow: false,
            memory: [0; MEMORY_SIZE],
            halted: false,
            cycle_count: 0,
            instruction_count: 0,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        self.acc = 0;
        self.ext = 0;
        self.iar = PROGRAM_START;
        self.carry = false;
        self.overflow = false;
        self.halted = false;
        self.cycle_count = 0;
        self.instruction_count = 0;
        // Note: Memory is NOT cleared on reset (program stays loaded)
    }

    /// Reset and clear all memory
    pub fn hard_reset(&mut self) {
        self.reset();
        self.memory = [0; MEMORY_SIZE];
    }

    // ===== Register Access =====

    /// Read accumulator
    pub fn read_acc(&self) -> u16 {
        self.acc
    }

    /// Write accumulator
    pub fn write_acc(&mut self, value: u16) {
        self.acc = value;
    }

    /// Read extension register
    pub fn read_ext(&self) -> u16 {
        self.ext
    }

    /// Write extension register
    pub fn write_ext(&mut self, value: u16) {
        self.ext = value;
    }

    /// Read instruction address register (program counter)
    pub fn iar(&self) -> u16 {
        self.iar
    }

    /// Set instruction address register
    pub fn set_iar(&mut self, addr: u16) -> Result<(), CpuError> {
        if addr as usize >= MEMORY_SIZE {
            return Err(CpuError::IarOutOfBounds(addr));
        }
        self.iar = addr;
        Ok(())
    }

    /// Increment IAR by one word
    pub fn increment_iar(&mut self) -> Result<(), CpuError> {
        self.set_iar(self.iar.wrapping_add(1))
    }

    /// Read index register 1 (from memory location 1)
    pub fn read_xr1(&self) -> u16 {
        self.memory[XR1_ADDR as usize]
    }

    /// Write index register 1 (to memory location 1)
    pub fn write_xr1(&mut self, value: u16) {
        self.memory[XR1_ADDR as usize] = value;
    }

    /// Read index register 2 (from memory location 2)
    pub fn read_xr2(&self) -> u16 {
        self.memory[XR2_ADDR as usize]
    }

    /// Write index register 2 (to memory location 2)
    pub fn write_xr2(&mut self, value: u16) {
        self.memory[XR2_ADDR as usize] = value;
    }

    /// Read index register 3 (from memory location 3)
    pub fn read_xr3(&self) -> u16 {
        self.memory[XR3_ADDR as usize]
    }

    /// Write index register 3 (to memory location 3)
    pub fn write_xr3(&mut self, value: u16) {
        self.memory[XR3_ADDR as usize] = value;
    }

    // ===== Memory Access =====

    /// Read a word from memory
    pub fn read_word(&self, addr: u16) -> Result<u16, CpuError> {
        if addr as usize >= MEMORY_SIZE {
            return Err(CpuError::MemoryOutOfBounds(addr));
        }
        Ok(self.memory[addr as usize])
    }

    /// Write a word to memory
    pub fn write_word(&mut self, addr: u16, value: u16) -> Result<(), CpuError> {
        if addr as usize >= MEMORY_SIZE {
            return Err(CpuError::MemoryOutOfBounds(addr));
        }
        self.memory[addr as usize] = value;
        Ok(())
    }

    /// Load program into memory starting at address
    pub fn load_program(&mut self, start_addr: u16, data: &[u16]) -> Result<(), CpuError> {
        if start_addr as usize + data.len() > MEMORY_SIZE {
            return Err(CpuError::MemoryOutOfBounds(start_addr));
        }

        let start = start_addr as usize;
        self.memory[start..start + data.len()].copy_from_slice(data);
        Ok(())
    }

    // ===== Flags =====

    /// Get carry flag
    pub fn carry(&self) -> bool {
        self.carry
    }

    /// Set carry flag
    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    /// Get overflow flag
    pub fn overflow(&self) -> bool {
        self.overflow
    }

    /// Set overflow flag
    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    /// Update flags for addition
    pub fn update_flags_add(&mut self, a: u16, b: u16, result: u16) {
        // Carry: unsigned overflow
        self.carry = result < a;

        // Overflow: signed overflow
        // Overflow occurs when adding two numbers of the same sign produces a result of opposite sign
        let a_sign = (a & 0x8000) != 0;
        let b_sign = (b & 0x8000) != 0;
        let r_sign = (result & 0x8000) != 0;
        self.overflow = (a_sign == b_sign) && (a_sign != r_sign);
    }

    /// Update flags for subtraction
    pub fn update_flags_sub(&mut self, a: u16, b: u16, result: u16) {
        // Carry: unsigned underflow (borrow)
        self.carry = a < b;

        // Overflow: signed overflow
        let a_sign = (a & 0x8000) != 0;
        let b_sign = (b & 0x8000) != 0;
        let r_sign = (result & 0x8000) != 0;
        self.overflow = (a_sign != b_sign) && (a_sign != r_sign);
    }

    // ===== Execution State =====

    /// Check if CPU is halted
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// Halt the CPU
    pub fn halt(&mut self) {
        self.halted = true;
    }

    /// Resume CPU execution
    pub fn resume(&mut self) {
        self.halted = false;
    }

    /// Get cycle count
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    /// Get instruction count
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// Increment cycle counter
    pub fn tick(&mut self) {
        self.cycle_count += 1;
    }

    /// Increment instruction counter
    pub fn count_instruction(&mut self) {
        self.instruction_count += 1;
    }

    // ===== Debugging =====

    /// Get a reference to memory (for debugging/display)
    pub fn memory(&self) -> &[u16; MEMORY_SIZE] {
        &self.memory
    }

    /// Get a slice of memory for a specific range
    pub fn memory_slice(&self, start: u16, len: usize) -> Result<&[u16], CpuError> {
        let start = start as usize;
        if start + len > MEMORY_SIZE {
            return Err(CpuError::MemoryOutOfBounds(start as u16));
        }
        Ok(&self.memory[start..start + len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_creation() {
        let cpu = CpuState::new();
        assert_eq!(cpu.iar(), PROGRAM_START);
        assert_eq!(cpu.read_acc(), 0);
        assert_eq!(cpu.read_ext(), 0);
        assert!(!cpu.is_halted());
        assert_eq!(cpu.cycle_count(), 0);
    }

    #[test]
    fn test_register_operations() {
        let mut cpu = CpuState::new();

        cpu.write_acc(0x1234);
        assert_eq!(cpu.read_acc(), 0x1234);

        cpu.write_ext(0xABCD);
        assert_eq!(cpu.read_ext(), 0xABCD);

        cpu.write_xr1(0x0010);
        assert_eq!(cpu.read_xr1(), 0x0010);
    }

    #[test]
    fn test_memory_operations() {
        let mut cpu = CpuState::new();

        cpu.write_word(0x50, 0x1234).unwrap();
        assert_eq!(cpu.read_word(0x50).unwrap(), 0x1234);

        // Out of bounds
        assert!(cpu.write_word(MEMORY_SIZE as u16, 0).is_err());
        assert!(cpu.read_word(MEMORY_SIZE as u16).is_err());
    }

    #[test]
    fn test_flags() {
        let mut cpu = CpuState::new();

        // Addition with overflow
        cpu.update_flags_add(0xFFFF, 1, 0);
        assert!(cpu.carry());

        // Signed overflow: MAX_POSITIVE + 1 = MIN_NEGATIVE
        cpu.update_flags_add(0x7FFF, 1, 0x8000);
        assert!(cpu.overflow());

        // Subtraction with underflow
        cpu.update_flags_sub(5, 10, 0xFFFB);
        assert!(cpu.carry());
    }

    #[test]
    fn test_iar_operations() {
        let mut cpu = CpuState::new();

        assert_eq!(cpu.iar(), PROGRAM_START);

        cpu.set_iar(0x50).unwrap();
        assert_eq!(cpu.iar(), 0x50);

        cpu.increment_iar().unwrap();
        assert_eq!(cpu.iar(), 0x51);

        // Out of bounds
        assert!(cpu.set_iar(MEMORY_SIZE as u16).is_err());
    }

    #[test]
    fn test_halt() {
        let mut cpu = CpuState::new();

        assert!(!cpu.is_halted());
        cpu.halt();
        assert!(cpu.is_halted());
        cpu.resume();
        assert!(!cpu.is_halted());
    }

    #[test]
    fn test_reset() {
        let mut cpu = CpuState::new();

        cpu.write_acc(42);
        cpu.set_iar(0x50).unwrap();
        cpu.halt();
        cpu.tick();
        cpu.count_instruction();

        cpu.reset();

        assert_eq!(cpu.read_acc(), 0);
        assert_eq!(cpu.iar(), PROGRAM_START);
        assert!(!cpu.is_halted());
        assert_eq!(cpu.cycle_count(), 0);
        assert_eq!(cpu.instruction_count(), 0);
    }

    #[test]
    fn test_load_program() {
        let mut cpu = CpuState::new();

        let program = vec![0x1234, 0x5678, 0x9ABC];
        cpu.load_program(PROGRAM_START, &program).unwrap();

        assert_eq!(cpu.read_word(PROGRAM_START).unwrap(), 0x1234);
        assert_eq!(cpu.read_word(PROGRAM_START + 1).unwrap(), 0x5678);
        assert_eq!(cpu.read_word(PROGRAM_START + 2).unwrap(), 0x9ABC);

        // Out of bounds
        let too_large = vec![0; MEMORY_SIZE + 1];
        assert!(cpu.load_program(0, &too_large).is_err());
    }
}
