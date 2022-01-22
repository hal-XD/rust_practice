// src/main.rsが実行可能ファイル生成時の
// エントリポイント

// クレート : コンパイルの単位
// モジュール : クレートを分割している
// クレートの最上位には匿名モジュールがある
// アイテム : モジュールの中の要素

// 名前空間に別名を付ける
use std::io::{self, Write};
//use std::io::Writer;

fn main() {
    let mut year = String::new();
    print!("Please input a yewar to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year",year);
    } else {
        println!("{} is not a leap year.",year);
    }
}

// 関数。
// シグネチャ : 関数の戻り値を除いた部分
fn is_leap_year(year : u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
