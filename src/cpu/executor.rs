//! IBM 1130 instruction execution
//!
//! This module implements the execution logic for IBM 1130 instructions.

use super::instruction::{AddressingMode, BranchCondition, Instruction};
use super::state::{CpuError, CpuState};

impl CpuState {
    /// Execute a single instruction
    pub fn execute(&mut self, instr: &Instruction) -> Result<(), CpuError> {
        if self.is_halted() {
            return Err(CpuError::Halted);
        }

        match instr {
            // Load/Store
            Instruction::LD { addr, mode } => self.exec_ld(*addr, *mode)?,
            Instruction::STO { addr, mode } => self.exec_sto(*addr, *mode)?,
            Instruction::LDX { addr } => self.exec_ldx(*addr)?,
            Instruction::STX { addr } => self.exec_stx(*addr)?,

            // Arithmetic
            Instruction::A { addr, mode } => self.exec_add(*addr, *mode)?,
            Instruction::S { addr, mode } => self.exec_sub(*addr, *mode)?,

            // Logical
            Instruction::AND { addr, mode } => self.exec_and(*addr, *mode)?,
            Instruction::OR { addr, mode } => self.exec_or(*addr, *mode)?,

            // Shift
            Instruction::SLA { count } => self.exec_sla(*count),
            Instruction::SRA { count } => self.exec_sra(*count),

            // Branch
            Instruction::BSC { addr, condition } => self.exec_bsc(*addr, *condition)?,
            Instruction::BSI { addr } => self.exec_bsi(*addr)?,

            // Control
            Instruction::WAIT => self.halt(),
            Instruction::NOP => {} // Do nothing
        }

        self.count_instruction();
        self.tick();
        Ok(())
    }

