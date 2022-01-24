use std::{ops::Index, default};

pub struct ToyVec<T> {
    elements : Box<[T]>,
    len : usize,
}

// trait境界にDefaultを指定
impl<T:Default> ToyVec<T> {

    /*
    関連関数とメソッド
    関連関数 : 第一引数がselfでない。 ::で呼び出し
    メソッド : 第一引数がself. .で呼び出し

    javaならstaticメソッドが関連関数？
    */

    // objectのnewと似ている
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // 指定された大きさのToyVecを生成する
    pub fn with_capacity(capacity : usize) -> Self {
        Self {
            elements : Self::allocate_in_heap(capacity),
            len : 0,
        }
    }

    fn allocate_in_heap(size : usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)// size個　T型のデフォルト値を作成する
                                            // take はiterからsize呼取り出す
            .collect::<Vec<_>>()// size個のT型のデフォルト値をVecに入れる
            .into_boxed_slice() // VecをBoxに変換
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self,element:T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element; // elementからselfへ値の所有権が移動する
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    fn grow(&mut self) {
        match  self.capacity() {
            0 => self.elements = Self::allocate_in_heap(1),
            _ => {
                let nbox = Self::allocate_in_heap(self.capacity() * 2);
                // nboxにself.elementsの値を渡したい
                // Tがcopyを実装しているか不明のためcopyはできない
                // elementsから参照外しで取ることもできない。

                // replace で oboxにselfのelemetsの値を,self.elementにnboxを渡す
                let obox = std::mem::replace(&mut self.elements, nbox);
                // oboxに退避した値をelementsに格納していく
                for (index, element) in obox.into_vec().into_iter().enumerate() {
                    self.elements[index] = element;
                }
            }
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        /* 
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
        */

        // 上のmatch式と等価
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    pub fn iter<'vec>(&'vec self) -> Iter<'vec,T> {
        Iter { 
            elements: &self.elements, 
            len: self.len, 
            pos: 0,
        }
    }

}

pub struct Iter<'vec, T> {
    elements : &'vec Box<[T]>,
    len : usize,
    pos : usize,
}

// Iterを経由せずに直接ToyVecに定義すると?
// pos の管理が面倒
// elementsを借用することでiterを使っている間は
// 変更を阻止できる
impl<'vec, T> Iterator for Iter<'vec, T> {
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None // posが端を指したら終わり
        } else {
            // posから一つず
            // posさえあればIterはいらないのでは？
            // 参照なら大したコストではない？
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}

#[cfg(test)]
mod toy_vec_test {
    use super::ToyVec;

    #[test]
    fn test1() { // ToyVec new len capacity push getの確認
        let mut tv = ToyVec::<String>::new();
        assert_eq!(tv.len(),0);
        assert_eq!(tv.capacity(),0);
        tv.push("C++".to_string());
        assert_eq!(tv.len(),1);
        assert_eq!(tv.capacity(),1);
        tv.push("python".to_string());
        assert_eq!(tv.len(),2);
        assert_eq!(tv.capacity(),2);
        let e = tv.get(1);
        assert_eq!(e,Some(&"python".to_string()));
    }

    #[test]
    fn test2() {
        let mut tv = ToyVec::<String>::new();
        tv.push("C++".to_string());
        tv.push("python".to_string());
        let t = tv.pop();
        assert_eq!(t,Some("python".to_string()));
        let s = tv.pop();
        assert_eq!(s,Some("C++".to_string()));
    }

    #[test]
    fn test3() {
        let mut tv = ToyVec::<String>::new();
        tv.push("C++".to_string());
        tv.push("python".to_string());
        let mut i = tv.iter();
        assert_eq!(i.next(),Some(&"C++".to_string()));
        assert_eq!(i.next(),Some(&"python".to_string()));
        assert_eq!(i.next(),None);
    }

}