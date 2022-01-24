
// module ファイルのインポート
pub mod chapter4_2;
pub mod chapter4_3;
pub mod chapter5;
pub mod chapter6;
pub mod chapter7;
pub mod enum1;

/*
モジュールの階層化について2通りの方法がある。
module1 : 2015 Edition方式. src/module1/mod.rs を参照
module2 : 2018 Edition方式. src/module2.rs を参照
 */
pub mod module1;
pub mod module2;

/**
    main.rs : エントリーバイナリファイル
    creat(木箱) : 
 */

fn main() {
    println!("rust grammer.");
    // say_hello()
    module1::some_module1::say_myself();
    module2::some1::say_myself();
    chapter7::chapter7_8::chapter7_8();
}

// rustc --cfg some_condition src/main.rs  でコンパイルすると実行される
#[cfg(some_condition)]
fn say_hello() {
    println!("hello");
}