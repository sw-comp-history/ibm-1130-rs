//! IBM 1130 assembler and disassembler
//!
//! This module provides assembly parsing, opcode encoding, and decoding functionality.

use crate::cpu::{AddressingMode, BranchCondition, Instruction};
use thiserror::Error;

/// Assembly errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum AssemblerError {
    #[error("Invalid instruction mnemonic: {0}")]
    InvalidMnemonic(String),

    #[error("Invalid addressing mode: {0}")]
    InvalidMode(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid operand: {0}")]
    InvalidOperand(String),

    #[error("Missing operand for instruction {0}")]
    MissingOperand(String),

    #[error("Invalid branch condition: {0}")]
    InvalidCondition(String),

    #[error("Invalid shift count: {0}")]
    InvalidShiftCount(String),

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Invalid DATA directive address: {0}")]
    InvalidDataAddress(String),

    #[error("Invalid DATA directive value: {0}")]
    InvalidDataValue(String),
}

/// Assembled program result
#[derive(Debug, Clone)]
pub struct AssembledProgram {
    /// Machine code words
    pub code: Vec<u16>,
    /// Starting address
    pub start_addr: u16,
    /// Assembly listing (address, opcode, source line)
    pub listing: Vec<AssemblyLine>,
}

/// Single line of assembly listing
#[derive(Debug, Clone)]
pub struct AssemblyLine {
    pub address: u16,
    pub opcode: u16,
    pub source: String,
}

/// IBM 1130 Assembler
pub struct Assembler {
    current_addr: u16,
}

impl Assembler {
    /// Create a new assembler starting at program start address
    pub fn new() -> Self {
        Self {
            current_addr: crate::cpu::PROGRAM_START,
        }
    }

    /// Assemble a complete program from source text
    pub fn assemble(&mut self, source: &str) -> Result<AssembledProgram, AssemblerError> {
        let mut code = Vec::new();
        let mut listing = Vec::new();
        let start_addr = self.current_addr;

        for line in source.lines() {
            // Remove comments
            let line = if let Some(pos) = line.find(';') {
                &line[..pos]
            } else {
                line
            };

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Check if this is an ORG directive
            if line.to_uppercase().starts_with("ORG") {
                let new_addr = self.parse_org_directive(line)?;
                self.current_addr = new_addr;
                continue;
            }

            // Check if this is a DATA directive
            if line.to_uppercase().starts_with("DATA") {
                let (_addr, _value) = self.parse_data_directive(line)?;
                // DATA directives set values at specific addresses, not sequential
                // For now, just skip them in the listing
                continue;
            }

            // Parse instruction
            let instr = self.parse_line(line)?;
            let opcode = encode_instruction(&instr)?;

            listing.push(AssemblyLine {
                address: self.current_addr,
                opcode,
                source: line.to_string(),
            });

            code.push(opcode);
            self.current_addr += 1;
        }

        Ok(AssembledProgram {
            code,
            start_addr,
            listing,
        })
    }

