
pub fn hoge(){
    println!("hoge");
}

pub mod hooge {
    pub struct Hoge {}
    impl Hoge {
        pub fn say(self){
            println!("Hoge!!");
        }
    }
}