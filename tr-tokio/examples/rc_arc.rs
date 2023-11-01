use std::rc::Rc;
use std::sync::Arc;
use std::thread;
fn main(){
    // スマートポインタ
    let x = Rc::new(42);
    assert_eq!(Rc::strong_count(&x),1);
    let y = x.clone();
    assert_eq!(Rc::strong_count(&x),2);
    assert_eq!(Rc::strong_count(&y),2);
    assert!(Rc::ptr_eq(&x, &y));
    println!("x={x:p} (pints to {x:})");
    println!("y={y:p} (pints to {y:})");

    // Arc = スレッド安全
    // Rcはスレッド安全ではない
    // let failed = thread::spawn(move || {println!("{x}")}); 
    let arc = Arc::new(42);
    let handle = thread::spawn(move || {
        println!("{arc}");
    });
    handle.join().unwrap();

    // +α Mutex,RefCall,Arefcall
}