    /// Parse a single line of assembly into an Instruction
    fn parse_line(&self, line: &str) -> Result<Instruction, AssemblerError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(AssemblerError::SyntaxError("Empty line".to_string()));
        }

        let mnemonic = parts[0].to_uppercase();

        match mnemonic.as_str() {
            // Load/Store with addressing mode
            "LD" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("LD".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::LD { addr, mode })
            }
            "STO" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("STO".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::STO { addr, mode })
            }
            "A" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("A".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::A { addr, mode })
            }
            "S" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("S".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::S { addr, mode })
            }
            "AND" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("AND".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::AND { addr, mode })
            }
            "OR" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("OR".to_string()));
                }
                let mode = self.parse_mode(parts[1])?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::OR { addr, mode })
            }

            // Index register operations (direct addressing only)
            "LDX" => {
                if parts.len() < 2 {
                    return Err(AssemblerError::MissingOperand("LDX".to_string()));
                }
                let addr = self.parse_address(parts[1])?;
                Ok(Instruction::LDX { addr })
            }
            "STX" => {
                if parts.len() < 2 {
                    return Err(AssemblerError::MissingOperand("STX".to_string()));
                }
                let addr = self.parse_address(parts[1])?;
                Ok(Instruction::STX { addr })
            }

            // Shift operations
            "SLA" => {
                if parts.len() < 2 {
                    return Err(AssemblerError::MissingOperand("SLA".to_string()));
                }
                let count = self.parse_shift_count(parts[1])?;
                Ok(Instruction::SLA { count })
            }
            "SRA" => {
                if parts.len() < 2 {
                    return Err(AssemblerError::MissingOperand("SRA".to_string()));
                }
                let count = self.parse_shift_count(parts[1])?;
                Ok(Instruction::SRA { count })
            }

            // Branch operations
            "BSC" => {
                if parts.len() < 3 {
                    return Err(AssemblerError::MissingOperand("BSC".to_string()));
                }
                let condition = BranchCondition::parse(parts[1])
                    .ok_or_else(|| AssemblerError::InvalidCondition(parts[1].to_string()))?;
                let addr = self.parse_address(parts[2])?;
                Ok(Instruction::BSC { addr, condition })
            }
            "BSI" => {
                if parts.len() < 2 {
                    return Err(AssemblerError::MissingOperand("BSI".to_string()));
                }
                let addr = self.parse_address(parts[1])?;
                Ok(Instruction::BSI { addr })
            }

            // Control
            "WAIT" => Ok(Instruction::WAIT),
            "NOP" => Ok(Instruction::NOP),

            _ => Err(AssemblerError::InvalidMnemonic(mnemonic)),
        }
    }

    /// Parse addressing mode (0 = direct, 1 = indexed)
    fn parse_mode(&self, s: &str) -> Result<AddressingMode, AssemblerError> {
        match s {
            "0" => Ok(AddressingMode::Direct),
            "1" => Ok(AddressingMode::Indexed),
            _ => Err(AssemblerError::InvalidMode(s.to_string())),
        }
    }

    /// Parse address (supports decimal and hex with 0x prefix)
    fn parse_address(&self, s: &str) -> Result<u16, AssemblerError> {
        if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            u16::from_str_radix(hex, 16).map_err(|_| AssemblerError::InvalidAddress(s.to_string()))
        } else {
            s.parse::<u16>()
                .map_err(|_| AssemblerError::InvalidAddress(s.to_string()))
        }
    }

    /// Parse shift count
    fn parse_shift_count(&self, s: &str) -> Result<u8, AssemblerError> {
        s.parse::<u8>()
            .map_err(|_| AssemblerError::InvalidShiftCount(s.to_string()))
    }

    /// Parse ORG directive (e.g., "ORG 16" or "ORG 0x10")
    fn parse_org_directive(&self, line: &str) -> Result<u16, AssemblerError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(AssemblerError::SyntaxError(
                "ORG directive requires an address".to_string(),
            ));
        }

        self.parse_address(parts[1])
            .map_err(|_| AssemblerError::InvalidDataAddress(parts[1].to_string()))
    }

    /// Parse DATA directive (e.g., "DATA 10 5")
    fn parse_data_directive(&self, line: &str) -> Result<(u16, u16), AssemblerError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(AssemblerError::SyntaxError(
                "DATA directive requires address and value".to_string(),
            ));
        }

        let addr = self
            .parse_address(parts[1])
            .map_err(|_| AssemblerError::InvalidDataAddress(parts[1].to_string()))?;
        let value = self
            .parse_address(parts[2])
            .map_err(|_| AssemblerError::InvalidDataValue(parts[2].to_string()))?;

        Ok((addr, value))
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Encode an instruction into a 16-bit opcode
///
/// Simplified encoding scheme for educational purposes:
/// - Bits 15-12: Opcode
/// - Bits 11-8: Subopcode/modifier
/// - Bits 7-0: Address/operand
pub fn encode_instruction(instr: &Instruction) -> Result<u16, AssemblerError> {
    match instr {
        Instruction::LD { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x1000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::STO { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x2000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::LDX { addr } => Ok(0x3000 | (addr & 0xFF)),
        Instruction::STX { addr } => Ok(0x4000 | (addr & 0xFF)),
        Instruction::A { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x5000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::S { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x6000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::AND { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x7000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::OR { addr, mode } => {
            let mode_bit = if matches!(mode, AddressingMode::Indexed) {
                1
            } else {
                0
            };
            Ok(0x8000 | (mode_bit << 8) | (addr & 0xFF))
        }
        Instruction::SLA { count } => Ok(0x9000 | (*count as u16)),
        Instruction::SRA { count } => Ok(0xA000 | (*count as u16)),
        Instruction::BSC { addr, condition } => {
            let cond_bits = match condition {
                BranchCondition::Zero => 0,
                BranchCondition::NonZero => 1,
                BranchCondition::Positive => 2,
                BranchCondition::Negative => 3,
                BranchCondition::Overflow => 4,
                BranchCondition::Carry => 5,
            };
            Ok(0xB000 | (cond_bits << 8) | (addr & 0xFF))
        }
        Instruction::BSI { addr } => Ok(0xC000 | (addr & 0xFF)),
        Instruction::WAIT => Ok(0xF000),
        Instruction::NOP => Ok(0x0000),
    }
}

/// Decode a 16-bit opcode into an Instruction
pub fn decode_instruction(opcode: u16) -> Result<Instruction, AssemblerError> {
    let op = (opcode >> 12) & 0xF;
    let modifier = (opcode >> 8) & 0xF;
    let addr = opcode & 0xFF;

    match op {
        0x0 => Ok(Instruction::NOP),
        0x1 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::LD { addr, mode })
        }
        0x2 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::STO { addr, mode })
        }
        0x3 => Ok(Instruction::LDX { addr }),
        0x4 => Ok(Instruction::STX { addr }),
        0x5 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::A { addr, mode })
        }
        0x6 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::S { addr, mode })
        }
        0x7 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::AND { addr, mode })
        }
        0x8 => {
            let mode = if modifier == 1 {
                AddressingMode::Indexed
            } else {
                AddressingMode::Direct
            };
            Ok(Instruction::OR { addr, mode })
        }
        0x9 => Ok(Instruction::SLA {
            count: (opcode & 0xFF) as u8,
        }),
        0xA => Ok(Instruction::SRA {
            count: (opcode & 0xFF) as u8,
        }),
        0xB => {
            let condition = match modifier {
                0 => BranchCondition::Zero,
                1 => BranchCondition::NonZero,
                2 => BranchCondition::Positive,
                3 => BranchCondition::Negative,
                4 => BranchCondition::Overflow,
                5 => BranchCondition::Carry,
                _ => {
                    return Err(AssemblerError::InvalidCondition(format!(
                        "Unknown condition code: {modifier}"
                    )));
                }
            };
            Ok(Instruction::BSC { addr, condition })
        }
        0xC => Ok(Instruction::BSI { addr }),
        0xF => Ok(Instruction::WAIT),
        _ => Err(AssemblerError::InvalidMnemonic(format!(
            "Unknown opcode: 0x{op:X}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_instructions() {
        let asm = Assembler::new();

        let instr = asm.parse_line("LD 0 10").unwrap();
        assert!(matches!(
            instr,
            Instruction::LD {
                addr: 10,
                mode: AddressingMode::Direct
            }
        ));

        let instr = asm.parse_line("STO 1 20").unwrap();
        assert!(matches!(
            instr,
            Instruction::STO {
                addr: 20,
                mode: AddressingMode::Indexed
            }
        ));

        let instr = asm.parse_line("WAIT").unwrap();
        assert!(matches!(instr, Instruction::WAIT));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let instructions = vec![
            Instruction::LD {
                addr: 10,
                mode: AddressingMode::Direct,
            },
            Instruction::A {
                addr: 20,
                mode: AddressingMode::Indexed,
            },
            Instruction::SLA { count: 3 },
            Instruction::BSC {
                addr: 100,
                condition: BranchCondition::Zero,
            },
            Instruction::WAIT,
            Instruction::NOP,
        ];

        for original in instructions {
            let opcode = encode_instruction(&original).unwrap();
            let decoded = decode_instruction(opcode).unwrap();
            assert_eq!(original, decoded);
        }
    }

    #[test]
    fn test_assemble_program() {
        let source = r#"
            LD 0 10
            A 0 11
            STO 0 12
            WAIT
        "#;

        let mut asm = Assembler::new();
        let result = asm.assemble(source).unwrap();

        assert_eq!(result.code.len(), 4);
        assert_eq!(result.start_addr, crate::cpu::PROGRAM_START);
        assert_eq!(result.listing.len(), 4);
    }

    #[test]
    fn test_parse_hex_addresses() {
        let asm = Assembler::new();
        let instr = asm.parse_line("LD 0 0x10").unwrap();
        assert!(matches!(
            instr,
            Instruction::LD {
                addr: 16,
                mode: AddressingMode::Direct
            }
        ));
    }

    #[test]
    fn test_parse_branch_conditions() {
        let asm = Assembler::new();

        let instr = asm.parse_line("BSC Z 50").unwrap();
        assert!(matches!(
            instr,
            Instruction::BSC {
                addr: 50,
                condition: BranchCondition::Zero
            }
        ));

        let instr = asm.parse_line("BSC NZ 60").unwrap();
        assert!(matches!(
            instr,
            Instruction::BSC {
                addr: 60,
                condition: BranchCondition::NonZero
            }
        ));
    }
}
