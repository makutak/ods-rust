#[cfg(test)]
mod new_method_tests {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn front_back_are_empty() {
        let daq = DualArrayDequeue::new();
        assert!(daq.front.is_empty());
        assert!(daq.back.is_empty());
    }
}
