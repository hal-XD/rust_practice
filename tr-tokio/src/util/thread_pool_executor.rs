

use std::collections::{VecDeque, HashSet};

pub trait Runnable {
    fn run(&self);
}

/// JavaのThreadPoolExecutorを参考にする
pub struct ThreadPoolExecutor<T>
    where T: Runnable
{
    core_pool_size: usize,
    maximum_pool_size: usize,
    keep_alive_time: usize,
    blocking_queue: VecDeque<T>,
    workers: HashSet<T>
}

impl<T> ThreadPoolExecutor<T>
    where T: Runnable
{

    /// keep_alive_time: この時間を超過したアイドル状態のスレッドは終了させる
    pub fn new(core_pool_size: usize, maximum_pool_size: usize, keep_alive_time: usize, blocking_queue: VecDeque<T>) -> ThreadPoolExecutor<T> {
        let workers = HashSet::<T>::new();
        for _ in 0..core_pool_size {
            let worker = tokio::spawn(async {});
        }
        ThreadPoolExecutor { 
            core_pool_size,
            maximum_pool_size,
            keep_alive_time,
            blocking_queue,
            workers,
        }
    }

    pub fn execute(&mut self, runner: T) {
        self.blocking_queue.push_back(runner);
    }
    pub fn shutdown(&self){}


}
struct  Worker<T> where T: Runnable{
    task: T,
}
impl<T> Worker<T>
where T:Runnable
{
    fn run_worker(&self){}
}
impl<T> Runnable for Worker<T> where T:Runnable{
    fn run(&self) {
        self.run_worker();
    }
}