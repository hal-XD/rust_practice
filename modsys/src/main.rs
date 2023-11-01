/// package
///  main.rs : クレートルート

pub mod fuga;
pub mod bar;

fn main() {
    println!("Hello, world!");
    hello::mod_hello_world();
    hello::mod_call_private();
    hello::hw::stand();

    // 他のクレートも使える
    //  modsys(crate名はlib.rsから)
    //  crateはクレートルートに引きこんがものから
    modsys::call_lib_func();    // lib.rsから
    modsys::hoge::hoge();       // src/hoge.rs から
    modsys::fuga::fuga::fuga(); // src/fuga/fuga.rs　から
    crate::fuga::fuga::fuga();  // pub mod fuga;でmianに引き込めばcrateからアクセスできる

    // 感覚的には crate::fuga::fuga()でアクセスしたい
    //           std::fs::read;
    crate::fuga::fuga::foouga::foooga();
    let s = modsys::hoge::hooge::Hoge{};
    s.say();

    bar::yyy();
}

/// モジュール分割
mod hello {
    
    /// 外に公開される
    pub fn mod_hello_world(){
        println!("mod hello?world?");
    }

    /// 外に公開されない
    fn mod_private(){
        println!("private!!");
    }

    /// privateな関数を呼んで外に公開できる。
    pub fn mod_call_private(){
        mod_private();
    }

    /// 外に公開されないprivateなモジュール
    mod world {
        pub fn the_world(){
            _the_world();
        }

        fn _the_world(){
            println!("stop time!!");
        }
    }

    // 外に公開されるpublicなモジュール
    pub mod hw {
        pub fn stand(){
            // superを使うことで上のモジュールの世界に入れる。
            super::world::the_world();
        }
    }

}