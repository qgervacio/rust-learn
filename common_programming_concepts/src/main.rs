// Keep in mind that Rust is a statically typed language, 
// which means that it must know the types of all 
// variables at compile time

fn main() {
    println!("\nvariables()");
    variables();

    println!("\ndata_types()");
    data_types();

    println!("\nfunctions()");
    functions();

    println!("\ncontrol_flow()");
    control_flow();
}

fn control_flow() {
    let is_good_looking = true;
    if is_good_looking { // parenthesis is not required and must be a bool
        println!("😊");
    } else {
        println!("😔");
    } // 😊

    // Because if is an expression, we can use it on 
    // the right side of a let statement.
    // Remember that blocks of code evaluate to the last 
    // expression in them, and numbers by themselves 
    // are also expressions. Also type has to match, 
    // else it's a compile error
    let a = if is_good_looking { 5 } else { 6 }; // no ternary operator in Rust
    println!("a is {}", a); // a is 5

    // `if` and `else` have incompatible types
    // let b = if is_good_looking { 5 } else { "no" };

    // Rust has three kinds of loops: loop, while, and for
    let mut a_loop = 3;
    loop {
        if a_loop > 0 {
            println!("a_loop is {}", a_loop); // a_loop is 3..1
            a_loop = a_loop - 1;
        } else {
            break;
        }
    }

    // Returning Values from Loops
    let mut counter = 0;
    let a_loop_return = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("a_loop_return is {}", a_loop_return); // a_loop_return is 20

    let mut a_while = 3;
    while a_while > 0 {
        println!("a_while is {}", a_while); // a_while is 3..1
        a_while = a_while - 1;
    }

    let arr0 = [1, 2, 3];
    for e in arr0.iter() {
        println!("e is {}", e); // e is 1..3
    }

    // reversed loop
    for e in (1..3).rev() {
        println!("e is {}", e); // e is 3..1
    }
}

fn functions() {
    // In function signatures, you must declare the type of each 
    // parameter. This is a deliberate decision in Rust’s design: 
    // requiring type annotations in function definitions means
    // the compiler almost never needs you to use them elsewhere
    // in the code to figure out what you mean.
    add(1, 2); // 1 + 2 = 3
    fn add(a: i32, b: i32) {
        println!("{} + {} = {}", a, b, a+b);
    }

    // The let y = 6 statement does not return a value, so there 
    // isn’t anything for x to bind to. This is different from 
    // what happens in other languages, such as C and Ruby, 
    // where the assignment returns the value of the assignment
    // let x = (let y = 6);; compile error

    // Expressions evaluate to something and make up most of 
    // the rest of the code that you’ll write in Rust
    let a = { // an expression block
        let b = 1;
        let c = 2;
        b + c
    };
    println!("a is {}", a); // a is 3

    // Expressions do not include ending semicolons. 
    // If you add a semicolon to the end of an expression, 
    // you turn it into a statement
    let b = { 1 + 1 };
    println!("b is {}", b); // b is 2

    // In Rust, the return value of the function is synonymous 
    // with the value of the final expression in the block of 
    // the body of a function. You can return early from a function 
    // by using the return keyword and specifying a value, but most 
    // functions return the last expression implicitly
    fn five() -> i32 { 
        6; // now this becomes a statement
        5
    }
    println!("five() is {}", five()); // five() is 5
}

