use std::collections::HashMap;

pub fn chapter7_8() {

    fn process_or_default(key: char, map: &mut HashMap<char,String>) {
        match map.get_mut(&key) {
            Some(value) => {
                // valueにはmapからget_mutで取り出されたSome(value)のvalueが貸し出されている
                // valueが生存している間はmapは借用された状態である。
                // よって以下のコードはコンパイルエラーになる
                // map.insert('s', "match!".to_string());
                // error[E0499]: cannot borrow `*map` as mutable more than once at a time
                value.push_str(",world")
            },
            None => {
                // ここではselfの借用は終了している
                // self = map.get_mut(&key)?
                map.insert(key, Default::default());
            }
        }
    }

    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
}