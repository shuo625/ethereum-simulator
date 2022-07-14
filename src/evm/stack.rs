use crate::eth_types::U256;

pub struct Stack {
    stack: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, v: U256) {
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> U256 {
        self.stack.pop().unwrap()
    }
}