    /// Calculate effective address based on addressing mode
    fn effective_address(&self, addr: u16, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Direct => addr,
            AddressingMode::Indexed => addr.wrapping_add(self.read_xr1()),
        }
    }

    // ===== Load/Store Instructions =====

    fn exec_ld(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let value = self.read_word(ea)?;
        self.write_acc(value);
        Ok(())
    }

    fn exec_sto(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let value = self.read_acc();
        self.write_word(ea, value)?;
        Ok(())
    }

    fn exec_ldx(&mut self, addr: u16) -> Result<(), CpuError> {
        let value = self.read_word(addr)?;
        self.write_xr1(value);
        Ok(())
    }

    fn exec_stx(&mut self, addr: u16) -> Result<(), CpuError> {
        let value = self.read_xr1();
        self.write_word(addr, value)?;
        Ok(())
    }

    // ===== Arithmetic Instructions =====

    fn exec_add(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let operand = self.read_word(ea)?;
        let acc = self.read_acc();
        let result = acc.wrapping_add(operand);
        self.write_acc(result);
        self.update_flags_add(acc, operand, result);
        Ok(())
    }

    fn exec_sub(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let operand = self.read_word(ea)?;
        let acc = self.read_acc();
        let result = acc.wrapping_sub(operand);
        self.write_acc(result);
        self.update_flags_sub(acc, operand, result);
        Ok(())
    }

    // ===== Logical Instructions =====

    fn exec_and(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let operand = self.read_word(ea)?;
        let result = self.read_acc() & operand;
        self.write_acc(result);
        Ok(())
    }

    fn exec_or(&mut self, addr: u16, mode: AddressingMode) -> Result<(), CpuError> {
        let ea = self.effective_address(addr, mode);
        let operand = self.read_word(ea)?;
        let result = self.read_acc() | operand;
        self.write_acc(result);
        Ok(())
    }

    // ===== Shift Instructions =====

    fn exec_sla(&mut self, count: u8) {
        let acc = self.read_acc();
        let result = acc << count;
        self.write_acc(result);
    }

    fn exec_sra(&mut self, count: u8) {
        let acc = self.read_acc() as i16; // Arithmetic shift preserves sign
        let result = (acc >> count) as u16;
        self.write_acc(result);
    }

    // ===== Branch Instructions =====

    fn exec_bsc(&mut self, addr: u16, condition: BranchCondition) -> Result<(), CpuError> {
        let should_branch = match condition {
            BranchCondition::Zero => self.read_acc() == 0,
            BranchCondition::NonZero => self.read_acc() != 0,
            BranchCondition::Positive => {
                let acc = self.read_acc() as i16;
                acc > 0
            }
            BranchCondition::Negative => {
                let acc = self.read_acc() as i16;
                acc < 0
            }
            BranchCondition::Overflow => self.overflow(),
            BranchCondition::Carry => self.carry(),
        };

        if should_branch {
            self.set_iar(addr)?;
        }
        Ok(())
    }

    fn exec_bsi(&mut self, addr: u16) -> Result<(), CpuError> {
        // Store current IAR at target address (for return)
        let return_addr = self.iar();
        self.write_word(addr, return_addr)?;
        // Branch to addr + 1
        self.set_iar(addr.wrapping_add(1))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::instruction::AddressingMode;

    #[test]
    fn test_ld_direct() {
        let mut cpu = CpuState::new();
        cpu.write_word(0x50, 0x1234).unwrap();

        let instr = Instruction::LD {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0x1234);
        assert_eq!(cpu.instruction_count(), 1);
    }

    #[test]
    fn test_ld_indexed() {
        let mut cpu = CpuState::new();
        cpu.write_xr1(5);
        cpu.write_word(105, 0xABCD).unwrap();

        let instr = Instruction::LD {
            addr: 100,
            mode: AddressingMode::Indexed,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0xABCD);
    }

    #[test]
    fn test_sto() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0x5678);

        let instr = Instruction::STO {
            addr: 0x60,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_word(0x60).unwrap(), 0x5678);
    }

    #[test]
    fn test_add() {
        let mut cpu = CpuState::new();
        cpu.write_acc(10);
        cpu.write_word(0x50, 20).unwrap();

        let instr = Instruction::A {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 30);
    }

    #[test]
    fn test_add_overflow() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0xFFFF);
        cpu.write_word(0x50, 1).unwrap();

        let instr = Instruction::A {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0);
        assert!(cpu.carry());
    }

    #[test]
    fn test_sub() {
        let mut cpu = CpuState::new();
        cpu.write_acc(30);
        cpu.write_word(0x50, 10).unwrap();

        let instr = Instruction::S {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 20);
    }

    #[test]
    fn test_and() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0b1111_0000);
        cpu.write_word(0x50, 0b1010_1010).unwrap();

        let instr = Instruction::AND {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0b1010_0000);
    }

    #[test]
    fn test_or() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0b1111_0000);
        cpu.write_word(0x50, 0b0000_1111).unwrap();

        let instr = Instruction::OR {
            addr: 0x50,
            mode: AddressingMode::Direct,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0b1111_1111);
    }

    #[test]
    fn test_sla() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0b0000_0001);

        let instr = Instruction::SLA { count: 3 };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), 0b0000_1000);
    }

    #[test]
    fn test_sra() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0b1000_0000_0000_0000); // Negative number

        let instr = Instruction::SRA { count: 2 };
        cpu.execute(&instr).unwrap();

        // Arithmetic shift preserves sign
        assert_eq!(cpu.read_acc(), 0b1110_0000_0000_0000);
    }

    #[test]
    fn test_bsc_zero() {
        let mut cpu = CpuState::new();
        cpu.write_acc(0);
        cpu.set_iar(0x10).unwrap();

        let instr = Instruction::BSC {
            addr: 0x50,
            condition: BranchCondition::Zero,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.iar(), 0x50);
    }

    #[test]
    fn test_bsc_no_branch() {
        let mut cpu = CpuState::new();
        cpu.write_acc(5);
        cpu.set_iar(0x10).unwrap();

        let instr = Instruction::BSC {
            addr: 0x50,
            condition: BranchCondition::Zero,
        };
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.iar(), 0x10); // Should not branch
    }

    #[test]
    fn test_wait() {
        let mut cpu = CpuState::new();

        let instr = Instruction::WAIT;
        cpu.execute(&instr).unwrap();

        assert!(cpu.is_halted());
    }

    #[test]
    fn test_nop() {
        let mut cpu = CpuState::new();
        let initial_state = cpu.read_acc();

        let instr = Instruction::NOP;
        cpu.execute(&instr).unwrap();

        assert_eq!(cpu.read_acc(), initial_state);
        assert_eq!(cpu.instruction_count(), 1);
    }
}
