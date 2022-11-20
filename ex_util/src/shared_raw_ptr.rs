pub struct TSharedMutPtr<T> {
    #[allow(unused)]
    pub value_: *mut T,
}

unsafe impl<T> Send for TSharedMutPtr<T> {}

impl<T> TSharedMutPtr<T> {
    #[allow(unused)]
    pub fn new(value: *mut T) -> Self {
        Self { value_: value }
    }
}
