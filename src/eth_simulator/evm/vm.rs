use core::panic;

use super::{
    super::{
        eth_types::{Address, Bytes, Code, EthFrom, EthSign, U256},
        hash,
    },
    ext::{Ext, ExtError},
    instructions::Instruction,
    memory::Memory,
    pc::PC,
    stack::Stack,
};

pub enum VMResult {
    Ok,
    Stop,
    Return(Bytes),
}

#[derive(Debug)]
pub enum VMErrorKind {
    NotExistedAddress(Address),
    NotExistedStorageKey,
}

#[derive(Debug)]
pub struct VMError {
    pub instruction: Instruction,
    pub pc: usize,
    pub error_kind: VMErrorKind,
}

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

    pub fn execute(&mut self, ext: &mut Ext) -> Result<VMResult, VMError> {
        while let Some(instruction) = self.pc.next() {
            match instruction {
                Instruction::STOP => return Ok(VMResult::Stop),
                Instruction::ADD => self.stack.two_items_op(|a, b| a + b),
                Instruction::MUL => self.stack.two_items_op(|a, b| a * b),
                Instruction::SUB => self.stack.two_items_op(|a, b| a - b),
                Instruction::DIV => self.stack.two_items_op(|a, b| a / b),
                Instruction::SDIV => self.stack.two_items_op(|a, b| a.to_sign() / b.to_sign()),
                Instruction::MOD => self.stack.two_items_op(|a, b| a % b),
                Instruction::SMOD => self.stack.two_items_op(|a, b| a.to_sign() % b.to_sign()),
                Instruction::ADDMOD => self.stack.three_items_op(|a, b, n| (a + b) % n),
                Instruction::MULMOD => self.stack.three_items_op(|a, b, n| (a * b) % n),
                Instruction::EXP => self.stack.two_items_op(U256::pow),
                Instruction::SIGNEXTEND => {
                    let bit_num = self.stack.pop();
                    let number = self.stack.pop();
                    let bit_position = bit_num.as_usize() * 8 + 7;
                    let bit = number.bit(bit_position);
                    let mask = (U256::one() << bit_position) - U256::one();
                    self.stack
                        .push(if bit { number | !mask } else { number | mask });
                }
                Instruction::LT => self.stack.two_items_op(|a, b| U256::ethfrom(a < b)),
                Instruction::SLT => {
                    let a = self.stack.pop().to_sign();
                    let neg_a = a.is_neg();
                    let b = self.stack.pop();
                    let neg_b = b.is_neg();

                    let is_positive_lt = a < b && !(neg_a | neg_b);
                    let is_negative_lt = a > b && (neg_a & neg_b);
                    let has_different_signs = neg_a && !neg_b;

                    self.stack.push(U256::ethfrom(
                        is_positive_lt | is_negative_lt | has_different_signs,
                    ));
                }
                Instruction::GT => self.stack.two_items_op(|a, b| U256::ethfrom(a > b)),
                Instruction::SGT => {
                    let a = self.stack.pop().to_sign();
                    let neg_a = a.is_neg();
                    let b = self.stack.pop().to_sign();
                    let neg_b = b.is_neg();

                    let is_positive_gt = a > b && !(neg_a | neg_b);
                    let is_negative_gt = a < b && (neg_a & neg_b);
                    let has_different_signs = !neg_a && neg_b;

                    self.stack.push(U256::ethfrom(
                        is_positive_gt | is_negative_gt | has_different_signs,
                    ));
                }
                Instruction::EQ => self.stack.two_items_op(|a, b| U256::ethfrom(a == b)),
                Instruction::ISZERO => self.stack.one_item_op(|a| U256::ethfrom(a.is_zero())),
                Instruction::AND => self.stack.two_items_op(|a, b| a & b),
                Instruction::OR => self.stack.two_items_op(|a, b| a | b),
                Instruction::XOR => self.stack.two_items_op(|a, b| a ^ b),
                Instruction::NOT => self.stack.one_item_op(|a| !a),
                Instruction::BYTE => self
                    .stack
                    .two_items_op(|i, x| U256::from(x.byte(i.as_usize()))),
                Instruction::SHL => self.stack.two_items_op(|shl, value| value << shl),
                Instruction::SHR => self.stack.two_items_op(|shr, value| value >> shr),
                Instruction::SAR => {
                    const CONST_256: U256 = U256([256, 0, 0, 0]);
                    const CONST_HIBIT: U256 = U256([0, 0, 0, 0x8000000000000000]);

                    let shift = self.stack.pop();
                    let value = self.stack.pop();
                    let sign = value & CONST_HIBIT != U256::zero();

                    let result = if shift >= CONST_256 {
                        if sign {
                            U256::max_value()
                        } else {
                            U256::zero()
                        }
                    } else {
                        let shift = shift.as_u32() as usize;
                        let mut shifted = value >> shift;
                        if sign {
                            shifted = shifted | (U256::max_value() << (256 - shift));
                        }
                        shifted
                    };
                    self.stack.push(result);
                }
                Instruction::SHA3 => self.stack.two_items_op(|offset, length| {
                    U256::ethfrom(hash::keccak(self.memory.read_slice(offset, length)))
                }),
                Instruction::ADDRESS => self.stack.push(ext.get_address()),
                Instruction::BALANCE => {
                    let address = Address::ethfrom(self.stack.pop());
                    match ext.get_balance(&address) {
                        Ok(balance) => self.stack.push(balance),
                        Err(err) => match err {
                            ExtError::NotExistedAddress(address) => {
                                return Err(VMError {
                                    instruction: Instruction::BALANCE,
                                    pc: self.pc.pc().as_usize(),
                                    error_kind: VMErrorKind::NotExistedAddress(address),
                                })
                            }
                            _ => {}
                        },
                    }
                }
                Instruction::ORIGIN => self.stack.push(ext.get_origin()),
                Instruction::CALLER => self.stack.push(ext.get_caller()),
                Instruction::CALLVALUE => self.stack.push(ext.get_callvalue()),
                Instruction::CALLDATALOAD => self.stack.one_item_op(|i| ext.get_calldata(i)),
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
                Instruction::GASPRICE => self.stack.push(ext.get_gasprice()),
                Instruction::EXTCODESIZE => {
                    let address = Address::ethfrom(self.stack.pop());

                    match ext.get_ext_codesize(&address) {
                        Ok(value) => self.stack.push(value),
                        Err(err) => match err {
                            ExtError::NotExistedAddress(address) => {
                                return Err(VMError {
                                    instruction: Instruction::EXTCODESIZE,
                                    pc: self.pc.pc().as_usize(),
                                    error_kind: VMErrorKind::NotExistedAddress(address),
                                })
                            }
                            _ => {}
                        },
                    }
                }
                Instruction::EXTCODECOPY => {
                    let address = Address::ethfrom(self.stack.pop());
                    let dest_offset = self.stack.pop();
                    let offset = self.stack.pop();
                    let length = self.stack.pop();

                    match ext.get_ext_code_slice(&address, offset, length) {
                        Ok(value) => self.memory.write_slice(dest_offset, value),
                        Err(err) => match err {
                            ExtError::NotExistedAddress(address) => {
                                return Err(VMError {
                                    instruction: Instruction::EXTCODECOPY,
                                    pc: self.pc.pc().as_usize(),
                                    error_kind: VMErrorKind::NotExistedAddress(address),
                                })
                            }
                            _ => {}
                        },
                    }
                }
                Instruction::RETURNDATASIZE => {}
                Instruction::RETURNDATACOPY => {}
                Instruction::EXTCODEHASH => {
                    let address = Address::ethfrom(self.stack.pop());

                    match ext.get_ext_code_hash(&address) {
                        Ok(value) => self.stack.push(value),
                        Err(err) => match err {
                            ExtError::NotExistedAddress(address) => {
                                return Err(VMError {
                                    instruction: Instruction::EXTCODEHASH,
                                    pc: self.pc.pc().as_usize(),
                                    error_kind: VMErrorKind::NotExistedAddress(address),
                                })
                            }
                            _ => {}
                        },
                    }
                }
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
                Instruction::MLOAD => self.stack.one_item_op(|offset| self.memory.read(offset)),
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
                    let key = self.stack.pop();

                    match ext.get_storage(key) {
                        Ok(value) => self.stack.push(value),
                        Err(err) => match err {
                            ExtError::NotExistedStorageKey => {
                                return Err(VMError {
                                    instruction: Instruction::SLOAD,
                                    pc: self.pc.pc().as_usize(),
                                    error_kind: VMErrorKind::NotExistedStorageKey,
                                })
                            }
                            _ => {}
                        },
                    }
                }
                Instruction::SSTORE => {
                    let key = self.stack.pop();
                    let value = self.stack.pop();
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
                Instruction::RETURN => {
                    let offset = self.stack.pop();
                    let length = self.stack.pop();
                    return Ok(VMResult::Return(Bytes::from(
                        self.memory.read_slice(offset, length),
                    )));
                }
                Instruction::DELEGATCALL => {}
                Instruction::CREAT2 => {}
                Instruction::STATICCALL => {}
                Instruction::REVERT => {}
                Instruction::SELFDESTRUCT => {}
                Instruction::INVALID => panic!("Invalid instruction"),
            }
        }

        Ok(VMResult::Ok)
    }
}
