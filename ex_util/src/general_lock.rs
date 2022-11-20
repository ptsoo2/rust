use spin::mutex::SpinMutex;
use std::sync::Mutex;

// 아주 거지같이 써야하네...
/////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct NullMutex {}
pub type MutexDefault = Mutex<i32>;
pub type SpinMutexDefault = SpinMutex<i32>;
/////////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait ILockable {
    fn new() -> Self;
    fn critical_process<F: FnOnce()>(&self, f: F);
}

impl ILockable for NullMutex {
    fn new() -> Self {
        Self {}
    }
    fn critical_process<F: FnOnce()>(&self, f: F) {
        f();
    }
}

impl ILockable for MutexDefault {
    fn new() -> Self {
        Mutex::default()
    }
    fn critical_process<F: FnOnce()>(&self, f: F) {
        let a = self.lock().ok().unwrap();
        f();
        drop(a);
    }
}

impl ILockable for SpinMutexDefault {
    fn new() -> Self {
        SpinMutex::default()
    }
    fn critical_process<F: FnOnce()>(&self, f: F) {
        let a = self.lock();
        f();
        drop(a);
    }
}
