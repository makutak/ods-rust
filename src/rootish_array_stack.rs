use super::array_stack::*;
use math::round;

#[derive(Debug)]
pub struct RootishArrayStack {
    pub blocks: ArrayStack,
    pub n: usize,
}

impl RootishArrayStack {
    pub fn new() -> Self {
        let ary = ArrayStack::new();
        let blocks = ary;
        let n = 0;
        Self {
            blocks: blocks,
            n: n,
        }
    }

    pub fn i2b(i: usize) -> i64 {
        round::ceil((-3.0 + ((9 + 8 * i) as f64).sqrt()) / 2.0, 0) as i64
    }

    pub fn get(&mut self, i: usize) -> u32 {
        let b = Self::i2b(i) as usize;
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b)
    }
}
