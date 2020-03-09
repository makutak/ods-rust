use std::cmp::max;

#[derive(Debug)]
pub struct ArrayDequeue {
    pub a: Vec<u32>,
    pub j: usize,
    pub n: usize,
}

impl ArrayDequeue {
    pub fn new() -> Self {
        let ary = Vec::new();
        Self { a: ary, j: 0, n: 0 }
    }

    pub fn get(&mut self, i: usize) -> u32 {
        self.a[(self.j + i) % self.a.len()]
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        let idx = (self.j) + i % self.a.len();
        let y = self.a[idx];
        self.a[idx] = x;
        y
    }

    pub fn add(&mut self, i: usize, x: u32) {
        if self.n == self.a.len() {
            self.resize()
        }

        let current_len = self.a.len();
        if i < self.n / 2 {
            self.j = (self.j - 1) % self.a.len();
            for k in 0..i {
                self.a[(self.j + k) % current_len] = self.a[(self.j + k - 1) % current_len]
            }
        } else {
            for k in (i..self.n).rev() {
                self.a[(self.j + k) % current_len] = self.a[(self.j + k - 1) % current_len];
            }
        }

        self.a[(self.j + i) % current_len] = x;
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> u32 {
        let current_len = self.a.len();
        let x = self.a[(self.j + i) % self.a.len()];

        if i < self.n / 2 {
            for k in (0..i).rev() {
                self.a[(self.j + k) % current_len] = self.a[(self.j + k - 1) % current_len];
            }
            self.j = (self.j + 1) % current_len;
        } else {
            for k in i..(self.n - 1) {
                self.a[(self.j + k) % current_len] = self.a[(self.j + k + 1) % current_len]
            }
        }

        self.n -= 1;
        if current_len >= 3 * self.n {
            self.resize();
        }

        x
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
