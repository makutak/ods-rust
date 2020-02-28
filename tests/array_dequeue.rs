#[cfg(test)]
mod new_method_tests {
    use ods_rust::array_dequeue::*;
    #[test]
    fn a_is_empty_vector() {
        let ad = ArrayDequeue::new();
        assert!(ad.a.is_empty());
    }

    #[test]
    fn j_is_zero() {
        let ad = ArrayDequeue::new();
        assert_eq!(ad.j, 0);
    }

    #[test]
    fn n_is_zero() {
        let ad = ArrayDequeue::new();
        assert_eq!(ad.n, 0);
    }
}

#[cfg(test)]
mod get_method_tests {
    use ods_rust::array_dequeue::*;
    #[test]
    fn get_indexed_value() {
        let mut ad = ArrayDequeue::new();
        ad.add(1);
        ad.add(2);
        assert_eq!(ad.get(0), 1);
    }

    #[test]
    #[should_panic]
    fn get_out_of_range_indexed_value() {
        let mut ad = ArrayDequeue::new();
        ad.get(10);
    }
}
