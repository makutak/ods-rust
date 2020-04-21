use super::array_stack::*;
#[derive(Debug)]
pub struct RootishArrayStack {
    pub blocks: ArrayStack,
    pub n: usize,
}

impl RootishArrayStack {
    pub fn new() -> Self {
        let blocks = ArrayStack::new();
        let n = 0;
        Self {
            blocks: blocks,
            n: n,
        }
    }

    pub fn i2b(&mut self, i: usize) {}

    pub fn get(&mut self, i: usize) {}
}
