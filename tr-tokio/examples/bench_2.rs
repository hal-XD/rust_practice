extern crate tr_tokio;
use std::thread::JoinHandle;

fn main() {
    let start = std::time::Instant::now();

    let mut handles = Vec::<JoinHandle<()>>::new();

    for i in 1..10000 {
        let handle = std::thread::spawn(|| {
            let mut sum: u128= 0;
            for i in 1..1000000 {
                sum += i;
            }
            sum = sum % 100;
        });
        handles.push(handle);
    }

    while let Some(result) = handles.pop() {
        result.join().unwrap();
    }

    let duration = start.elapsed();

    // Execution time: 8.26901868s
    println!("Execution time: {duration:?}");
}