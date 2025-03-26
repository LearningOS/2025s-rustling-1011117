// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.



struct Wrapper<T> {
    value: T,
}

impl Wrapper<u32> {
    pub fn new(value: u32) -> Wrapper::<u32> {
        Wrapper::<u32> { value }
    }
}
impl Wrapper<&'static str> {
    pub fn new(value: &'static str) -> Wrapper::<&'static str> {
        Wrapper::<&'static str> { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::<u32>::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::<&'static str>::new("Foo").value, "Foo");
    }
}
