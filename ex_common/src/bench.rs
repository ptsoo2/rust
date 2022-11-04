use crate::{function, log};
use chrono::Local;
use std::time::Instant;

pub fn bench_multiple<F>(name: &str, count: u32, mut inner: F)
where
    F: FnMut(),
{
    let start = Instant::now();

    for _ in 0..count {
        inner();
    }

    let duration = start.elapsed();

    log!("[{}] count: {}, duration: {:?}", name, count, duration);
}
