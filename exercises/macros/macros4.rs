// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($val:expr) => {
        println!("Look at this macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    let x = 7777;
    my_macro!(x);
}
