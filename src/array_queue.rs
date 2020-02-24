use std::cmp::max;

#[derive(Debug)]
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

    pub fn remove(&mut self) -> u32 {
        match self.n {
            n if n == 0 => panic!("No such Element!!"),
            _ => {
                let x = self.a[self.j];
                self.j = (self.j + 1) % self.a.len();
                self.n -= 1;

                if self.a.len() >= (3 * self.n) {
                    self.resize();
                }

                x
            }
        }
    }

    pub fn resize(&mut self) {
        let mut b = vec![0; max(1, self.n * 2)];
        for k in 0..self.n {
            b[k] = self.a[(self.j + k) % self.a.len()];
        }
        self.a = b;
        self.j = 0;
    }
}
