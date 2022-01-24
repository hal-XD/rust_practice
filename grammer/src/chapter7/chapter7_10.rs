
/*
ポインタ?
Rc型  ... Reference Counted(参照カウントされた) 1つのリソースに対して複数の所有者を持たせられる
Arc型 ... Atomically Reference Counted 複数スレッドで共有化 
*/

use std::{rc::Rc};

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn chapter7_10() {
    let mut rc1 = Rc::new(Child(1)); // Child(1)　への参照を作る
    assert_eq!(Rc::strong_count(&rc1),1);
    {
        let rc2 = Rc::clone(&rc1); // rc1への参照が複製される
        assert_eq!(Rc::strong_count(&rc1),2); // Child(1) への参照は二つ
        assert_eq!(Rc::strong_count(&rc2),2);
    } // rc2 がスコープから抜ける
    assert_eq!(Rc::strong_count(&rc1),1);

    if let Some(child) /* パターン */ 
        = Rc::get_mut(&mut rc1) /* 比較対象 */ {
        child.0 += 1;
    }

    let weak = Rc::downgrade(&rc1); // weakポインタ

    assert_eq!(Rc::strong_count(&rc1),1); // weak ポインタはカウントされない

    if let Some(rc3) = weak.upgrade() {
        // weakからrcにアップグレードすると参照カウントが増加する
        assert_eq!(Rc::strong_count(&rc1),2);
        assert_eq!(Rc::strong_count(&rc3),2);
    } else {
        unreachable!();
    }

    std::mem::drop(rc1);
    assert!(weak.upgrade().is_none()) // Noneとなる
}

#[cfg(test)]
mod chapter7_10 {
    
    #[test]
    fn test1() {
        super::chapter7_10();
    }
}