pub fn get_first_element<T>(vec: &Vec<T>) -> Option<&T> {
    vec.first()
}

pub fn get_first_from_slice<T>(slice: &[T]) -> Option<&T> {
    slice.first()
}
