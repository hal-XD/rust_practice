
pub mod chapter6 {

    pub fn chapter6_1() {
        const ELEMENT1:&str = "POLICE OFFICER";
        assert_eq!(ELEMENT1,"POLICE OFFICER");
        let b1:i8 = 0b1111111; // 8bit 符号有
        let b2:u8 = 0b11111111; // 8bit符号無
        let b3:i8 = - 0b1111111; // 負　8bit目を指定できない？
        assert_eq!(b1 >> 2, 0b0011111); // 論理シフト
        assert_eq!(b2 >> 2, 0b0111111); // 算術シフト
        assert_eq!(b3,-127);
    }

    pub fn chapter6_8_1() {
        let even_or_odd = if 10 % 2 == 0 {
            "an_even"
        } else {
            unreachable!()
        };
        assert_eq!(even_or_odd,"an_even");

        let test = if 12 % 5 == 0 {
            unreachable!()
        } else {
            "not_return"; // ;を付けると()が返る
        };
        assert_eq!(test,());
    }

    pub fn chapter6_8_2() {
        let value = 10;
        match value {
            1 => assert_eq!(value,1),
            10 => assert_eq!(value,10),
            _ => unreachable!(),
        }

        let v = match  value {
            10 => assert_eq!(value,10),
            _ => unreachable!(),
        };

        let number = 46;
        let s = match number {
            1 | 2 | 3 => unreachable!(),
            40...50 => assert_eq!(number,46),
            _ => unreachable!(),
        };

        let score = Some(100);
        // if let = matchの糖衣構文
        if let Some(100) /* パターン */ = score /* 比較対象 */ {
            assert_eq!(score,Some(100)) /* パターンにマッチしたらここ */
        } else {
            unreachable!()
        }
    }

    pub fn chapter6_9_1() {
        let mut counter = 0;
        loop { // 無限ループを作る
            println!("loop!!");
            counter = counter + 1;
            if counter == 10 {
                break;
            }
        }

        let mut counter = Some(0);
        // パターンにマッチした場合 処理を実行しループを繰り返す
        // 失敗した場合はループを抜ける
        while let Some(i) /* パターン */= counter /* 比較対象 */ {
            if i == 10 { // Someのiが10かどうか
                counter = None;
            } else {
                println!("{}",i);
                counter = Some(i+1);
            }
        } 
    }

    pub fn chapter6_10() {
        let one = 1;
        let puls_one = |x:i32| {
            x + one
        };
        assert_eq!(11,puls_one(10));

        let mut two = 2;
        let puls_two = move /* moveにより変数がコピーされる */ 
            |x:i32| {
            x + two
        };
        two += 2; // コピー元を変更してもコピー先に影響はない
        assert_eq!(12,puls_two(10))
    }
}

#[cfg(test)]
mod test_chapter6 {
    use super::chapter6::{chapter6_1, chapter6_8_1, chapter6_8_2, chapter6_10};

    #[test]
    fn test1 () {
        chapter6_1();
        chapter6_8_1();
        chapter6_8_2();
        chapter6_10();
    }
}
