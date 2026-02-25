//! Challenge system for the IBM 1130 assembly game
//!
//! Defines puzzles, test cases, and validation logic.

use crate::cpu::CpuState;
use serde::{Deserialize, Serialize};

/// Difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// A single test case for a challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// Name of this test case
    pub name: String,

    /// Initial memory values (address, value)
    #[serde(default)]
    pub initial_memory: Vec<(u16, u16)>,

    /// Expected accumulator value
    pub expected_acc: Option<u16>,

    /// Expected memory values (address, value)
    #[serde(default)]
    pub expected_memory: Vec<(u16, u16)>,

    /// Expected index register 1 value
    pub expected_xr1: Option<u16>,
}

impl TestCase {
    /// Check if the CPU state matches expected values
    pub fn validate(&self, cpu: &CpuState) -> Result<(), String> {
        // Check accumulator
        if let Some(expected_acc) = self.expected_acc {
            let actual = cpu.read_acc();
            if actual != expected_acc {
                return Err(format!(
                    "ACC mismatch: expected 0x{:04X} ({}), got 0x{:04X} ({})",
                    expected_acc, expected_acc as i16, actual, actual as i16
                ));
            }
        }

        // Check XR1
        if let Some(expected_xr1) = self.expected_xr1 {
            let actual = cpu.read_xr1();
            if actual != expected_xr1 {
                return Err(format!(
                    "XR1 mismatch: expected 0x{expected_xr1:04X} ({expected_xr1}), got 0x{actual:04X} ({actual})"
                ));
            }
        }

        // Check memory
        for (addr, expected) in &self.expected_memory {
            let actual = cpu
                .read_word(*addr)
                .map_err(|e| format!("Invalid memory address: {e}"))?;

            if actual != *expected {
                return Err(format!(
                    "Memory[0x{:04X}] mismatch: expected 0x{:04X} ({}), got 0x{:04X} ({})",
                    addr, expected, *expected as i16, actual, actual as i16
                ));
            }
        }

        Ok(())
    }
}

/// A challenge/puzzle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    /// Unique challenge ID
    pub id: u32,

    /// Challenge title
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Difficulty level
    pub difficulty: Difficulty,

    /// Test cases to validate the solution
    pub test_cases: Vec<TestCase>,

    /// Maximum allowed cycles (None = no limit)
    pub max_cycles: Option<u64>,

    /// Maximum allowed instructions (None = no limit)
    pub max_instructions: Option<u64>,

    /// Hints for the player
    #[serde(default)]
    pub hints: Vec<String>,

    /// Learning objectives
    #[serde(default)]
    pub learning_objectives: Vec<String>,
}

impl Challenge {
    /// Run all test cases against a CPU state
    pub fn validate_solution(&self, cpu: &CpuState) -> Result<ValidationResult, String> {
        let mut results = Vec::new();

        for test_case in self.test_cases.iter() {
            // Validate test case
            match test_case.validate(cpu) {
                Ok(()) => results.push(TestResult {
                    test_name: test_case.name.clone(),
                    passed: true,
                    error: None,
                    cycles: cpu.cycle_count(),
                    instructions: cpu.instruction_count(),
                }),
                Err(e) => results.push(TestResult {
                    test_name: test_case.name.clone(),
                    passed: false,
                    error: Some(e),
                    cycles: cpu.cycle_count(),
                    instructions: cpu.instruction_count(),
                }),
            }
        }

        let all_passed = results.iter().all(|r| r.passed);

        Ok(ValidationResult {
            challenge_id: self.id,
            passed: all_passed,
            test_results: results,
        })
    }
}

/// Result of validating a single test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub error: Option<String>,
    pub cycles: u64,
    pub instructions: u64,
}

/// Result of validating an entire challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub challenge_id: u32,
    pub passed: bool,
    pub test_results: Vec<TestResult>,
}

/// Get all available challenges
pub fn get_all_challenges() -> Vec<Challenge> {
    vec![
        challenge_1_load_value(),
        challenge_2_add_numbers(),
        challenge_3_use_index_register(),
    ]
}

/// Challenge 1: Load a Value
fn challenge_1_load_value() -> Challenge {
    Challenge {
        id: 1,
        title: "Challenge 1: Load a Value".to_string(),
        description:
            "Load the value 25 from memory address 0x0010 into the accumulator, then halt.\n\n\
                     Use the LD instruction to load from memory.\n\
                     Remember to end with WAIT!\n\n\
                     Hint: The value 25 is already stored at address 0x0010."
                .to_string(),
        difficulty: Difficulty::Beginner,
        test_cases: vec![TestCase {
            name: "ACC should contain 25".to_string(),
            initial_memory: vec![(0x0010, 25)],
            expected_acc: Some(25),
            expected_memory: vec![],
            expected_xr1: None,
        }],
        max_cycles: Some(100),
        max_instructions: Some(10),
        hints: vec![
            "The LD instruction loads from memory into ACC".to_string(),
            "Syntax: LD 0 addr loads from address addr into ACC".to_string(),
            "Don't forget WAIT to halt!".to_string(),
        ],
        learning_objectives: vec![
            "Understand the LD instruction".to_string(),
            "Learn about memory addressing".to_string(),
            "Practice using the accumulator".to_string(),
        ],
    }
}

