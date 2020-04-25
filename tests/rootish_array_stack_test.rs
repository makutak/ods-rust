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

mod i2b_method_tests {
    use ods_rust::rootish_array_stack::*;
    #[test]
    fn test_i2b() {
        assert_eq!(RootishArrayStack::i2b(1), 1);
        assert_eq!(RootishArrayStack::i2b(2), 1);
        assert_eq!(RootishArrayStack::i2b(3), 2);
        assert_eq!(RootishArrayStack::i2b(4), 2);
        assert_eq!(RootishArrayStack::i2b(5), 2);
        assert_eq!(RootishArrayStack::i2b(6), 3);
    }
}
