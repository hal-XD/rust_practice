/// ライブラリクレートになる。

pub mod hoge;
pub mod fuga;

pub fn call_lib_func(){
    crate::fuga::fuga::fuga();
    println!("call_lib_func");
}