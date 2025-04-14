pub trait GetResult<T> {
    fn get_result(&self, input: &str) -> T;
}

pub trait RemoveElementIfMaxValue {
    fn remove_element_if_max_value(&mut self, max_value: usize);
}

pub trait GetElementByName<'a, T> {
    fn get_element_by_name(&'a mut self, name: &str) -> T;
}
