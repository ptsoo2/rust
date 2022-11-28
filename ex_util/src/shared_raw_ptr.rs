pub struct SharedMutPtr<T> {
    #[allow(unused)]
    pub value_: *mut T,
}

unsafe impl<T> Send for SharedMutPtr<T> {}
unsafe impl<T> Sync for SharedMutPtr<T> {}

impl<T> SharedMutPtr<T> {
    #[allow(unused)]
    pub fn new(value: *mut T) -> Self {
        Self { value_: value }
    }
}
