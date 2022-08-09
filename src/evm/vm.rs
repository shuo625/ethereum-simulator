use core::panic;
use std::str::FromStr;

use ethereum_types::{BigEndianHash, H256};

use super::{ext::Ext, instructions::Instruction, memory::Memory, pc::PC, stack::Stack};
use crate::{
    eth_types::{Address, Code, U256},
    hash,
};

pub struct VM {
    stack: Stack,
    memory: Memory,
    pc: PC,
}

impl VM {
    pub fn new(code: Code) -> Self {
        VM {
            stack: Stack::new(),
            memory: Memory::new(),
            pc: PC::new(code),
        }
    }

    fn stack_one_item_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256) -> U256,
    {
        let a = self.stack.pop();
        self.stack.push(op(a));
    }

    fn stack_two_items_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256, U256) -> U256,
    {
        let a = self.stack.pop();
        let b = self.stack.pop();
        self.stack.push(op(a, b));
    }

    fn stack_three_items_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256, U256, U256) -> U256,
    {
        let a = self.stack.pop();
        let b = self.stack.pop();
        let c = self.stack.pop();
        self.stack.push(op(a, b, c));
    }

    pub fn execute(&mut self, ext: &mut Ext) {
        while let Some(instruction) = self.pc.next() {
            match instruction {
                Instruction::STOP => return,
                Instruction::ADD => self.stack_two_items_op(|a, b| a + b),
                Instruction::MUL => self.stack_two_items_op(|a, b| a * b),
                Instruction::SUB => self.stack_two_items_op(|a, b| a - b),
                // TODO: handle SDIV
                Instruction::DIV | Instruction::SDIV => self.stack_two_items_op(|a, b| a / b),
                // TODO: handle SMOD
                Instruction::MOD | Instruction::SMOD => self.stack_two_items_op(|a, b| a % b),
                Instruction::ADDMOD => self.stack_three_items_op(|a, b, n| (a + b) % n),
                Instruction::MULMOD => self.stack_three_items_op(|a, b, n| (a * b) % n),
                Instruction::EXP => self.stack_two_items_op(U256::pow),
                // TODO: handle this
                Instruction::SIGNEXTEND => self.stack_two_items_op(|b, x| b),
                // TODO: handle SLT
                Instruction::LT | Instruction::SLT => {
                    self.stack_two_items_op(|a, b| if a < b { U256::one() } else { U256::zero() })
                }
                // TODO: handle SGT
                Instruction::GT | Instruction::SGT => {
                    self.stack_two_items_op(|a, b| if a > b { U256::one() } else { U256::zero() })
                }
                Instruction::EQ => {
                    self.stack_two_items_op(|a, b| if a == b { U256::one() } else { U256::zero() })
                }
                Instruction::ISZERO => self.stack_one_item_op(|a| {
                    if a == U256::zero() {
                        U256::one()
                    } else {
                        U256::zero()
                    }
                }),
                Instruction::AND => self.stack_two_items_op(|a, b| a & b),
                Instruction::OR => self.stack_two_items_op(|a, b| a | b),
                Instruction::XOR => self.stack_two_items_op(|a, b| a ^ b),
                Instruction::NOT => self.stack_one_item_op(|a| !a),
                Instruction::BYTE => {
                    let i = self.stack.pop();
                    let x = self.stack.pop();
                    self.stack.push(U256::from(x.byte(i.as_usize())));
                }
                Instruction::SHL => self.stack_two_items_op(|shl, value| value << shl),
                Instruction::SHR | Instruction::SAR => {
                    self.stack_two_items_op(|shr, value| value >> shr)
                }
                Instruction::SHA3 => {
                    let offset = self.stack.pop();
                    let size = self.stack.pop();
                    let k = hash::keccak(self.memory.read_slice(offset, size));
                    self.stack.push(U256::from(k.as_bytes()));
                }
                Instruction::ADDRESS => self.stack.push(ext.get_address()),
                Instruction::BALANCE => {
                    let address = Address::from_str(&self.stack.pop().to_string()).unwrap();
                    self.stack.push(ext.get_balance(&address));
                }
                Instruction::ORIGIN => self.stack.push(ext.get_origin()),
                Instruction::CALLER => self.stack.push(ext.get_caller()),
                Instruction::CALLVALUE => self.stack.push(ext.get_callvalue()),
                Instruction::CALLDATALOAD => {
                    let i = self.stack.pop();
                    self.stack.push(ext.get_calldata(i));
                }
                Instruction::CALLDATASIZE => self.stack.push(ext.get_calldatasize()),
                Instruction::CALLDATACOPY => {
                    let dest_offset = self.stack.pop();
                    let offset = self.stack.pop();
                    let length = self.stack.pop();
                    self.memory
                        .write_slice(dest_offset, ext.get_calldata_slice(offset, length));
                }
                Instruction::CODESIZE => self.stack.push(ext.get_codesize()),
                Instruction::CODECOPY => {
                    let dest_offset = self.stack.pop();
                    let offset = self.stack.pop();
                    let length = self.stack.pop();
                    self.memory
                        .write_slice(dest_offset, ext.get_code_slice(offset, length));
                }
                Instruction::GASPRICE => {}
                Instruction::EXTCODESIZE => {}
                Instruction::EXTCODECOPY => {}
                Instruction::RETURNDATASIZE => {}
                Instruction::RETURNDATACOPY => {}
                Instruction::EXTCODEHASH => {}
                Instruction::BLOCKHASH => {}
                Instruction::COINBASE => {}
                Instruction::TIMESTAMP => {}
                Instruction::NUMBER => {}
                Instruction::DIFFICULT => {}
                Instruction::GASLIMIT => {}
                Instruction::CHAINID => self.stack.push(ext.get_chainid()),
                Instruction::SELFBALANCE => {}
                Instruction::BASEFEE => {}
                Instruction::POP => {
                    self.stack.pop();
                }
                Instruction::MLOAD => {
                    let offset = self.stack.pop();
                    self.stack.push(self.memory.read(offset));
                }
                Instruction::MSTORE => {
                    let offset = self.stack.pop();
                    let value = self.stack.pop();
                    self.memory.write(offset, value);
                }
                Instruction::MSTORE8 => {
                    let offset = self.stack.pop();
                    let value = self.stack.pop();
                    self.memory.write_byte(offset, value);
                }
                Instruction::SLOAD => {
                    let key = H256::from_uint(&self.stack.pop());
                    self.stack.push(ext.get_storage(&key).into_uint());
                }
                Instruction::SSTORE => {
                    let key = H256::from_uint(&self.stack.pop());
                    let value = H256::from_uint(&self.stack.pop());
                    ext.set_storage(key, value);
                }
                Instruction::JUMP => self.pc.jump(self.stack.pop()),
                Instruction::JUMPI => {
                    let destination = self.stack.pop();
                    let condition = self.stack.pop();
                    if !condition.is_zero() {
                        self.pc.jump(destination);
                    }
                }
                Instruction::PC => self.stack.push(self.pc.pc()),
                Instruction::MSIZE => self.stack.push(self.memory.size()),
                Instruction::GAS => self.stack.push(ext.get_gas()),
                Instruction::JUMPDEST => {}
                Instruction::PUSH(value) => self.stack.push(value),
                Instruction::DUP(i) => self.stack.dup_top(i),
                Instruction::SWAP(i) => self.stack.swap_with_top(i),
                Instruction::LOG(i) => {}
                Instruction::CREAT => {}
                Instruction::CALL => {}
                Instruction::CALLCODE => {}
                Instruction::RETURN => {}
                Instruction::DELEGATCALL => {}
                Instruction::CREAT2 => {}
                Instruction::STATICCALL => {}
                Instruction::REVERT => {}
                Instruction::SELFDESTRUCT => {}
                Instruction::INVALID => panic!("Invalid instruction"),
            }
        }
    }
}
