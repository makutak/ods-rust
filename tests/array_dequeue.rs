#[cfg(test)]
mod new_method_tests {
    use ods_rust::array_dequeue::*;
    #[test]
    fn a_is_empty_vector() {
        let aq = ArrayDequeue::new();
        assert!(aq.a.is_empty());
    }

    #[test]
    fn j_is_zero() {
        let aq = ArrayDequeue::new();
        assert_eq!(aq.j, 0);
    }

    #[test]
    fn n_is_zero() {
        let aq = ArrayDequeue::new();
        assert_eq!(aq.n, 0);
    }
}
