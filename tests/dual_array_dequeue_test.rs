#[cfg(test)]
mod new_method_tests {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn front_and_back_are_empty() {
        let daq = DualArrayDequeue::new();
        assert_eq!(daq.front.size(), 0);
        assert_eq!(daq.back.size(), 0);
    }
}

mod size_method_tests {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn size_is_sum_of_front_and_back() {
        let mut daq = DualArrayDequeue::new();
        assert_eq!(daq.size(), 0);
    }
}
