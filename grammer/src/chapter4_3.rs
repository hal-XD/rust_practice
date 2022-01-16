use std::ops::RangeBounds;


pub fn c4_tuplp() {
    let t1 = (100,true);
    assert_eq!(t1.0,100);
    // pattern match
    let (_,x2) = t1;
    assert_eq!(x2,true);
}

pub fn c4_arrary() {
    let a1 = [1,23,4,5,6,7,8];
    assert_eq!(a1.len(),7);
    assert_eq!(a1[1],23);
    assert_eq!(a1.get(6),Some(&8_i32));
    assert_eq!(a1.get(1000),None);
    // i32
    for el in a1 {
        print!("{}",el);
    }
    println!("");
    // &i32
    for el in a1.iter() {
        print!("{}",el)
    }
    println!("")
}

pub fn c4_slice() {
    let s = "my name is hal.";
    let ca = ['a','c','f','e'];
    println!("{:9} len : {}, first: {:?}, sl[1]: {:?}, last: {:?}",
        s,
        ca.len(),
        ca.first(),
        ca[1],
        ca.last()
    )
}

pub fn c4_slice_2() {
    let a1 = ['a','c','e','g'];
    print_info("&a[..]", &a1[..]);
}

fn print_info(name:&str, sl: &[char]) {
    println!("{:9} - {}, {:?},{:?},{:?}",
        name,
        sl.len(),
        sl.first(),
        sl[1],
        sl.last()
    )
}

pub fn c4_slice_3() {
    let a3 = ["zero","one","two","three"];
    assert_eq!(a3.first(),Some(&"zero"));
    assert_eq!(a3.get(1),Some(&"one"));
    assert!(a3.contains(&"two"));
}

pub fn c4_slice_str() {
    // &str
    let s1 = "abcdefghijklmnopqlstuvwxyz";
    let s2 = "1234567890";
    assert!(s1 > s2);
    assert!(s1 != s2);
    let sentence = "Today is good day.\nAnd yeserday is also good day.";
    let mut lines = sentence.lines();
    assert_eq!(lines.next(),Some("Today is good day."));
}

pub fn c4_string() {
    let s1 = "test";
    let s2 = "test";
    let mut s3 = s1.to_string();
    s3 = s3 + "hoge";
    println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
}

#[cfg(test)]
mod c4_3_test {
    use super::{c4_tuplp, c4_slice_3, c4_slice_str};

    #[test]
    fn test1() {
        c4_tuplp();
    }

    #[test]
    fn test2() {
        c4_slice_3();
    }

    #[test]
    fn test3() {
        c4_slice_str();
    }

}