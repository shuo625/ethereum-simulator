use crate::eth_types::U256;

pub struct Stack {
    stack: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn size(&self) -> U256 {
        U256::from(self.stack.len())
    }

    pub fn push(&mut self, v: U256) {
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> U256 {
        self.stack.pop().unwrap()
    }

    pub fn dup_top(&mut self, i: usize) {
        let idx = self.stack.len() - i;
        let value = self.stack[idx].clone();
        self.stack.push(value);
    }

    pub fn swap_with_top(&mut self, i: usize) {
        let last_value = self.stack.pop().unwrap();
        let idx = self.stack.len() - i;
        let tmp = self.stack[idx].clone();
        self.stack[idx] = last_value;
        self.stack.push(tmp);
    }

    pub fn one_item_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256) -> U256,
    {
        let a = self.stack.pop().unwrap();
        self.stack.push(op(a));
    }

    pub fn two_items_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256, U256) -> U256,
    {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(op(a, b));
    }

    pub fn three_items_op<F>(&mut self, op: F)
    where
        F: FnOnce(U256, U256, U256) -> U256,
    {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let c = self.stack.pop().unwrap();
        self.stack.push(op(a, b, c));
    }
}
