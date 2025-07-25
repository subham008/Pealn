

use std::fmt;


pub struct PeaParsed {
     pub start_index: usize,
     pub end_index: usize,
     pub full_match: String, // Full match string
     pub modifier:String,
     pub value: String
}

impl fmt::Display for PeaParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the output as needed
        write!(f, "PeaParse value: {}", self.value)
    }
}
