// same level with eat_at_restaurant so 
// it does nont have to be pub-ed
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    serve_order();

    // by default it's private
    struct A {
        pub a: String,
        b: String
    };
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::serve_order();
    }
}