use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct StopToken {
    stop_flag_: *const AtomicBool,
}

impl StopToken {
    pub fn is_stop(&self) -> bool {
        unsafe { (&*self.stop_flag_).load(Acquire) }
    }
}

unsafe impl Send for StopToken {}

pub struct StopHandle {
    stop_flag_: AtomicBool,
}

impl StopHandle {
    pub fn new() -> Self {
        Self {
            stop_flag_: AtomicBool::new(false),
        }
    }

    pub fn stop(&mut self) {
        self.stop_flag_.store(true, Release);
    }

    pub fn is_stop(&self) -> bool {
        self.stop_flag_.load(Acquire)
    }

    pub fn get_token(&mut self) -> StopToken {
        StopToken {
            stop_flag_: &self.stop_flag_,
        }
    }
}
