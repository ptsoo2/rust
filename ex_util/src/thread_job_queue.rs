use std::{collections::VecDeque, ptr::null_mut, sync::Mutex};

pub struct ThreadJobQueue<T> {
    write_queue_: *mut VecDeque<T>,
    read_queue_: *mut VecDeque<T>,
    lst_queue_: [VecDeque<T>; 2],
    lock_: Mutex<i32>, // todo! spin_lock
}

unsafe impl<T> Send for ThreadJobQueue<T> {}
unsafe impl<T> Sync for ThreadJobQueue<T> {}

impl<T> Default for ThreadJobQueue<T> {
    fn default() -> Self {
        let mut ret_self = Self {
            write_queue_: null_mut(),
            read_queue_: null_mut(),
            lst_queue_: [VecDeque::new(), VecDeque::new()],
            lock_: Mutex::default(),
        };

        ret_self.write_queue_ = &mut ret_self.lst_queue_[0];
        ret_self.read_queue_ = &mut ret_self.lst_queue_[1];
        ret_self
    }
}

impl<T> ThreadJobQueue<T> {
    pub fn push(&mut self, val: T) {
        if let Ok(_) = self.lock_.lock() {
            unsafe {
                (*self.write_queue_).push_back(val);
            }
        }
    }

    pub fn swap(&mut self) {
        if let Ok(_) = self.lock_.lock() {
            std::mem::swap(&mut self.write_queue_, &mut self.read_queue_);
            // let temp_write_queue = self.write_queue_;
            // self.write_queue_ = self.read_queue_;
            // self.read_queue_ = temp_write_queue;
        }
    }

    pub fn consume_all<F>(&mut self, mut iter: F)
    where
        F: FnMut(T) -> (),
    {
        self.swap();
        unsafe {
            let read_queue = self.get_read_queue().as_mut().unwrap();
            while read_queue.is_empty() == false {
                iter(read_queue.pop_front().unwrap());
            }
        }
    }

    #[allow(unused)]
    fn trace(&self) {
        unsafe {
            ex_common::log!(
                "write: {:?}({})",
                self.write_queue_,
                (*self.write_queue_).len()
            );
            ex_common::log!(
                "read: {:?}({})",
                self.read_queue_,
                (*self.read_queue_).len()
            );
        }
    }

    pub fn get_read_queue(&mut self) -> *mut VecDeque<T> {
        self.read_queue_
    }
}
