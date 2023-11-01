use futures::executor::block_on;

// 並行処理、並列処理、非同期処理
// 並列処理: 複数コア、ハイパースレッディングなどが必要 => 実行環境に依存する


fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {std::process::exit(1);}
    match args[1].as_str() {
        "1" => {parallel()}
        "2" => {concurrent()}
        "3" => {asynchronous()}
        _ => {unimplemented!()}
    }
}

// 並列処理
fn parallel() {
    // マルチコアやマルチプロセス環境では並行処理が並列に実行される可能性がある
    concurrent();
}

// 並行処理
fn concurrent(){
    let handle = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("hello? from thread.");
    });

    println!("hello. from main thread");
    handle.join().unwrap();
}

// 非同期処理
fn asynchronous(){
    block_on(sample_async());
}

async fn sample_async(){

}