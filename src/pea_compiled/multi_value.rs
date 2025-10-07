// Define a generic MultiValue enum
#[derive(Clone, Debug, PartialEq)]
pub enum MultiValue<A, B> {
    First(A),
    Second(B),
}

impl<A, B> MultiValue<A, B> {
    // Method to get a reference to the first value if it exists
    pub fn first(&self) -> Option<&A> {
        match self {
            MultiValue::First(a) => Some(a),
            _ => None,
        }
    }

    // Method to get a reference to the second value if it exists
    pub fn second(&self) -> Option<&B> {
        match self {
            MultiValue::Second(b) => Some(b),
            _ => None,
        }
    }

    pub fn is_first(&self) -> bool {
        matches!(self, MultiValue::First(_))
    }

    pub fn is_second(&self) -> bool {
        matches!(self, MultiValue::Second(_))
    }
}
