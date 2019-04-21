#[macro_use]
extern crate new_debug_unreachable;

fn main() {
    println!("Hello, world!");

    if 0 > 100 {
        // Can't happen!
        unsafe { debug_unreachable!() }
    } else {
        println!("Good, 0 <= 100.");
    }
}
