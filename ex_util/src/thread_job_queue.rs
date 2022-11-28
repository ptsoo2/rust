use std::{collections::VecDeque, ptr::null_mut};

use crate::{
    general_lock::{ILockable, MutexDefault, NullMutex, SpinMutexDefault},
    shared_raw_ptr::SharedMutPtr,
};

pub struct ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    write_queue_: SharedMutPtr<VecDeque<TElem>>,
    read_queue_: SharedMutPtr<VecDeque<TElem>>,
    lst_queue_: Box<[VecDeque<TElem>; 2]>,

    #[allow(unused)]
    lock_: TLock,
}

unsafe impl<TElem, TLock> Send for ThreadJobQueueBase<TElem, TLock> where TLock: ILockable {}

impl<TElem, TLock> Default for ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    fn default() -> Self {
        let mut ret_self = Self {
            write_queue_: SharedMutPtr { value_: null_mut() },
            read_queue_: SharedMutPtr { value_: null_mut() },
            lst_queue_: Box::new([VecDeque::new(), VecDeque::new()]),
            lock_: ILockable::new(),
        };

        ret_self.write_queue_.value_ = &mut ret_self.lst_queue_[0];
        ret_self.read_queue_.value_ = &mut ret_self.lst_queue_[1];
        ret_self
    }
}

impl<TElem, TLock> ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable + 'static,
{
    pub fn push(&mut self, val: TElem) {
        self.lock_.critical_process(|| unsafe {
            (*self.write_queue_.value_).push_back(val);
        });
    }

    pub fn swap_conditional(&mut self) {
        self.lock_.critical_process(|| {
            // read_queue => empty && write_queue => not empty
            unsafe {
                if ((*self.read_queue_.value_).is_empty() == true)
                    && ((*self.write_queue_.value_).is_empty() == false)
                {
                    std::mem::swap(&mut self.write_queue_, &mut self.read_queue_);
                }
            }
        });
    }

    pub fn swap_must(&mut self) {
        self.lock_.critical_process(|| {
            std::mem::swap(&mut self.write_queue_, &mut self.read_queue_);
        });
    }

    pub fn consume_all<F>(&mut self, mut iter: F)
    where
        F: FnMut(TElem) -> (),
    {
        self.swap_must();
        let read_queue = self.get_read_queue();
        while read_queue.is_empty() == false {
            iter(read_queue.pop_front().unwrap());
        }
    }

    #[allow(unused)]
    pub fn get_read_queue(&mut self) -> &mut VecDeque<TElem> {
        unsafe { &mut (*self.read_queue_.value_) }
    }
}

/////////////////////////////////////////////////////////////////////////////////////
pub type ThreadJobQueueNull<TElem> = ThreadJobQueueBase<TElem, NullMutex>;
pub type ThreadJobQueueMutex<TElem> = ThreadJobQueueBase<TElem, MutexDefault>;
pub type ThreadJobQueueSpin<TElem> = ThreadJobQueueBase<TElem, SpinMutexDefault>;
