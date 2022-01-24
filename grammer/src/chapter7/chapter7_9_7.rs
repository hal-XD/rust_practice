
/*
const : 不変の値
static : スタティックなライフタイムを持つミュータブルな値
*/

pub fn chapter7_9_7() {
    static I0 : i32 = 10; // 'staticスコープを持つ。 プログラム終了まで生存
    let mut s0 : &'static str;
    let s1 = "10";
    let s2 = 10.to_string();
    s0 = s1;
    // s0 = &s2; &'static strが作れないため
}

#[cfg(test)]
mod chapter7_9_7_test {

    #[test]
    fn test1(){
        super::chapter7_9_7();
    }
}