fn main() {
    println!("\ndefining_an_enum");
    defining_an_enum();

    println!("\nthe_match_control_flow_operator");
    the_match_control_flow_operator();

    println!("\nconcise_control_flow_with_if_let");
    concise_control_flow_with_if_let();
}

fn concise_control_flow_with_if_let() {
    enum Console { PS5, Xbox };
    let fave = Console::PS5;
    if let Console::PS5 = fave {
        println!("Fave!");
    }
    if let Console::Xbox = fave {
        println!("Not a fave but why not?!"); // will not show
    }    
}

fn the_match_control_flow_operator() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    };

    fn value_i_cennts(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter =>  {
                print!("additionnal stuff to do: ");
                25
            },
        }
    }

    println!("{}", value_i_cennts(&Coin::Penny)); // 1
    println!("{}", value_i_cennts(&Coin::Nickel)); // 5
    println!("{}", value_i_cennts(&Coin::Dime)); // 10
    println!("{}", value_i_cennts(&Coin::Quarter)); // additionnal stuff to do: 25
}

fn defining_an_enum() {
    // since i learned struct, i would probably do this
    #[derive(Debug)]
    enum IpAddrKind { V4 };

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        ip: String
    };
    let ip1 = IpAddr {
        kind: IpAddrKind::V4,
        ip: String::from("127.0.0.1")
    };
    println!("{:?}", ip1); // IpAddr { kind: V4, ip: "127.0.0.1" }

    // but a more concise way would be like this
    #[derive(Debug)]
    enum IpAddrKindCon {
        V4(u32, u32, u32, u32),
        V6(String)
    };
    let ip2 = IpAddrKindCon::V4(127, 0, 0, 1);
    let ip3 = IpAddrKindCon::V6(String::from("::1"));
    println!("{:?}", ip2); // V4(127, 0, 0, 1)
    println!("{:?}", ip3); // V6("::1")

    // you can also have a method for enum
    impl IpAddrKindCon {
        fn show(&self) {}
    };
    ip3.show();

    // yo yo yo Rust do not ave null! Options to the rescue!
    let op1: Option<u32> = Some(5);
    println!("{}", op1.unwrap()); // 5

    fn fun_enum() {
        #[derive(Debug)]
        struct Person {
            name: String,
            lambo: Option<String>
        }

        let p1 = Person { 
            name: String::from("P1"), 
            lambo: Some(String::from("Fire"))
        };
        println!("{:?}", p1); // Person { name: "P1", lambo: Some("Fire") }
        println!("{}", p1.lambo.unwrap_or_default());

        let p2 = Person { 
            name: String::from("P1"), 
            lambo: None
        };
        println!("{:?}", p2); // Person { name: "P1", lambo: None }
    }
    fun_enum();
}