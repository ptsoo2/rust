use std::collections::VecDeque;

use crate::general_lock::{ILockable, MutexDefault, NullMutex, SpinMutexDefault};

pub struct ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    // write_queue_: *mut VecDeque<TElem>,
    // read_queue_: *mut VecDeque<TElem>,
    write_queue_offset_: usize,
    read_queue_offset_: usize,
    lst_queue_: [VecDeque<TElem>; 2],

    #[allow(unused)]
    lock_: TLock,
}

// unsafe impl<TElem, TLock> Send for ThreadJobQueueBase<TElem, TLock> where TLock: ILockable {}
unsafe impl<TElem, TLock> Sync for ThreadJobQueueBase<TElem, TLock> where TLock: ILockable {}

impl<TElem, TLock> Default for ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable,
{
    fn default() -> Self {
        Self {
            write_queue_offset_: 0,
            read_queue_offset_: 1,
            lst_queue_: [VecDeque::new(), VecDeque::new()],
            lock_: ILockable::new(),
        }
    }
}

impl<TElem, TLock> ThreadJobQueueBase<TElem, TLock>
where
    TLock: ILockable + 'static,
{
    pub fn push(&mut self, val: TElem) {
        self.lock_.critical_process(|| {
            self.lst_queue_[self.write_queue_offset_].push_back(val);
        });
    }

    pub fn swap_conditional(&mut self) {
        self.lock_.critical_process(|| {
            // read_queue => empty && write_queue => not empty
            if (self.lst_queue_[self.read_queue_offset_].is_empty() == true)
                && (self.lst_queue_[self.write_queue_offset_].is_empty() == false)
            {
                std::mem::swap(&mut self.write_queue_offset_, &mut self.read_queue_offset_);
            }
        });
    }

    pub fn swap_must(&mut self) {
        self.lock_.critical_process(|| {
            std::mem::swap(&mut self.write_queue_offset_, &mut self.read_queue_offset_);
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
        &mut self.lst_queue_[self.read_queue_offset_]
    }
}

/////////////////////////////////////////////////////////////////////////////////////
pub type ThreadJobQueueNull<TElem> = ThreadJobQueueBase<TElem, NullMutex>;
pub type ThreadJobQueueMutex<TElem> = ThreadJobQueueBase<TElem, MutexDefault>;
pub type ThreadJobQueueSpin<TElem> = ThreadJobQueueBase<TElem, SpinMutexDefault>;
