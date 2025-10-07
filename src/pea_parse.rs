use std::fmt;

pub struct PeaParsed {
    pub start_index: usize,
    pub end_index: usize,
    pub full_match: String, // Full match string
    pub modifier: String,
    pub value: String,
}

impl fmt::Display for PeaParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the output as needed
        write!(f, "PeaParse value: {}", self.value)
    }
}

impl std::fmt::Debug for PeaParsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeaParsed")
            .field("start_index", &self.start_index)
            .field("end_index", &self.end_index)
            .field("full_match", &self.full_match)
            .field("modifier", &self.modifier)
            .field("value", &self.value)
            .finish()
    }
}
