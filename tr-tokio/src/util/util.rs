use std::time::Duration;


pub fn cal_bench<T>(f:T) -> Duration
where T: FnOnce()
{
    let start = std::time::Instant::now();
    f();
    let duration = start.elapsed();
    println!("Execution time: {duration:?}");
    duration
} 