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
        assert_eq!(aq.a, [1]);
        assert_eq!(aq.n, 1);
    }

    #[test]
    fn value_is_added_to_the_end_of_array() {
        let mut aq = ArrayQueue::new();
        aq.add(1);
        aq.add(2);
        aq.add(3);
        assert_eq!(aq.a, [1, 2, 3, 0]);
        assert_eq!(aq.n, 3);
    }
}

#[cfg(test)]
mod remove_method_tests {
    use ods_rust::array_queue::*;

    #[test]
    fn can_remove_value() {
        let mut aq = ArrayQueue::new();
        aq.add(1);
        assert_eq!(aq.a, [1]);
        assert_eq!(aq.n, 1);

        let a = aq.remove();
        assert_eq!(a, 1);
        assert_eq!(aq.a, [0]);
        assert_eq!(aq.n, 0);
    }

    #[test]
    fn can_remove_first_value_of_array() {
        let mut aq = ArrayQueue::new();
        aq.add(1);
        aq.add(2);
        aq.add(3);
        aq.add(4);
        assert_eq!(aq.a, vec![1, 2, 3, 4]);
        assert_eq!(aq.n, 4);

        let a = aq.remove();
        assert_eq!(a, 1);
        assert_eq!(aq.n, 3);

        let b = aq.remove();
        assert_eq!(b, 2);
        assert_eq!(aq.n, 2);

        let c = aq.remove();
        assert_eq!(c, 3);
        assert_eq!(aq.n, 1);

        let d = aq.remove();
        assert_eq!(d, 4);
        assert_eq!(aq.n, 0);
    }

    #[test]
    #[should_panic]
    fn if_queue_size_is_zero_raise_no_such_error() {
        let mut aq = ArrayQueue::new();
        aq.remove();
    }
}
