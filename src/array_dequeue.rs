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

    pub fn add(&mut self, x: u32) {}
}
