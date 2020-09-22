#[cfg(test)]
mod new_method_tests {
    use ods_rust::rootish_array_stack::*;
    #[test]
    fn blocks_is_empty() {
        let ras = RootishArrayStack::new();
        assert_eq!(ras.blocks.len(), 0);
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

mod add_tests {
    use ods_rust::rootish_array_stack::*;
    #[test]
    fn test_add() {
        let mut ras = RootishArrayStack::new();
        ras.add(0, 1);
        assert_eq!(ras.size(), 1);
        ras.add(1, 2);
        assert_eq!(ras.size(), 2);
    }
}

mod get_test {
    use ods_rust::rootish_array_stack::*;
    #[test]
    fn test_get_1() {
        let mut ras = RootishArrayStack::new();
        ras.add(0, 1);
        assert_eq!(ras.get(0), 1);
    }

    #[test]
    fn test_get_2() {
        let mut ras = RootishArrayStack::new();
        ras.add(0, 0);
        ras.add(1, 1);
        ras.add(2, 2);
        assert_eq!(ras.get(1), 1);
    }

    #[test]
    fn test_get_3() {
        let mut ras = RootishArrayStack::new();
        ras.add(0, 0);
        ras.add(1, 1);
        ras.add(2, 2);
        assert_eq!(ras.get(2), 2);
    }
}
