#[cfg(test)]
mod new_method_tests {
    use ods_rust::array_queue::*;
    #[test]
    fn a_is_empty_vector() {
        let aq = ArrayQueue::new();
        assert!(aq.a.is_empty());
    }

    #[test]
    fn j_is_zero() {
        let aq = ArrayQueue::new();
        assert_eq!(aq.j, 0);
    }

    #[test]
    fn n_is_zero() {
        let aq = ArrayQueue::new();
        assert_eq!(aq.n, 0);
    }
}

#[cfg(test)]
mod add_method_tests {
    use ods_rust::array_queue::*;
    #[test]
    fn can_add_value() {
        let mut aq = ArrayQueue::new();
        aq.add(1);
        let expected = vec![1];
        assert_eq!(aq.a, expected);
    }

    #[test]
    fn value_is_added_to_the_end_of_array() {
        let mut aq = ArrayQueue::new();
        aq.add(1);
        aq.add(2);
        aq.add(3);

        let expected = vec![1, 2, 3, 0];
        assert_eq!(aq.a, expected);
    }
}
