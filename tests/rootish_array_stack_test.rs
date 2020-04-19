#[cfg(test)]
mod new_method_tests {
    use ods_rust::rootish_array_stack::*;
    #[test]
    fn blocks_is_empty() {
        let ras = RootishArrayStack::new();
        assert_eq!(ras.blocks.size(), 0);
    }

    #[test]
    fn n_is_zero() {
        let ras = RootishArrayStack::new();
        assert_eq!(ras.n, 0);
    }
}
