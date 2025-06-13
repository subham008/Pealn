#![allow(non_snake_case)]

use std::fmt;

#[allow(non_snake_case)]
pub struct PeaParsed {
     pub startIndex: usize,
     pub endIndex: usize,
     pub modifier:String,
     pub value: String
}

impl fmt::Display for PeaParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the output as needed
        write!(f, "PeaParse value: {}", self.value)
    }
}