fn data_types() {
    // converting a String to integer
    let x: u32 = "4".parse().unwrap();
    println!("x is {}", x);

    // why type annotation is still nedeed?
    // this is because parse() is so general
    // see doc of parse()
    let y: u32 = "42".parse().expect("Not a number!");
    println!("y is {}", y);

    // now annotation type is not needed because 
    // you helped the type inference algorithm using
    // 'turbofish' ::<>
    let z = "4".parse::<u32>();
    println!("z is {}", z.unwrap()); // TODO: why unwrap is needed?

    // A scalar type represents a single value. Rust has four 
    // primary scalar types: integers, floating-point numbers, 
    // Booleans, and characters. You may recognize these 
    // from other programming languages. Let’s jump into 
    // how they work in Rust.
    let a_i8: i8 = 1;
    let b_i16: i16 = 1;
    let c_i32: i32 = 1;
    let d_i64: i64 = 1;
    let e_i128: i128 = 1;
    let f_isize: isize = 1;
    let g_u8: u8 = 1;
    let h_u16: u16 = 1;
    let i_u32: u32 = 1;
    let j_u64: u64 = 1;
    let k_u128: u128 = 1;
    let l_usize: usize = 1_000_000; // like 1,000,000
    println!("a_i8 is {}", a_i8);
    println!("b_i16 is {}", b_i16);
    println!("c_i32 is {}", c_i32);
    println!("d_i64 is {}", d_i64);
    println!("e_i128 is {}", e_i128);
    println!("f_isize is {}", f_isize);
    println!("g_u8 is {}", g_u8);
    println!("h_u16 is {}", h_u16);
    println!("i_u32 is {}", i_u32);
    println!("j_u64 is {}", j_u64);
    println!("k_u128 is {}", k_u128);
    println!("l_usize is {}", l_usize);

    // So how do you know which type of integer to use? If you’re unsure, 
    // Rust’s defaults are generally good choices, and integer types 
    // default to i32: this type is generally the fastest, even on 64-bit
    // systems. The primary situation in which you’d use isize or usize 
    // is when indexing some sort of collection.
    // SEE: ch03-02-data-types.html#integer-overflow
    // the result may vary whether your build with --debug or --release
    
    let a_f32: f32 = 1.2;
    let b_f32: f64 = 1.2;
    println!("a_f32 is {}", a_f32); // a_f32 is 1.2
    println!("b_f32 is {}", b_f32); // b_f32 is 1.2

    println!("1 + 1 = {}", 1+1); // 1 + 1 = 2

    println!("Am i good looking? {}", true); // Am i good looking? true

    println!("{} goes meow!", "😻"); // 😻 goes meow!
    println!("'z' as {}", 'z'); // 'z' as z

    // Compound types can group multiple values into one type. 
    // Rust has two primitive compound types: tuples and arrays.

    // A tuple is a general way of grouping together a number 
    // of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot 
    // grow or shrink in size.
    
    let (a0, b1, c2) = (1, 1.2, true); // un-named tuple
    println!("a0={}, b1={}, c2={}", a0, b1, c2); // a0=1, b1=1.2, c2=true

    let tup0: (i32, f64, bool) = (1, 1.2, true);
    println!("tup0.0 is {}", tup0.0); // tup0.0 is 1
    println!("tup0.1 is {}", tup0.1); // tup0.1 is 1.2
    println!("tup0.2 is {}", tup0.2); // tup0.2 is true

    // or assign to variable
    let (tup0_0, _, _) = tup0; // i don't care about the rest
    println!("tup0_0 is {}", tup0_0); // tup0_0 is 1

    // Another way to have a collection of multiple values is 
    // with an array. Unlike a tuple, every element of an array 
    // must have the same type. Arrays in Rust are different from 
    // arrays in some other languages because arrays in Rust have 
    // a fixed length, like tuples.

    let arr0 = [1, 2, 3];
    println!("arr0[0] is {}", arr0[0]); // arr0[0] is 1

    let arr1: [f32; 3] = [1.0, 2.0, 3.0];
    println!("arr1[1] is {}", arr1[1]); // arr1[1] is 2

    // both are the same
    let arr2 = [3; 5];
    let arr3 = [3, 3, 3, 3, 3];
    println!("arr2[1] is {}", arr2[1]); // arr3[1] is 3
    println!("arr3[1] is {}", arr3[1]); // arr3[1] is 3
}

fn variables() {
    // once a variable is declared it is immutable by default
    let x = 5;
    // x = 3; cannot assign twice to immutable variable `x`
    println!("x is {}", x);

    // unless of course if you make it mutable
    let mut y = 1;
    println!("y is {}", y);
    y = 2;
    println!("y is now {}", y);

    // constants may be set only to a constant expression, 
    // not the result of a function call or any other value 
    // that could only be computed at runtime.
    // type is required
    const Z:u32 = 5;
    println!("Z is {}", Z);
}
