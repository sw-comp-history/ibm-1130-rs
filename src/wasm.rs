//! WebAssembly bindings for IBM 1130 emulator
//!
//! This module provides JavaScript-accessible functions for the IBM 1130 CPU emulator.

use crate::cpu::{CpuState, Instruction};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// WASM-accessible CPU wrapper
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmCpu {
    cpu: CpuState,
}

/// CPU state snapshot for JavaScript
#[derive(Serialize, Deserialize)]
pub struct CpuSnapshot {
    pub acc: u16,
    pub ext: u16,
    pub iar: u16,
    pub xr1: u16,
    pub xr2: u16,
    pub xr3: u16,
    pub carry: bool,
    pub overflow: bool,
    pub halted: bool,
    pub cycle_count: u64,
    pub instruction_count: u64,
    pub memory: Vec<u16>,
}

#[wasm_bindgen]
impl WasmCpu {
    /// Create a new CPU instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: CpuState::new(),
        }
    }

    /// Reset CPU to initial state (keeps program in memory)
    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    /// Hard reset - clears all memory
    pub fn hard_reset(&mut self) {
        self.cpu.hard_reset();
    }

    /// Load a program into memory starting at address
    pub fn load_program(&mut self, start_addr: u16, data: Vec<u16>) -> Result<(), JsValue> {
        self.cpu
            .load_program(start_addr, &data)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Execute a single instruction at current IAR
    pub fn step(&mut self, opcode: u16) -> Result<(), JsValue> {
        let instr = self.decode(opcode)?;
        self.cpu
            .execute(&instr)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.cpu
            .increment_iar()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Run until WAIT instruction or error
    pub fn run(&mut self, max_cycles: u64) -> Result<(), JsValue> {
        for _ in 0..max_cycles {
            if self.cpu.is_halted() {
                break;
            }

            let iar = self.cpu.iar();
            let opcode = self
                .cpu
                .read_word(iar)
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            self.step(opcode)?;
        }
        Ok(())
    }

    /// Get CPU state as JSON
    pub fn get_state(&self) -> Result<JsValue, JsValue> {
        let snapshot = CpuSnapshot {
            acc: self.cpu.read_acc(),
            ext: self.cpu.read_ext(),
            iar: self.cpu.iar(),
            xr1: self.cpu.read_xr1(),
            xr2: self.cpu.read_xr2(),
            xr3: self.cpu.read_xr3(),
            carry: self.cpu.carry(),
            overflow: self.cpu.overflow(),
            halted: self.cpu.is_halted(),
            cycle_count: self.cpu.cycle_count(),
            instruction_count: self.cpu.instruction_count(),
            memory: self.cpu.memory().to_vec(),
        };

        serde_wasm_bindgen::to_value(&snapshot).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Read a word from memory
    pub fn read_memory(&self, addr: u16) -> Result<u16, JsValue> {
        self.cpu
            .read_word(addr)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Write a word to memory
    pub fn write_memory(&mut self, addr: u16, value: u16) -> Result<(), JsValue> {
        self.cpu
            .write_word(addr, value)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get accumulator value
    pub fn get_acc(&self) -> u16 {
        self.cpu.read_acc()
    }

    /// Get extension register value
    pub fn get_ext(&self) -> u16 {
        self.cpu.read_ext()
    }

    /// Get instruction address register (program counter)
    pub fn get_iar(&self) -> u16 {
        self.cpu.iar()
    }

    /// Get index register 1
    pub fn get_xr1(&self) -> u16 {
        self.cpu.read_xr1()
    }

    /// Get index register 2
    pub fn get_xr2(&self) -> u16 {
        self.cpu.read_xr2()
    }

    /// Get index register 3
    pub fn get_xr3(&self) -> u16 {
        self.cpu.read_xr3()
    }

    /// Check if CPU is halted
    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    /// Get cycle count
    pub fn get_cycle_count(&self) -> u64 {
        self.cpu.cycle_count()
    }

    /// Get instruction count
    pub fn get_instruction_count(&self) -> u64 {
        self.cpu.instruction_count()
    }

    /// Assemble source code and load into memory
    pub fn assemble(&mut self, source: String, start_addr: u16) -> Result<JsValue, JsValue> {
        use crate::assembler::Assembler;

        let mut assembler = Assembler::new();
        let program = assembler
            .assemble(&source)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Load program into memory
        self.cpu
            .load_program(start_addr, &program.code)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Convert listing to JSON for JavaScript
        #[derive(serde::Serialize)]
        struct ListingLine {
            address: u16,
            opcode: String,
            source: String,
        }

        // Adjust listing addresses to match where code was actually loaded
        let addr_offset = start_addr.wrapping_sub(program.start_addr);

        let listing: Vec<ListingLine> = program
            .listing
            .iter()
            .map(|line| ListingLine {
                address: line.address.wrapping_add(addr_offset), // Adjust address
                opcode: format!("0x{:04X}", line.opcode),
                source: line.source.clone(),
            })
            .collect();

        serde_wasm_bindgen::to_value(&listing).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get all available challenges
    pub fn get_challenges(&self) -> Result<JsValue, JsValue> {
        use crate::challenge::get_all_challenges;

        let challenges = get_all_challenges();
        serde_wasm_bindgen::to_value(&challenges).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Check current CPU state against a challenge
    pub fn check_challenge(&self, challenge_id: u32) -> Result<JsValue, JsValue> {
        use crate::challenge::get_all_challenges;

        let challenges = get_all_challenges();
        let challenge = challenges
            .iter()
            .find(|c| c.id == challenge_id)
            .ok_or_else(|| JsValue::from_str(&format!("Challenge {} not found", challenge_id)))?;

        let result = challenge
            .validate_solution(&self.cpu)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Decode an opcode into an instruction
    fn decode(&self, opcode: u16) -> Result<Instruction, JsValue> {
        crate::assembler::decode_instruction(opcode).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

impl Default for WasmCpu {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmCpu {
    /// Get direct reference to internal CPU state (for challenge validation)
    /// This method is not exported to WASM since it returns a reference
    pub fn cpu_state(&self) -> &CpuState {
        &self.cpu
    }
}

/// Initialize WASM module and mount Yew app
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Mount the Yew app
    yew::Renderer::<crate::app::App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_cpu_creation() {
        let cpu = WasmCpu::new();
        assert!(!cpu.is_halted());
        assert_eq!(cpu.get_acc(), 0);
        assert_eq!(cpu.get_cycle_count(), 0);
    }

    #[test]
    fn test_reset() {
        let mut cpu = WasmCpu::new();
        cpu.cpu.write_acc(42);
        cpu.cpu.halt();

        cpu.reset();

        assert_eq!(cpu.get_acc(), 0);
        assert!(!cpu.is_halted());
    }

    #[test]
    fn test_memory_access() {
        let mut cpu = WasmCpu::new();

        cpu.write_memory(100, 0x1234).unwrap();
        assert_eq!(cpu.read_memory(100).unwrap(), 0x1234);
    }
}
