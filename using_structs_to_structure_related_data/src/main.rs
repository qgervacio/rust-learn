fn main() {
    println!("\ndefining_and_instantiating_structs");
    defining_and_instantiating_structs();

    println!("\nan_example_program_using_structs");
    an_example_program_using_structs();

    println!("\nmethod_syntax");
    method_syntax();
}

fn method_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };

    // this is the method.
    // Methods are similar to functions: they’re declared with 
    // the fn keyword and their name, they can have parameters
    // and a return value, and they contain some code that is 
    // run when they’re called from somewhere else. However, 
    // methods are different from functions in that they’re 
    // defined within the context of a struct (or an enum or 
    // a trait object)
    impl Rectangle { // must be same name as the struct
        fn calculate(&self) -> u32 {
            self.width * self.height
        }

        fn can_fit(&self, others: &Rectangle) -> bool {
            self.width >= others.height && self.height >= others.height
        }

        // this is an associated function
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    };

    let rec1 = Rectangle {
        width: 10,
        height: 10
    };
    println!("{}", rec1.calculate()); // 100

    println!(
        "{:?}", 
        Rectangle{width: 10, height: 10}
            .can_fit(&Rectangle{width: 10, height: 10}) // true
    );

    // Another useful feature of impl blocks is that we’re 
    // allowed to define functions within impl blocks that 
    // don’t take self as a parameter. These are called 
    // associated functions because they’re associated with 
    // the struct. They’re still functions, not methods, 
    // because they don’t have an instance of the struct to 
    // work with. You’ve already used the String::from 
    // associated function.
    let rec2 = Rectangle::square(20);
    println!("{:?}", rec2); // Rectangle { width: 20, height: 20 }

    // Each struct is allowed to have multiple impl blocks
    impl Rectangle {
        fn another() {}
    }
}

fn an_example_program_using_structs() {

    // plain area
    fn area0(width: u32, height: u32) -> u32 {
        width * height
    }
    println!("{}", area0(10, 10)); // 100

    // area using tuples
    fn area1(dimension: (u32, u32)) -> u32 {
        dimension.0 * dimension.1
    }
    println!("{}", area1((20, 20))); // 400

    // area using tuple structs
    struct Dime0(u32, u32);
    fn area2(dimension: Dime0) -> u32 {
        dimension.0 * dimension.1
    }
    println!("{}", area2(Dime0(30, 30))); // 900

    // using structs
    #[derive(Debug)] // this allows you to print out debugging information
    struct Dime1 {
        width: u32,
        height: u32
    }
    fn area3(dimension: Dime1) -> u32 {
        dimension.width * dimension.height
    }
    let dime1 = Dime1{width: 40, height: 40};
    println!("{}", area3(dime1)); // 1600

    let dime2 = Dime1{width: 40, height: 40};

    // i can now do this because of #[derive(Debug)]
    println!("{:?}", dime2); // Dime1 { width: 40, height: 40 }
    println!("{:#?}", dime2);
    /*
    Dime1 {
        width: 40,
        height: 40,
    }
    */
}

fn defining_and_instantiating_structs() {
    struct User {
        first_name: String,
        last_name: String,
        user_name: String,
        email: String
    };

    let mut user = User {
        first_name: String::from("Quirino"),
        last_name: String::from("Gervacio"),
        user_name: String::from("qgervacio"),
        email: String::from("email"),
    };
    user.email = String::from("qgervacio@gmail.com"); // user have to be mut
    println!("first_name is {}", user.first_name); // first_name is Quirino
    println!("last_name is {}", user.last_name); // last_name is Gervacio
    println!("user_name is {}", user.user_name); // user_name is qgervacio
    println!("email is {}", user.email); // email is qgervacio@gmail.com

    // field init shorthand
    // Because the parameter names and the struct field 
    // names are exactly the same (user_name)
    fn copy_identity(user_name: String) -> User {
        User {
            user_name,
            first_name: String::from("Quirino"),
            last_name: String::from("Gervacio"),
            email: String::from("email"),
        }
    };
    let user2 = copy_identity(String::from("jun_pogi"));
    println!("{}", user2.user_name); // jun_pogi

    // struct update
    let user3 = User {
        email: String::from("email@email.com"),
        ..user2
    };
    println!("{}", user3.user_name); // jun_pogi
    println!("{}", user3.email); // email@email.com

    // tuple struts
    struct Hero(String, String);
    let superman = Hero(
        String::from("Superman"),
        String::from("Awesomeness"),
    );
    println!("My name is {}", superman.0); // My name is Superman
    println!("My super power is {}", superman.1); // My super power is Awesomeness
}