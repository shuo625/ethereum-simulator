use super::{
    super::eth_types::{Code, U256},
    instructions::Instruction,
};

pub struct PC {
    code: Code,
    pc: usize,
}

impl PC {
    pub fn new(code: Code) -> Self {
        PC { code, pc: 0 }
    }

    pub fn jump(&mut self, destination: U256) {
        let dest = destination.as_usize();
        self.pc = dest;
    }

    pub fn pc(&self) -> U256 {
        U256::from(self.pc)
    }

    pub fn next(&mut self) -> Option<Instruction> {
        if self.pc >= self.code.len() {
            return None;
        }
        let instruction = match self.code[self.pc] {
            0x00 => Instruction::STOP,
            0x01 => Instruction::ADD,
            0x02 => Instruction::MUL,
            0x03 => Instruction::SUB,
            0x04 => Instruction::DIV,
            0x05 => Instruction::SDIV,
            0x06 => Instruction::MOD,
            0x07 => Instruction::SMOD,
            0x08 => Instruction::ADDMOD,
            0x09 => Instruction::MULMOD,
            0x0a => Instruction::EXP,
            0x0b => Instruction::SIGNEXTEND,
            0x10 => Instruction::LT,
            0x11 => Instruction::GT,
            0x12 => Instruction::SLT,
            0x13 => Instruction::SGT,
            0x14 => Instruction::EQ,
            0x15 => Instruction::ISZERO,
            0x16 => Instruction::AND,
            0x17 => Instruction::OR,
            0x18 => Instruction::XOR,
            0x19 => Instruction::NOT,
            0x1a => Instruction::BYTE,
            0x1b => Instruction::SHL,
            0x1c => Instruction::SHR,
            0x1d => Instruction::SAR,
            0x20 => Instruction::SHA3,
            0x30 => Instruction::ADDRESS,
            0x31 => Instruction::BALANCE,
            0x32 => Instruction::ORIGIN,
            0x33 => Instruction::CALLER,
            0x34 => Instruction::CALLVALUE,
            0x35 => Instruction::CALLDATALOAD,
            0x36 => Instruction::CALLDATASIZE,
            0x37 => Instruction::CALLDATACOPY,
            0x38 => Instruction::CODESIZE,
            0x39 => Instruction::CODECOPY,
            0x3a => Instruction::GASPRICE,
            0x3b => Instruction::EXTCODESIZE,
            0x3c => Instruction::EXTCODECOPY,
            0x3d => Instruction::RETURNDATASIZE,
            0x3e => Instruction::RETURNDATACOPY,
            0x3f => Instruction::EXTCODEHASH,
            0x40 => Instruction::BLOCKHASH,
            0x41 => Instruction::COINBASE,
            0x42 => Instruction::TIMESTAMP,
            0x43 => Instruction::NUMBER,
            0x44 => Instruction::DIFFICULT,
            0x45 => Instruction::GASLIMIT,
            0x46 => Instruction::CHAINID,
            0x47 => Instruction::SELFBALANCE,
            0x48 => Instruction::BASEFEE,
            0x50 => Instruction::POP,
            0x51 => Instruction::MLOAD,
            0x52 => Instruction::MSTORE,
            0x53 => Instruction::MSTORE8,
            0x54 => Instruction::SLOAD,
            0x55 => Instruction::SSTORE,
            0x56 => Instruction::JUMP,
            0x57 => Instruction::JUMPI,
            0x58 => Instruction::PC,
            0x59 => Instruction::MSIZE,
            0x5a => Instruction::GAS,
            0x5b => Instruction::JUMPDEST,
            0x60..=0x7f => {
                let size = (self.code[self.pc] - 0x60 + 1) as usize;
                let instr = Instruction::PUSH(U256::from_big_endian(
                    &self.code[self.pc + 1..self.pc + size + 1],
                ));
                self.pc += size;
                instr
            }
            0x80..=0x8f => Instruction::DUP((self.code[self.pc] - 0x80 + 1) as usize),
            0x90..=0x9f => Instruction::SWAP((self.code[self.pc] - 0x90 + 1) as usize),
            0xa0..=0xa4 => Instruction::LOG((self.code[self.pc] - 0xa0) as usize),
            0xf0 => Instruction::CREAT,
            0xf1 => Instruction::CALL,
            0xf2 => Instruction::CALLCODE,
            0xf3 => Instruction::RETURN,
            0xf4 => Instruction::DELEGATCALL,
            0xf5 => Instruction::CREAT2,
            0xfa => Instruction::STATICCALL,
            0xfd => Instruction::REVERT,
            0xff => Instruction::SELFDESTRUCT,
            _ => Instruction::INVALID,
        };

        self.pc += 1;
        Some(instruction)
    }
}