/// Challenge 2: Add Two Numbers
fn challenge_2_add_numbers() -> Challenge {
    Challenge {
        id: 2,
        title: "Challenge 2: Add Two Numbers".to_string(),
        description: "Add two numbers from memory and store the result.\n\n\
                     - Address 0x0010 contains 15\n\
                     - Address 0x0011 contains 27\n\
                     - Store the sum (42) at address 0x0012\n\n\
                     Use LD to load, A to add, and STO to store."
            .to_string(),
        difficulty: Difficulty::Beginner,
        test_cases: vec![TestCase {
            name: "Memory[0x0012] should contain 42 (15 + 27)".to_string(),
            initial_memory: vec![(0x0010, 15), (0x0011, 27)],
            expected_acc: None, // Don't care about final ACC value
            expected_memory: vec![(0x0012, 42)],
            expected_xr1: None,
        }],
        max_cycles: Some(200),
        max_instructions: Some(20),
        hints: vec![
            "First, load the value from 0x0010 into ACC".to_string(),
            "Then, add the value from 0x0011 to ACC".to_string(),
            "Finally, store ACC to 0x0012".to_string(),
            "Example: LD 0 0x10, A 0 0x11, STO 0 0x12".to_string(),
        ],
        learning_objectives: vec![
            "Chain multiple instructions together".to_string(),
            "Use the A instruction for addition".to_string(),
            "Store results with STO".to_string(),
        ],
    }
}

/// Challenge 3: Use Index Register
fn challenge_3_use_index_register() -> Challenge {
    Challenge {
        id: 3,
        title: "Challenge 3: Use Index Register".to_string(),
        description: "Use index register XR1 to access memory.\n\n\
                     - Load the value 5 into XR1\n\
                     - Use XR1 to load from address (0x0010 + XR1) = 0x0015\n\
                     - The value at 0x0015 is 100\n\
                     - ACC should end up with 100\n\n\
                     This demonstrates indexed addressing mode."
            .to_string(),
        difficulty: Difficulty::Beginner,
        test_cases: vec![TestCase {
            name: "ACC should contain 100 using indexed load".to_string(),
            initial_memory: vec![(0x0015, 100)],
            expected_acc: Some(100),
            expected_memory: vec![],
            expected_xr1: Some(5),
        }],
        max_cycles: Some(200),
        max_instructions: Some(15),
        hints: vec![
            "First store 5 into memory location 0x0001 (XR1)".to_string(),
            "Use STO with index: STO 1 addr stores ACC to memory[addr]".to_string(),
            "Wait, you need to get 5 into ACC first, then store it at 0x0001".to_string(),
            "Then use LD 1 0x10 to load from address (0x10 + XR1) = 0x15".to_string(),
        ],
        learning_objectives: vec![
            "Understand index registers".to_string(),
            "Learn indexed addressing mode".to_string(),
            "Practice multi-step operations".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_program(program: &str) -> Result<CpuState, String> {
        use crate::assembler::Assembler;

        let mut cpu = CpuState::new();
        let mut assembler = Assembler::new();

        let program = assembler
            .assemble(program)
            .map_err(|e| format!("Assembly error: {e}"))?;

        cpu.load_program(4, &program.code)
            .map_err(|e| format!("Load error: {e}"))?;

        // Execute until WAIT or max iterations
        let max_iterations = 10000;
        for _ in 0..max_iterations {
            if cpu.is_halted() {
                break;
            }

            let iar = cpu.iar();
            let opcode = cpu.read_word(iar).map_err(|e| format!("Read error: {e}"))?;

            let instr = crate::assembler::decode_instruction(opcode)
                .map_err(|e| format!("Decode error: {e}"))?;

            cpu.execute(&instr)
                .map_err(|e| format!("Execute error: {e}"))?;

            cpu.increment_iar().map_err(|e| format!("IAR error: {e}"))?;
        }

        Ok(cpu)
    }

    #[test]
    fn test_challenge_1_valid_solution() {
        let challenge = challenge_1_load_value();

        // Set up initial memory as specified in test case
        let mut cpu = CpuState::new();
        for (addr, value) in &challenge.test_cases[0].initial_memory {
            cpu.write_word(*addr, *value).unwrap();
        }

        // Run the solution
        let program = "LD 0 0x10\nWAIT";
        let mut cpu = run_program(program).unwrap();

        // Apply initial memory again (since run_program creates new CPU)
        for (addr, value) in &challenge.test_cases[0].initial_memory {
            cpu.write_word(*addr, *value).unwrap();
        }

        // Actually run it properly
        use crate::assembler::Assembler;
        let mut assembler = Assembler::new();
        let prog = assembler.assemble(program).unwrap();
        cpu.load_program(4, &prog.code).unwrap();

        // Execute
        for _ in 0..10 {
            if cpu.is_halted() {
                break;
            }
            let iar = cpu.iar();
            let opcode = cpu.read_word(iar).unwrap();
            let instr = crate::assembler::decode_instruction(opcode).unwrap();
            cpu.execute(&instr).unwrap();
            cpu.increment_iar().unwrap();
        }

        let result = challenge.validate_solution(&cpu).unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_challenge_2_valid_solution() {
        let challenge = challenge_2_add_numbers();

        let mut cpu = CpuState::new();
        for (addr, value) in &challenge.test_cases[0].initial_memory {
            cpu.write_word(*addr, *value).unwrap();
        }

        use crate::assembler::Assembler;
        let mut assembler = Assembler::new();
        let program = "LD 0 0x10\nA 0 0x11\nSTO 0 0x12\nWAIT";
        let prog = assembler.assemble(program).unwrap();
        cpu.load_program(4, &prog.code).unwrap();

        // Execute
        for _ in 0..20 {
            if cpu.is_halted() {
                break;
            }
            let iar = cpu.iar();
            let opcode = cpu.read_word(iar).unwrap();
            let instr = crate::assembler::decode_instruction(opcode).unwrap();
            cpu.execute(&instr).unwrap();
            cpu.increment_iar().unwrap();
        }

        let result = challenge.validate_solution(&cpu).unwrap();
        assert!(result.passed);
    }
}
