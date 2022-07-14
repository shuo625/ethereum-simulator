use super::{memory::Memory, stack::Stack};

pub struct VM {
    stack: Stack,
    memory: Memory,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Stack::new(),
            memory: Memory::new(),
        }
    }
}
