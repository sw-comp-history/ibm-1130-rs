//! IBM 1130 instruction definitions
//!
//! This module defines the instruction types and addressing modes for the simplified IBM 1130 ISA.

/// Addressing modes supported by the IBM 1130
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// Direct addressing - use address as-is
    Direct,
    /// Indexed addressing - add XR1 to address
    Indexed,
}

/// Branch conditions for BSC instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchCondition {
    /// Branch if ACC == 0
    Zero,
    /// Branch if ACC != 0
    NonZero,
    /// Branch if ACC > 0 (positive, not zero)
    Positive,
    /// Branch if ACC < 0 (negative, sign bit set)
    Negative,
    /// Branch if overflow flag set
    Overflow,
    /// Branch if carry flag set
    Carry,
}

/// IBM 1130 instructions (simplified subset)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // ===== Load/Store Instructions =====
    /// Load ACC from memory
    LD { addr: u16, mode: AddressingMode },

    /// Store ACC to memory
    STO { addr: u16, mode: AddressingMode },

    /// Load index register XR1 from memory
    LDX { addr: u16 },

    /// Store index register XR1 to memory
    STX { addr: u16 },

    // ===== Arithmetic Instructions =====
    /// Add memory to ACC
    A { addr: u16, mode: AddressingMode },

    /// Subtract memory from ACC
    S { addr: u16, mode: AddressingMode },

    // ===== Logical Instructions =====
    /// Boolean AND with ACC
    AND { addr: u16, mode: AddressingMode },

    /// Boolean OR with ACC
    OR { addr: u16, mode: AddressingMode },

    // ===== Shift Instructions =====
    /// Shift Left ACC
    SLA { count: u8 },

    /// Shift Right ACC (arithmetic)
    SRA { count: u8 },

    // ===== Branch Instructions =====
    /// Branch or Skip on Condition
    BSC {
        addr: u16,
        condition: BranchCondition,
    },

    /// Branch and Store IAR (subroutine call)
    BSI { addr: u16 },

    // ===== Control Instructions =====
    /// Wait/Halt
    WAIT,

    /// No Operation
    NOP,
}

impl Instruction {
    /// Get the mnemonic string for this instruction
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Instruction::LD { .. } => "LD",
            Instruction::STO { .. } => "STO",
            Instruction::LDX { .. } => "LDX",
            Instruction::STX { .. } => "STX",
            Instruction::A { .. } => "A",
            Instruction::S { .. } => "S",
            Instruction::AND { .. } => "AND",
            Instruction::OR { .. } => "OR",
            Instruction::SLA { .. } => "SLA",
            Instruction::SRA { .. } => "SRA",
            Instruction::BSC { .. } => "BSC",
            Instruction::BSI { .. } => "BSI",
            Instruction::WAIT => "WAIT",
            Instruction::NOP => "NOP",
        }
    }
}

impl BranchCondition {
    /// Get the string representation of the branch condition
    pub fn to_str(&self) -> &'static str {
        match self {
            BranchCondition::Zero => "Z",
            BranchCondition::NonZero => "NZ",
            BranchCondition::Positive => "P",
            BranchCondition::Negative => "N",
            BranchCondition::Overflow => "V",
            BranchCondition::Carry => "C",
        }
    }

    /// Parse a branch condition from a string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "Z" => Some(BranchCondition::Zero),
            "NZ" => Some(BranchCondition::NonZero),
            "P" => Some(BranchCondition::Positive),
            "N" => Some(BranchCondition::Negative),
            "V" => Some(BranchCondition::Overflow),
            "C" => Some(BranchCondition::Carry),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_condition_str() {
        assert_eq!(BranchCondition::Zero.to_str(), "Z");
        assert_eq!(BranchCondition::NonZero.to_str(), "NZ");
        assert_eq!(BranchCondition::Positive.to_str(), "P");
    }

    #[test]
    fn test_branch_condition_parse() {
        assert_eq!(BranchCondition::parse("Z"), Some(BranchCondition::Zero));
        assert_eq!(BranchCondition::parse("nz"), Some(BranchCondition::NonZero));
        assert_eq!(BranchCondition::parse("invalid"), None);
    }

    #[test]
    fn test_instruction_mnemonic() {
        let ld = Instruction::LD {
            addr: 100,
            mode: AddressingMode::Direct,
        };
        assert_eq!(ld.mnemonic(), "LD");

        let wait = Instruction::WAIT;
        assert_eq!(wait.mnemonic(), "WAIT");
    }
}
