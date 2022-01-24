
/*
    src/main.sr
       module1
        |______ mod.rs
                some_module.rs
    
    main.rsからmoudel1配下のsome_module.rsを呼ぶには
    module1/mod.rsを作ってそこで pob mod some_moduleで公開する
    すると main.rsからは mod module1::some_module1で使える
    ようになる。

*/

// module1に入っているmoduleを公開する
pub mod some_module1;