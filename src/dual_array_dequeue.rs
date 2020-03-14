use super::array_stack::*;

pub struct DualArrayDequeue {
    pub front: ArrayStack,
    pub back: ArrayStack,
}

impl DualArrayDequeue {
    pub fn new() -> Self {
        let front = ArrayStack::new();
        let back = ArrayStack::new();
        Self {
            front: front,
            back: back,
        }
    }

    pub fn size(&mut self) -> usize {
        self.front.size() + self.back.size()
    }
}
