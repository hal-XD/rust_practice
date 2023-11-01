use std::{time::{Instant, Duration}};
use futures::stream::FuturesUnordered;
use futures::StreamExt;


// 非同期処理のベンチマーク

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let mut handles = FuturesUnordered::<tokio::task::JoinHandle<()>>::new();

    for i in 1..10000 {
        let handle = tokio::spawn(async {
            let mut sum: u128 = 0;
            for i in 1..1000000 {
                sum += i;
            }
            sum = sum % 100;
        });
        handles.push(handle);
    }

    while let Some(result) = handles.next().await {
        continue;
    }
    let duration = start.elapsed();

    // Execution time: 8.982620435s
    println!("Execution time: {duration:?}");
}
