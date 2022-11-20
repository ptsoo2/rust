use std::{collections::VecDeque, ptr::null_mut};

use crate::general_lock::{ILockable, MutexDefault, NullMutex, SpinMutexDefault};

pub struct ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    write_queue_: *mut VecDeque<TElem>,
    read_queue_: *mut VecDeque<TElem>,
    lst_queue_: [VecDeque<TElem>; 2],

    #[allow(unused)]
    lock_: TLock,
}

unsafe impl<TElem, TLock> Send for ThreadJobQueueBase<TElem, TLock> where TLock: ILockable {}
unsafe impl<TElem, TLock> Sync for ThreadJobQueueBase<TElem, TLock> where TLock: ILockable {}

impl<TElem, TLock> Default for ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    fn default() -> Self {
        let mut ret_self = Self {
            write_queue_: null_mut(),
            read_queue_: null_mut(),
            lst_queue_: [VecDeque::new(), VecDeque::new()],
            lock_: ILockable::new(),
        };

        ret_self.write_queue_ = &mut ret_self.lst_queue_[0];
        ret_self.read_queue_ = &mut ret_self.lst_queue_[1];
        ret_self
    }
}

impl<TElem, TLock> ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable + 'static,
{
    pub fn push(&mut self, val: TElem) {
        unsafe {
            let closure = || {
                (*self.write_queue_).push_back(val);
            };
            self.lock_.critical_process(closure);
        }
    }

    pub fn swap(&mut self) {
        self.lock_.critical_process(|| {
            std::mem::swap(&mut self.write_queue_, &mut self.read_queue_);
        });
    }

    pub fn consume_all<F>(&mut self, mut iter: F)
    where
        F: FnMut(TElem) -> (),
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

    #[allow(unused)]
    pub fn get_read_queue(&mut self) -> *mut VecDeque<TElem> {
        self.read_queue_
    }
}

/////////////////////////////////////////////////////////////////////////////////////
pub type ThreadJobQueueNull<TElem> = ThreadJobQueueBase<TElem, NullMutex>;
pub type ThreadJobQueueMutex<TElem> = ThreadJobQueueBase<TElem, MutexDefault>;
pub type ThreadJobQueueSpin<TElem> = ThreadJobQueueBase<TElem, SpinMutexDefault>;
