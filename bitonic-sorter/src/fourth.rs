use super::SortOrder;
use rayon;
use std::{cmp::Ordering};

const PARALLEL_THRESHOLD : usize = 4096;

// pub = 外部に公開!!
// 型パラメータTにtrait境界Ordを追加
// trait境界は+で連結可能
pub fn sort<T:Ord+Send>(x:&mut [T],order:&SortOrder) -> Result<(),String>{
    match *order {
        SortOrder::Ascending => sort_by(x,&|a,b| a.cmp(b)),
        SortOrder::Descending => sort_by(x, &|a,b| b.cmp(a)),
    }
}

// クロージャーは全て異なる型となるためジェネリクスにする必要がある。
pub fn sort_by<T,F>(x:&mut [T],comparator:&F) -> Result<(),String>
    // where でtrait境界を指定 = 受け取るクロージャーの条件?
    // + でtrait境界を連結できる
    where F:Fn(&T,&T) -> Ordering + Sync,
        T : Send
{
    if x.len().is_power_of_two() {
        do_sort(x, true,comparator);
        Ok(())
    } else {
        Err(format!("x'length is invalid. x.len():{}",x.len()))
    }
}

fn do_sort<T,F>(x:&mut [T],forward : bool,comparator:&F) 
    where F : Fn(&T,&T) -> Ordering + Sync,
        T : Send
{
    if x.len() > 1 {
        let mid_point = x.len()/2;
        let (first,second) = x.split_at_mut(mid_point);
        if mid_point >= PARALLEL_THRESHOLD {
            rayon::join(|| do_sort(first, true, comparator),
                        || do_sort(second, false, comparator));
        } else {
            do_sort(first,true,comparator);
            do_sort(second,false,comparator);
        }
        sub_sort(x, forward,comparator);
    }
}

// pubを付けない = このモジュール内だけ！！
fn sub_sort<T,F>(x:&mut [T],forward:bool,comparator:&F) 
    where F : Fn(&T,&T) -> Ordering + Sync,
        T : Send
{
    if x.len()>1{
        compare_and_swap(x, forward,comparator);
        let mid_point = x.len()/2;
        let (first,second) = x.split_at_mut(mid_point);
        if mid_point > PARALLEL_THRESHOLD {
            rayon::join(|| sub_sort(first, forward, comparator),
                        || sub_sort(second, forward, comparator));
        } else {
            sub_sort(first, forward,comparator);
            sub_sort(second, forward,comparator);
        }
    }
}

fn compare_and_swap<T,F>(x: &mut [T],forward : bool,comparator:&F)
    where F: Fn(&T,&T) -> Ordering
{
    // Rust Style Guideで推奨されている書き方
    let swap_condition = if forward {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    let mid_point = x.len()/2;
    for i in 0..mid_point{
        if comparator(&x[i],&x[mid_point+i]) == swap_condition {
            x.swap(i, mid_point + i);
        }
    }
}

// #[cfg(test)]が付いたモジュールはcargo testの時のみコンパイルされる
#[cfg(test)]
mod tests {
    use super::{sort,sort_by};
    use crate::SortOrder::*;
    use crate::utils::{new_u32_vec,is_sorted_ascending,is_sorted_descending};

    // #[test]が付いた関数はcargo testをしたときに実行される
    #[test]
    fn sort_u32_ascending() {
        let mut x: Vec<u32> = vec![10,30,11,20,4,330,21,110];
        assert_eq!(sort(&mut x,&Ascending),Ok(()));
        assert_eq!(x,vec![4,10,11,20,21,30,110,330]);
    }

    #[test]
    fn sort_u32_decending(){
        let mut x : Vec<u32> = vec![10,30,11,20,4,330,21,110];
        assert_eq!(sort(&mut x,&Descending),Ok(()));
        assert_eq!(x,vec![330,110,30,21,20,11,10,4]);
    }

    #[test]
    fn sort_str_ascending(){
        let mut x = vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];
        assert_eq!(sort(&mut x,&Ascending),Ok(()));
        assert_eq!(x,vec!["GC","Rust","and","fast","is","memory-efficient","no","with"]);
    }

    #[test]
    fn sort_str_descending(){
        let mut x = vec!["Rust","is","fast","and","memory-efficient","with","no","GC"];
        assert_eq!(sort(&mut x,&Descending),Ok(()));
        assert_eq!(x,vec!["with","no","memory-efficient","is","fast","and","Rust","GC"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10,30,99];
        assert!(sort(&mut x,&Ascending).is_err());
    }

    // deriveアトリビュートでDebugトレイととPartialEqトレイトを
    // 自動導出 (= コードの自動生成)している。
    #[derive(Debug,PartialEq)]
    struct Student {
        first_name : String,
        last_name : String,
        age : u8,
    }

    // 構造体にトレイトを実装している
    impl Student {
        fn new(first_name : &str,last_name:&str,age:u8) -> Self {
            Self {
                first_name : first_name.to_string(),
                last_name : last_name.to_string(),
                age,
            }
        }
    }

    #[test]
    fn sort_sutudents_by_age_ascending(){
        let mike = Student::new("Mike","Joge",20);
        let kei = Student::new("kei","Kawasaki",8);
        let alex = Student::new("Alex","Potter",5);
        let ryo = Student::new("Ryo","Matuda",99);
        let mut x = vec![&mike,&kei,&alex,&ryo];
        let expected = vec![&alex,&kei,&mike,&ryo];
        // クロージャーについて
        // |a,b| a.age.cmp(&b.age)
        // fn comparator(a:&&Student,b:&&Student)->std::cmp::Ordering {a.age.cmp(&b.age)}
        assert_eq!(sort_by(&mut x,&|a,b| a.age.cmp(&b.age)),Ok(()));
        assert_eq!(x,expected);
    }

    #[test]
    fn sort_sutudents_by_name_ascending(){
        let mike = Student::new("Mike","Joge",20);
        let kei = Student::new("Kei","Joge",8);
        let alex = Student::new("Alex","Potter",5);
        let ryo = Student::new("Ryo","Matuda",99);
        let mut x = vec![&mike,&kei,&alex,&ryo];
        let expected = vec![&kei,&mike,&ryo,&alex];
        // クロージャーについて
        // |a,b| a.age.cmp(&b.age)
        // fn comparator(a:&&Student,b:&&Student)->std::cmp::Ordering {a.age.cmp(&b.age)}
        assert_eq!(sort_by(&mut x,&|a,b| a.last_name.cmp(&b.last_name) 
            .then_with(|| a.first_name.cmp(&b.first_name))),Ok(()));
        assert_eq!(x,expected);
    }

    #[test]
    fn sort_u32_large(){
        // {}で別スコープになっている
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x,&Ascending),Ok(()));
            assert!(is_sorted_ascending(&x));
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x,&Descending),Ok(()));
            assert!(is_sorted_descending(&x));
        }
    }
}