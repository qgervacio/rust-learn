/*
Ownership is Rust’s most unique feature, and it enables 
Rust to make memory safety guarantees without needing a 
garbage collector. Therefore, it’s important to understand 
how ownership works in Rust. In this chapter, we’ll talk 
about ownership as well as several related features: borrowing, 
slices, and how Rust lays data out in memory.

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
*/
fn main() {
    println!("\nwhat_is_ownership()");
    what_is_ownership();

    println!("\nreferences_and_borrowing()");
    references_and_borrowing();
}

fn references_and_borrowing() {
    // pass a reference. s1 is still valid
    // in nside main() since ownership is not 
    // passed to fn1. this is called borrowing
    let s1 = String::from("hello");
    fn fn1(s: &String) -> usize { s.len() }
    println!("{}", fn1(&s1)); // 5
    println!("{}", s1); // hello

    // also, you cannot modify what you borrowed
    let s2 = String::from("hello");
    // fn fn2(s: &String) { s.push_str("test"); } compile error. really nice!
    // you can't modify what you borrowed!

    // but if you really want to modify it
    let mut s3 = String::from("hello");
    fn fn2(s: &mut String) { 
        s.push_str("test"); 
    };
    fn2(&mut s3);
    println!("{}", s3); // hellotest

    // But mutable references have one big restriction: you can 
    // have only one mutable reference to a particular piece of 
    // data in a particular scope.
}

fn what_is_ownership(){
    fn transfer_of_ownership(){
        let a = String::from("test");
        fn s(s: String) { println!("{}", s); } // test
        s(a);

        // compile error. a is now invalid inside main()
        // since the reference pointer is "moved" to s()
        // println!("{}", a); borrow of moved value: `a`

        // this is ok since it's a copy
        let b = 1;
        fn i(i: i32) { println!("{}", i); } // 1
        i(b);
        println!("{}", b); // 1
    }
    transfer_of_ownership();

    fn return_of_ownership() {
        let mut a = String::from("test");
        fn s(s: String) -> String { 
            println!("{}", s); // test
            s
        }
        a = s(a);
        println!("{}", a); // test
    }
    return_of_ownership();
}
