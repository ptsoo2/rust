use crate::log;
use std::time::Instant;

pub fn bench_multiple<F>(name: &str, count: u32, mut iter: F)
where
    F: FnMut(),
{
    let start = Instant::now();

    for _ in 0..count {
        iter();
    }

    let duration = start.elapsed();

    log!("[{}] count: {}, duration: {:?}", name, count, duration);
}
