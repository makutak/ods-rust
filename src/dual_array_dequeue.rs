use super::array_stack::*;
#[derive(Debug)]
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

    pub fn get(&mut self, i: usize) -> u32 {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    pub fn set(&mut self, i: usize, x: u32) -> u32 {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    pub fn add(&mut self, i: usize, x: u32) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, x);
        } else {
            self.back.add(i - self.front.size(), x);
        }

        self.balance();
    }

    pub fn remove(&mut self, i: usize) -> u32 {
        let x;
        if i < self.front.size() {
            x = self.front.remove(self.front.size() - i - 1);
        } else {
            x = self.back.remove(i - self.front.size());
        }
        //self.balance();
        x
    }

    pub fn balance(&mut self) {
        let n = self.size();
        let mid = n / 2;
        if 3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size() {
            let mut f = ArrayStack::new();
            for i in 0..mid {
                f.add(i, self.get(mid - i - 1));
            }
            let mut b = ArrayStack::new();
            for i in 0..n - mid {
                b.add(i, self.get(mid + i));
            }
            self.front = f;
            self.back = b;
        }
    }
}
