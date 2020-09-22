//use super::array_stack::*;
use math::round;

#[derive(Debug)]
pub struct RootishArrayStack {
    pub blocks: Vec<Vec<u32>>,
    pub n: usize,
}

impl RootishArrayStack {
    pub fn new() -> Self {
        let blocks = vec![];
        let n = 0;
        Self {
            blocks: blocks,
            n: n,
        }
    }

    fn grow(&mut self) {
        self.blocks.push(vec![0; self.blocks.len() + 1]);
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.len();
        while r > 0 && (r - 2) * (r - 1) / 2 >= self.n {
            self.blocks.remove(self.blocks.len() - 1);
            r -= 1;
        }
    }

    pub fn i2b(i: usize) -> i64 {
        round::ceil((-3.0 + ((9 + 8 * i) as f64).sqrt()) / 2.0, 0) as i64
    }

    pub fn get(&mut self, i: usize) -> u32 {
        let b = Self::i2b(i) as usize;
        let j = i - b * (b + 1) / 2;
        self.blocks[b][j]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let b = Self::i2b(i) as usize;
        let j = i - b * (b + 1) / 2;
        let y = self.blocks[b][j];
        self.blocks[b][j] = x;
        y
    }

    pub fn add(&mut self, i: usize, x: u32) {
        let r = self.blocks.len();

        if r * (r + 1) / 2 < self.n + 1 {
            self.grow();
        }
        self.n += 1;
        for j in (i..self.n - 1).rev() {
            let y = self.get(j - 1);
            self.set(i, y);
        }
        self.set(i, x);
    }

    pub fn remove() {}

    pub fn size(&mut self) -> usize {
        self.n
    }

    pub fn clear() {}
}
