use std::fmt;

struct DisplayableThing {
    first_value: String,
    second_value: String,
}

impl fmt::Display for DisplayableThing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first_value, self.second_value)
    }
}