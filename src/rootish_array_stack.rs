use super::array_stack::*;
#[derive(Debug)]
pub struct RootishArrayStack {
    pub blocks: ArrayStack,
    pub n: usize,
}
