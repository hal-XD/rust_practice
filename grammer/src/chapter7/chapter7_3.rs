use std::ops::Drop;
use std::marker::Copy;

#[derive(Debug)]
struct Parent(usize,Child,Child);

#[derive(Debug)]
struct Child(usize);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}",self)
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}",self)
    }
}

// Copyトレイトを実装しているとmove semanticsの代わりにcopy semanticsとなる
#[derive(Debug,Copy,Clone)]
struct CParent(usize,CChild,CChild);

#[derive(Debug,Copy,Clone)]
struct CChild(usize);

pub fn chapter7_3() {
    // p1 はParent(..)を所有する。
    // Parent()は1,Child(11),Child(12)を所有する
    // Childはそれぞれ11,12を所有する
    let p1 = Parent(1,Child(11),Child(12));
    {
        let p2 = Parent(2,Child(21),Child(22));
        println!("1 {:?} \n  {:?}",p1,p2);
    }
    println!("2 {:?} ",p1);
    let p3 = Parent(3,Child(31),Child(32));
    println!("3 {:?} \n  {:?}",p1,p3);

    let mut p4 = Parent(4,Child(41),Child(42));
    let p5 = p4; // move semantics p4の所有権をp5にmove (譲渡)
    println!("4 {:?} ",p5);
    // println!("x {:?} ",p4); error[E0382]: borrow of moved value: `p4`
    p4 = Parent(5,Child(51),Child(52)); // 新たにParentを割り当てることはできる
    println!("5 {:?} ",p4);

    let mut p6 = CParent(4,CChild(41),CChild(42));
    let p7 = p4; // copy seamntics
    println!("6 {:?} ",p6);
    println!("7 {:?} ",p7); // p6に所有権が残ったまま


    let mut p8 = Parent(1,Child(11),Child(12));
    borrow1(&p8); // 不変の借用
    borrow_mut2(&mut p8); // 可変の借用
    println!("8 {:?} ",p8); // 貸し出しただけなのでp8はまだ所有権を持っている
}


fn borrow1(p : &Parent) {
    println!("borrow1: {:?}",p);
}

fn borrow_mut2(p : &mut Parent) {
    println!("borrow_mut2: {:?}",p);
}

/* デストラクタの動き
1 Parent(1, Child(11), Child(12)) 
  Parent(2, Child(21), Child(22))
Dropping Parent(2, Child(21), Child(22))
Dropping Child(21)
Dropping Child(22)
2 Parent(1, Child(11), Child(12)) 
3 Parent(1, Child(11), Child(12)) 
  Parent(3, Child(31), Child(32))
Dropping Parent(3, Child(31), Child(32))
Dropping Child(31)
Dropping Child(32)
Dropping Parent(1, Child(11), Child(12))
Dropping Child(11)
Dropping Child(12)
*/