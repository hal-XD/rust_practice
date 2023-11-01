
extern crate tr_tokio;
use tr_tokio::thread_pool_executor::*;
use std::collections::{VecDeque};

// ThreadPoolExecutor実行用のタスク
struct TestTask {}
impl Runnable for TestTask {
    fn run(&self) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("finished!!")
    }
}

fn main() {
    let mut executor = ThreadPoolExecutor::new(1
        ,1
        ,1
        ,VecDeque::<TestTask>::new());
    let task = TestTask{};
    executor.execute(task);
}