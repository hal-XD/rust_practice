use std::fmt::Debug;


fn borrow_x<T>(x:T)
    where T: Debug
{
    dbg!(x);
}
fn borrow_y(x: &i32){
    dbg!(x);
}

fn main(){
    let x = "hoge".to_string();
    let y = x.clone(); // ここで所有権の移動は発生しない？？
    dbg!(&x);
    borrow_x(x);
    let z = 13;
    borrow_x(z);
    borrow_x(z);
}