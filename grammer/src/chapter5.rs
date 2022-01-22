
pub mod chapter5 {
    use std::{collections::HashMap, vec};
    use crate::enum1::Task::*;


    pub fn chapter5_1() {
        let t1 = (3,"test".to_string());
        // dataはheapに置かれる
        let mut _b1 = Box::new(t1);
    }

    pub fn chapter5_2() {
        // arrayとvector
        // array -> stack
        // vector -> heap
        let _a1 = [1,2,34,5];
        let _v1 = vec![1,2,34,5];

        // hash Map
        let mut hm1 = HashMap::new();
        hm1.insert("key1", 100);
        assert_eq!(hm1.len(),1);
        hm1.insert("key2", 123);
        assert_eq!(hm1.get("key1"),Some(&100));
        assert_eq!(hm1.get("keyX"),None);
        let d = hm1.entry("key3").or_insert(200);
        *d += 13;
        assert_eq!(hm1.get("key3"),Some(&213));
        let hm2 = vec![("kkey1","apple"),("kkey2","gohst")] // vectorの宣言
                            .into_iter() // iterの生成
                            .collect::<HashMap<_,_>>(); // collectでHashMapに値を格納する
        assert_eq!(hm2.get("kkey1"),Some(&"apple"));
    }

    pub fn chapter5_4() {
        let mut s1 = "strawberry".to_string();
        let mut s2 = String::from("strawberry");
        assert_eq!(s1,s2);
        s1.push_str("icecream"); // &strを追加する
        s2.push('&'); // charを追加する
        s2.push_str(&s1);
        assert_eq!(s2,"strawberry&strawberryicecream");

        // to_string
        let i = 100;
        assert_eq!(i.to_string(),"100");
        let f = 3.1 + 0.04;
        assert_eq!(f.to_string(), "3.14");
        let f2 = 4.3 + 0.1;
        assert_eq!(f2.to_string(), "4.3999999999999995");
        let t = (1,"456");
        assert_eq!(format!("{:?}",t),r#"(1, "456")"#);

        let si1 = "314";
        assert_eq!(si1.parse::<i32>(), Ok(314));
        let si2 = "abc";
        let r2 : Result<f64, _> = si2.parse();
        assert!(r2.is_err());

        let cs = ['b','i','r','d','s'];
        assert_eq!(cs.iter().collect::<String>(), "birds");
        assert_eq!(&cs[1..].iter().collect::<String>(), "irds");

        let bad_utf8: [u8; 7] = [
            b'a',0xf0,0x90,0x80,0xe3,0x81,0x82
        ];
        let s = String::from_utf8_lossy(&bad_utf8);
        assert_eq!(s,"a\u{fffd}あ");
    }

    pub fn chapter5_5() {
        let a = [1,2,3,4,5,6,7];
        assert_eq!(a[..],[1,2,3,4,5,6,7]);
        assert_eq!(a[..3],[1,2,3,]);
        assert_eq!(a[..=3],[1,2,3,4]);
        assert_eq!(a[1..3],[2,3]);
        assert_eq!(a[1..=3],[2,3,4,]);

        assert_eq!(..,   std::ops::RangeFull);
        assert_eq!(..3,  std::ops::RangeTo{end:3});
        assert_eq!(..=3, std::ops::RangeToInclusive{end:3});
        assert_eq!(1..,  std::ops::RangeFrom{start:1});
        assert_eq!(1..3, std::ops::Range{start:1,end:3});
        assert_eq!(1..=3,std::ops::RangeInclusive::new(1,3));
    }

    pub fn chapter5_6() {
        let a1 = ['a','b','c','d'];
        assert_eq!(a1.get(0),Some(&'a'));
        assert_eq!(a1.get(4),None);
        let mut o1 = Some(10);
        match  o1 {
            Some(s) => assert_eq!(s,10),
            None => unreachable!(),
        }

        o1 = Some(20);
        if let Some(s) = o1 { // someはifでも取り出せる
            assert_eq!(s,20);
        }

        // someのunwrap
        let mut _o2 = Some(String::from("abcd"));
        assert_eq!(_o2.unwrap(),"abcd"); 
        _o2 = None;
        assert_eq!(_o2.unwrap_or_else(|| String::from("o2 is none")),"o2 is none");

        let mut o3 = Some(10);
        assert_eq!(o3.map(|n| n*10),Some(100));
        o3 = None;
        assert_eq!(o3.map(|n| n*11),None);
        o3 = Some(10);
        assert_eq!(
            o3.map(|n| n*10)
                .and_then(|n| 
                    if n>10000 {
                        Some(n)
                    } else {
                        None
                    })
                , None
        );

        fn add_elements(s:&[i32]) -> Option<i32> {
            // ? : Optionsの値がNoneの場合処理を
            // 中断してNoneを返送する
            let s0 = s.get(0)?;
            let s3 = s.get(3)?;
            Some(s0 + s3)
        }

        assert_eq!(add_elements(&[1,2,3,4]),Some(1+4));
        assert_eq!(add_elements(&[1,2]),None);

    }

    pub fn chapter5_7() {
        // ::<i32> は turbofish. generic関数の引数の型を指定している。
        assert_eq!("10".parse::<i32>(), Ok(10));
        assert!("a".parse::<i32>().is_err());

        fn add(s0: &str, s1:&str) -> Result<i32,std::num::ParseIntError> {
            let i0 = s0.parse::<i32>()?; // ? ErrならErr(std::num::ParseIntError)が返送される
            let i1 = s1.parse::<i32>()?;
            // let i1 = s1.parse::<i32>().map_err(|_e| "failed")?;
            Ok(i0+i1)
        }

        assert_eq!(add("3","4"),Ok(7));
        assert!(add("a","b").is_err());
    }

    pub fn chapter5_3_1() {
        // type alias
        type UserName = String;
        type Id = i64;
        type TimeStamp = i64;
        type User = (Id, UserName, TimeStamp);
        fn new_user(name: UserName, id: Id, date: TimeStamp) -> User {
            (id,name,date)
        }
        assert_eq!(
            new_user("mike".to_string(),10_i64, 100_i64)
        ,(10_i64,"mike".to_string(),100_i64));

        let testId : Id = 10;
        let testts: TimeStamp = 20;
        let bad_user = new_user("badUser".to_string(),testts, testId);

        // 型エイリアスの代わりにフィールドが一つのタプル構造体を使う。
        struct SUserName(String);
        struct SId(u64);
        struct STimeStamp(u64);
        type User2 = (SId,SUserName,STimeStamp);

        fn new_user2(name:SUserName,id:SId,timestamp: STimeStamp) -> User2 {
            (id,name,timestamp)
        }
        let n = SUserName(String::from("bas user2"));
        let i = SId(100);
        let t = STimeStamp(300);
        // コンパイルエラーになる
        // let bad_user2_NG = new_user2(n, t, i);
        let bad_user2_OK = new_user2(n, i, t);
    }

    pub fn chapter5_3_2() {
        // struct

        // 名前付きフィールド構造体
        struct Polygon {
            vertexes: Vec<(i32,i32)>,
            stroke_width : u8,
            fill : (u8,u8,u8)
        }

        // タプル構造体
        struct Triangle(Vertex, Vertex, Vertex);
        struct Vertex(i32,i32);

        // ユニット構造体
        struct UniqueValue;

        let mut trianle = Polygon {
            vertexes : vec![(0,0),(1,0),(0,1)],
            fill : (255,255,255),
            stroke_width : 1,
        };

        assert_eq!(trianle.vertexes[0],(0,0));
        assert_eq!(trianle.fill,(255,255,255));
        assert_eq!(trianle.stroke_width, 1);

        trianle.stroke_width += 10;
        assert_eq!(trianle.stroke_width, 11);

        // パターンマッチでアクセス .. で省略加
        // 変数を定義してそこにtrianleの値を入れる
        let Polygon { vertexes: quad_vx, fill,.. } = trianle;
        assert_eq!(3,quad_vx.len());
        assert_eq!(fill,(255_u8,255_u8,255_u8));

        // 変数名を省略したらフィールドと同じ名前の変数が作られる
        let Polygon { fill, ..} = trianle; 
        assert_eq!((255,255,255),fill);

        // 元の値を元に新しい構造体を作る
        // レコードアップデート構文
        // functional record update syntax
        let triangle1 = Polygon {
            vertexes : vec![(0,0),(3,0),(2,2)],
            fill : (200,200,200),
            stroke_width : 5,
        };

        let triangle2 = Polygon {
            vertexes : vec![(10,10),(-3,5),(40,50)],
            .. triangle1
        };

        assert_eq!(triangle2.stroke_width,5);

        //#[derive(Default)]
        struct Polygon2 {
            name : String,
            id : i32,
        }

        let polygon2 : Polygon2 = Default::default();
        assert_eq!(polygon2.name,"POLYGON2".to_string());

        // ここで実装したトレイとは上のpolygon2のnameのデフォルト値にも影響を及ぼす
        impl Default for Polygon2 {
            fn default() -> Self {
                Self {
                    name : "POLYGON2".to_string(),
                    id : 214,
                }
            }
        }
        let polygon2_2 = Polygon2 {
            id : 200,
            .. Default::default()
        };
        assert_eq!(polygon2_2.id,200);

        // tuple-like struct
        struct Triple(Double,Double,Double);
        struct Double(i32,i32);
        let d1 = Double(10,20);
        let d2 = Double(30,20);
        let d3 = Double(40,60);
        let triple = Triple(d1,d2,d3);
        assert_eq!(triple.0.0,10);

        // ユニット構造体
        // 値はいらないがトレイトは実装したいときなど
        #[derive(Debug,PartialEq)]
        struct UnitStruct;
        let us1 = UnitStruct;
        let us2 = UnitStruct;
        assert_eq!(us1,us2);
    }


    // enum 代数的データ型
    pub fn chapter5_3_3() {
        #[derive(Debug, PartialEq)]
        enum Weekday {
            // variant
            // データを持たないenumのvariantにはisizeの値が割り当てられる
            // 0    1        2          3         4       5          6
            Monday, Tuesday, Wednesday, Thursday, Friday, Saturaday, Sunday,
        }

        let ew = Weekday::Monday;
        assert_eq!(ew,Weekday::Monday);
        assert_eq!(1,Weekday::Monday as isize);

        let tasks = vec![
            AssignedTo(String::from("mika")),
            Working {
                assignee: String::from("yuki"),
                remaining_hours: 24,
            },
            Done,
        ];

        for (i,task) in tasks.iter().enumerate() {
            match task {
                AssignedTo(assignee) => {
                    assert_eq!(assignee,&"mika".to_string());
                }
                Working {assignee, remaining_hours} => {
                    assert_eq!(assignee,&String::from("yuki"));
                    assert_eq!(remaining_hours,&24);
                }
                _ => println!("Done")
            }
        }
    }
}

#[cfg(test)]
mod test_ch5 {
    use super::chapter5::{chapter5_2, chapter5_4, chapter5_5, chapter5_6, chapter5_7, chapter5_3_1,
         chapter5_3_2, chapter5_3_3};


    #[test]
    fn test1() {
        chapter5_2();
    }

    #[test]
    fn test2() {
        chapter5_4();
    }

    #[test]
    fn test3() {
        chapter5_5();
    }

    #[test]
    fn test4() {
        chapter5_6();
    }

    #[test]
    fn test5() {
        chapter5_7();
    }

    #[test]
    fn test6() {
        chapter5_3_1();
    }

    #[test]
    fn test7() {
        chapter5_3_2()
    }

    fn test8() {
        chapter5_3_3()
    }
}