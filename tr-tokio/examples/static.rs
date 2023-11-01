// static　変数はコンパイル時に生成される。
// 基本はimmutable, 変更するのはunsage
// プログラムの終了まで有効
static BYTES:[u8;3] = [1,2,3];
static mut MUT_BYTE:[u8;3] = [2,3,5];
fn main(){
    let str_literal:&str = "str";
    let str_literal2:&'static str = "str";
    unsafe {
        // staticな変数の変更はunsafe
        MUT_BYTE[1] = 99;
    }
}
