pub struct ArrayQueue {
    pub a: Vec<u32>,
    pub j: usize,
    pub n: usize,
}

impl ArrayQueue {
    pub fn new() -> ArrayQueue {
        let ary = Vec::new();
        ArrayQueue { a: ary, j: 0, n: 0 }
    }
}
