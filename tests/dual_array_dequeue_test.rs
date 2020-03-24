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

mod get_method_test {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn get_selecteed_value() {
        let mut daq = DualArrayDequeue::new();
        daq.add(0, 1);
        assert_eq!(daq.get(0), 1);
    }
}

mod set_method_test {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn set_selected_value() {
        let mut daq = DualArrayDequeue::new();
        daq.add(0, 777);
        assert_eq!(daq.get(0), 777);
        daq.set(0, 1);
        assert_eq!(daq.get(0), 1);
    }
}

mod add_method_test {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn add_selected_value() {
        let mut daq = DualArrayDequeue::new();
        daq.add(0, 1);
        assert_eq!(daq.size(), 1);
    }
}

mod remove_method_test {
    use ods_rust::dual_array_dequeue::*;
    #[test]
    fn remove_selected_value() {
        let mut daq = DualArrayDequeue::new();
        daq.add(0, 1);
        assert_eq!(daq.size(), 1);
        let ret = daq.remove(0);
        assert_eq!(ret, 1);
    }
}
