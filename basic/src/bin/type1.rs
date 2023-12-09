
const CONST_VER:&str = "CONST";

// モジュールでまとめることもできる
mod my_const {
    pub const NO_1: &str = "No1";
    pub static NO_2: &str = "No2";
    pub static mut NO_3: &str = "mut";
}

fn main() {
    assert_eq!(CONST_VER, "CONST");
    static STATIC_VER: &str = "static";
    // CONST_VER = "foo"; constな変数は変更できない
    assert_eq!(STATIC_VER, "static");
    assert_eq!(my_const::NO_1,"No1");
    assert_eq!(my_const::NO_2,"No2");
    unsafe {
        assert_eq!(my_const::NO_3,"mut");
        my_const::NO_3 = "No3";
        assert_eq!(my_const::NO_3,"No3");
    }
}
