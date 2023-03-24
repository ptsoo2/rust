use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use chrono::DateTime;
use ex_common::bench::bench_multiple;
use ex_util::{
    general_lock::{ILockable, MutexDefault, SpinMutexDefault},
    shared_raw_ptr::SharedMutPtr,
    stop_handle::StopHandle,
    thread_job_queue::{ThreadJobQueueBase, ThreadJobQueueNull, ThreadJobQueueSpin},
};
use libc::{c_uint, srand};

#[allow(unused)]
pub(crate) fn test_stop_handle(thread_count: usize, with_sec: u64) {
    let mut stop_handle = StopHandle::new();
    let mut vec_handle = Vec::with_capacity(thread_count);
    for idx in 0..thread_count {
        let stop_token = stop_handle.get_token();
        let handle = thread::spawn(move || {
            println!("[{}] thread spawn...", idx);
            while !stop_token.is_stop() {
                std::thread::sleep(Duration::from_millis(1));
            }
            println!("[{}] thread exit...", idx);
        });
        vec_handle.push(handle);
    }

    std::thread::sleep(Duration::from_secs(with_sec));
    stop_handle.stop();
    for handle in vec_handle.into_iter() {
        handle.join().unwrap();
    }
    println!("all thread exit...");
}

#[allow(unused)]
fn test_thread_job_queue_st() {
    let mut thread_job_queue = ThreadJobQueueNull::<i32>::default();

    // publish
    {
        let mut a = 0;
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
        a += 1;
        thread_job_queue.push(a);
    }

    // consume
    {
        thread_job_queue.consume_all(|element| {
            println!("{}", element);
        });
    }
}

#[allow(unused)]
pub(crate) fn test_thread_job_queue_mt(publish_thread_count: usize) {
    let mut thread_job_queue: ThreadJobQueueSpin<String> = ThreadJobQueueSpin::default();

    let mut vec_handle = Vec::with_capacity(publish_thread_count);

    // publisher
    for idx in 0..publish_thread_count {
        let wrapper = SharedMutPtr::new(&mut thread_job_queue);

        unsafe {
            let thread_process = move || {
                println!("[{}]spawn publisher", idx);
                let wrapper = wrapper;
                let queue = wrapper.value_.as_mut().unwrap();

                {
                    let a = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("!!")
                        .as_millis();
                    srand(a as c_uint);
                }

                loop {
                    // random exit
                    let wait_seconds = (libc::rand() % 5) as u64;
                    if wait_seconds == 4 {
                        println!("[{}]exit publisher", idx);
                        queue.push("-1".to_owned());
                        break;
                    }

                    let system_time: DateTime<chrono::Utc> = SystemTime::now().into();
                    let value = system_time.format("%Y/%m/%dT%T").to_string();
                    queue.push(value.clone());
                    println!("[{}]publish({})", idx, value);
                    thread::sleep(Duration::from_secs(wait_seconds));
                }
            };

            vec_handle.push(thread::spawn(thread_process));
        }
    }

    // consumer
    let wrapper = SharedMutPtr::new(&mut thread_job_queue);

    unsafe {
        let thread_process = move || {
            let wrapper = wrapper;
            let queue = wrapper.value_.as_mut().unwrap();

            let mut exit_count = 0;
            let mut is_stop = false;
            while !is_stop {
                queue.consume_all(|elem| {
                    if elem.eq(&"-1".to_owned()) {
                        exit_count += 1;
                        if exit_count == publish_thread_count {
                            is_stop = true;
                        }
                    }
                    println!("consume({})", elem);
                });
            }
        };
        vec_handle.push(thread::spawn(thread_process));
    }

    for handle in vec_handle.into_iter() {
        handle.join().unwrap();
    }
    println!("all thread exit...");
}

#[allow(unused)]
pub(crate) fn test_thread_job_queue_custom_lock<TLock>(
    publish_thread_count: usize,
    mut publish_count: usize,
) where
    TLock: ILockable + 'static,
{
    let mut thread_job_queue: ThreadJobQueueBase<String, TLock> = ThreadJobQueueBase::default();
    let mut vec_handle = Vec::with_capacity(publish_thread_count);

    // publisher
    for idx in 0..publish_thread_count {
        let wrapper = SharedMutPtr::new(&mut thread_job_queue);

        unsafe {
            let thread_process = move || {
                let wrapper = wrapper;
                let queue = wrapper.value_.as_mut().unwrap();

                while publish_count > 0 {
                    queue.push("12312312312312123".to_owned());
                    publish_count -= 1;
                }
            };

            vec_handle.push(thread::spawn(thread_process));
        }
    }

    // consumer
    let wrapper = SharedMutPtr::new(&mut thread_job_queue);

    unsafe {
        let thread_process = move || {
            let wrapper = wrapper;
            let queue = wrapper.value_.as_mut().unwrap();

            let mut remain_consume_count = publish_thread_count * publish_count;
            let mut is_stop = false;
            while remain_consume_count > 0 {
                queue.consume_all(|elem| {
                    remain_consume_count -= 1;
                });
            }
        };
        vec_handle.push(thread::spawn(thread_process));
    }

    for handle in vec_handle.into_iter() {
        handle.join().unwrap();
    }
}

#[allow(unused)]
pub(crate) fn test_thread_job_queue_performance(
    publish_thread_count: usize,
    publish_count: usize,
    loop_count: u32,
) {
    for _ in 0..10 {
        bench_multiple("spin_mutex", loop_count, || {
            test_thread_job_queue_custom_lock::<SpinMutexDefault>(
                publish_thread_count,
                publish_count,
            );
        });
    }
    for _ in 0..10 {
        bench_multiple("mutex", loop_count, || {
            test_thread_job_queue_custom_lock::<MutexDefault>(publish_thread_count, publish_count);
        });
    }
}
