#[derive(Debug)]
pub struct DualArrayDequeue {
    pub front: Vec<u32>,
    pub back: Vec<u32>,
}

impl DualArrayDequeue {
    pub fn new() -> Self {
        let front = Vec::new();
        let back = Vec::new();
        Self {
            front: front,
            back: back,
        }
    }
}
