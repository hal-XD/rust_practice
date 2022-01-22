use std::fmt::Debug;

// 関数
pub fn hello() {
    println!("hello.")
}

// 桁あふれ
pub fn overflow() {
    let n1 = 200u8;
    let n2 = 3u8;
    assert_eq!(n1.checked_mul(n2), None);
}

// 文字操作
pub fn character() {
    let c1 = 'a';
    let s1 = "a";
    assert_eq!(c1,'a');
    assert_eq!(s1,"a");
    let emoji = '\u{1f600}';
    let emos = "\u{1F916} oh emoji! \u{1F4AF}";
    println!("{} {}",emoji,emos);
}

// 参照 reference
pub fn reference() {
    let x = 10;
    let y = &x;
    let z = *y;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
    let x_ptr : *const i32 = &x;
    unsafe{
        println!("pointer dereference : {}", *x_ptr);
    }

    pub fn cal(n: i32, m: i32) -> i32 {
        n*m
    }

    pub fn cal2(n: i32,m:i32) -> i32 {
        n + m
    }

    // pub fn pointer type
    let mut f: fn(i32,i32) -> i32 = cal;
    println!("{} , mem size=[{}]",f(3,5),std::mem::size_of_val(&f));
    f = cal2;
    println!("{} , mem size=[{}]",f(3, 5),std::mem::size_of_val(&f));
    utilpring(&f);
}

pub fn utilpring<T>(a:&T)
    where T: Debug
{
    println!("{:?}",a)
}

#[cfg(test)]
mod test {
    
}