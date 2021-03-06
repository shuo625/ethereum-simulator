use super::instructions::Instruction;
use crate::eth_types::num_op;

pub struct PC {
    code: Vec<u8>,
    pc: usize,
}

impl PC {
    pub fn new(code: &str) -> Self {
        PC {
            code: {
                let bytes_origin = code.as_bytes();
                let mut bytes: Vec<u8> = Vec::new();
                let mut i = 0;
                while i < bytes_origin.len() - 1 {
                    bytes.push(num_op::u8s_to_u8(
                        bytes_origin[i] - b'0',
                        bytes_origin[i + 1] - b'0',
                    ));
                    i = i + 2;
                }
                bytes
            },
            pc: 0,
        }
    }

    pub fn next(&mut self) -> Option<Instruction> {
        if self.pc >= self.code.len() {
            return None;
        }
        let instruction = match self.code[self.pc] {
            0x00 => Some(Instruction::STOP),
            0x01 => Some(Instruction::ADD),
            0x02 => Some(Instruction::MUL),
            0x03 => Some(Instruction::SUB),
            0x04 => Some(Instruction::DIV),
            0x05 => Some(Instruction::SDIV),
            0x06 => Some(Instruction::MOD),
            0x07 => Some(Instruction::SMOD),
            0x08 => Some(Instruction::ADDMOD),
            0x09 => Some(Instruction::MULMOD),
            0x0a => Some(Instruction::EXP),
            0x0b => Some(Instruction::SIGNEXTEND),
            0x10 => Some(Instruction::LT),
            0x11 => Some(Instruction::GT),
            0x12 => Some(Instruction::SLT),
            0x13 => Some(Instruction::SGT),
            0x14 => Some(Instruction::EQ),
            0x15 => Some(Instruction::ISZERO),
            0x16 => Some(Instruction::AND),
            0x17 => Some(Instruction::OR),
            0x18 => Some(Instruction::XOR),
            0x19 => Some(Instruction::NOT),
            0x1a => Some(Instruction::BYTE),
            0x1b => Some(Instruction::SHL),
            0x1c => Some(Instruction::SHR),
            0x1d => Some(Instruction::SAR),
            0x20 => Some(Instruction::SHA3),
            0x30 => Some(Instruction::ADDRESS),
            0x31 => Some(Instruction::BALANCE),
            0x32 => Some(Instruction::ORIGIN),
            0x33 => Some(Instruction::CALLER),
            0x34 => Some(Instruction::CALLVALUE),
            0x35 => Some(Instruction::CALLDATALOAD),
            0x36 => Some(Instruction::CALLDATASIZE),
            0x37 => Some(Instruction::CALLDATACOPY),
            0x38 => Some(Instruction::CODESIZE),
            0x39 => Some(Instruction::CODECOPY),
            0x3a => Some(Instruction::GASPRICE),
            0x3b => Some(Instruction::EXTCODESIZE),
            0x3c => Some(Instruction::EXTCODECOPY),
            0x3d => Some(Instruction::RETURNDATASIZE),
            0x3e => Some(Instruction::RETURNDATACOPY),
            0x3f => Some(Instruction::EXTCODEHASH),
            0x40 => Some(Instruction::BLOCKHASH),
            0x41 => Some(Instruction::COINBASE),
            0x42 => Some(Instruction::TIMESTAMP),
            0x43 => Some(Instruction::NUMBER),
            0x44 => Some(Instruction::DIFFICULT),
            0x45 => Some(Instruction::GASLIMIT),
            0x46 => Some(Instruction::CHAINID),
            0x47 => Some(Instruction::SELFBALANCE),
            0x48 => Some(Instruction::BASEFEE),
            0x50 => Some(Instruction::POP),
            0x51 => Some(Instruction::MLOAD),
            0x52 => Some(Instruction::MSTORE),
            0x53 => Some(Instruction::MSTORE8),
            0x54 => Some(Instruction::SLOAD),
            0x55 => Some(Instruction::SSTORE),
            0x56 => Some(Instruction::JUMP),
            0x57 => Some(Instruction::JUMPI),
            0x58 => Some(Instruction::PC),
            0x59 => Some(Instruction::MSIZE),
            0x5a => Some(Instruction::GAS),
            0x5b => Some(Instruction::JUMPDEST),
            0x60 => {
                let value = self.code[self.pc + 1];
                self.pc += 1;
                Some(Instruction::PUSH1(value))
            }
            0x61 => {
                let value = num_op::u8s_to_u16(&self.code[self.pc..self.pc + 2]);
                self.pc += 2;
                Some(Instruction::PUSH2(value))
            }
            0x63 => {
                let value = num_op::u8s_to_u32(&self.code[self.pc..self.pc + 4]);
                self.pc += 4;
                Some(Instruction::PUSH4(value))
            }
            0x67 => {
                let value = num_op::u8s_to_u64(&self.code[self.pc..self.pc + 8]);
                self.pc += 8;
                Some(Instruction::PUSH8(value))
            }
            0x6f => {
                let value = num_op::u8s_to_u128(&self.code[self.pc..self.pc + 16]);
                self.pc += 16;
                Some(Instruction::PUSH16(value))
            }
            0x80..=0x8f => Some(Instruction::DUP((self.code[self.pc] - 0x80 + 1) as usize)),
            0x90..=0x9f => Some(Instruction::SWAP((self.code[self.pc] - 0x90 + 1) as usize)),
            0xa0..=0xa4 => Some(Instruction::LOG((self.code[self.pc] - 0xa0) as usize)),
            0xf0 => Some(Instruction::CREAT),
            0xf1 => Some(Instruction::CALL),
            0xf2 => Some(Instruction::CALLCODE),
            0xf3 => Some(Instruction::RETURN),
            0xf4 => Some(Instruction::DELEGATCALL),
            0xf5 => Some(Instruction::CREAT2),
            0xfa => Some(Instruction::STATICCALL),
            0xfd => Some(Instruction::REVERT),
            0xff => Some(Instruction::SELFDESTRUCT),
            _ => Some(Instruction::INVALID),
        };

        self.pc += 1;
        instruction
    }
}
