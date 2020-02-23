use std::cmp::max;

pub struct ArrayQueue {
    pub a: Vec<u32>,
    pub j: usize,
    pub n: usize,
}

impl ArrayQueue {
    pub fn new() -> Self {
        let ary = Vec::new();
        Self { a: ary, j: 0, n: 0 }
    }

    pub fn add(&mut self, x: u32) -> bool {
        if self.n + 1 > self.a.len() {
            self.resize();
        }

        let current_len = self.a.len();
        self.a[(self.j + self.n) % current_len] = x;
        self.n += 1;
        true
    }

    pub fn resize(&mut self) {
        let current_len = self.a.len();
        let mut b = vec![0; max(1, 2 * self.n)];
        for k in 0..self.n {
            b[k] = self.a[(self.j + k) % current_len];
        }
        self.a = b;
        self.j = 0;
    }
}
