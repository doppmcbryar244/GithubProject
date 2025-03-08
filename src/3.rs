use std::fmt;

struct Foo {
    bar: i32,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bar)
    }
}
