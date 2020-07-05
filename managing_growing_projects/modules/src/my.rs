pub fn fn0() {
    println!("fn0");
}

pub mod mod0 {
    pub mod mod0_0 {
        pub fn fn0() {
            println!("inner fn0");
        }

        pub fn fn1() {
            println!("inner fn1");
        }

        pub fn fn2() {
            println!("inner fn2");
        }
    }
}