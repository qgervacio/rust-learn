mod my;

// or just my::mod0::mod0_0
pub use crate::my::mod0::mod0_0;

use my::mod0::mod0_0::fn0 as other_fn0;
use my::mod0::mod0_0::{
    fn1, fn2
};

fn main() {
    my::fn0(); // fn0

    my::mod0::mod0_0::fn0(); // inner fn0
    mod0_0::fn0(); // inner fn0
    other_fn0(); // inner fn0
    fn1(); // inner fn1
    fn2(); // inner fn2
}
