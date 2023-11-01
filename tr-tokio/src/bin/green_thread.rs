
// osスレッドとgreen threadの比較
// タスクをスレッドに分割することで時間が短くなる
// 

use std::{collections::HashMap, f32::consts::E};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    println!("start green_thread");

    //let mut tasks = Vec::<tokio::task::JoinHandle<i32>>::new();
    // vecではなくFuturesUnorderedで管理することで終了したtaskを取り出すことができるようになる。
    let mut tasks = futures::stream::FuturesUnordered::<tokio::task::JoinHandle<i32>>::new();
    let h1 = tokio::spawn(async {
        let mut sum = 0;
        for i in 0..10000 {
            sum += i + i % 2;
        }
        sum
    });
    tasks.push(h1);
    for rn in 1..10 {
        let hx = tokio::spawn(async move {
            let mut sum = 0;
            for i in 0..10000 {
                sum += i + i % 2 + rn % 10 ;
            }
            sum
        });
        tasks.push(hx);
    }

    while let Some(result) = tasks.next().await {
        match result {
            Ok(r) => {println!("Task completed with result.{r}")},
            Err(e) if e.is_panic() => {eprintln!("Task paniced. {e}")},
            Err(e) => {eprintln!("task failed with error. {e}")},
        }
    }
    println!("end green thread");
}