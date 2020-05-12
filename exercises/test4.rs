// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        format!("Hello {}",$val)
    };
    ($val:expr) => {
        println!("Look at this macro: {}", $val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
