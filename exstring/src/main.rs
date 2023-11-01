
// global const
const HOGE: &str = "hoge!!";

fn main() {
    println!("Hello, world! {}", HOGE);
    let s1 = String::from(HOGE);
    let c1 = 'a';
    println!("s1 {} c1 {}", s1,c1);

    // 16進数表記の文字列を10進数に変換する。
    let hex_8bit = "FF";
    assert_eq!(255, u8::from_str_radix(&hex_8bit, 16).unwrap());
    assert_eq!(255, u16::from_str_radix(&hex_8bit, 16).unwrap());
    assert_eq!(255, u32::from_str_radix(&hex_8bit, 16).unwrap());
    assert_eq!(255, u64::from_str_radix(&hex_8bit, 16).unwrap());
    assert_eq!(255, u128::from_str_radix(&hex_8bit, 16).unwrap());

    // 2進数表記の文字列を10進数に変換する
    let hex = "1111";
    assert_eq!(15, u8::from_str_radix(&hex, 2).unwrap());

    // バイト列 => 文字列
    let bytes: [u8;10] = [112,113,123,23,24,23,53,78,45,44];
    let s = bytes.iter()
        .map(|&b| b as char)
        .collect::<String>();
    println!("{}", s);
}
