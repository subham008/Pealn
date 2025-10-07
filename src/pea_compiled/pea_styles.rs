#![allow(non_snake_case)]

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeaStyle {
    BOLD,
    DIM,
    ITALIC,
    UNDERLINE,
    BLINK,
    REVERSE,
    HIDDEN,
    STRIKETHROUGH,
}

impl PeaStyle {
    pub fn from(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bold" => Some(PeaStyle::BOLD),
            "dim" => Some(PeaStyle::DIM),
            "italic" => Some(PeaStyle::ITALIC),
            "underline" => Some(PeaStyle::UNDERLINE),
            "blink" => Some(PeaStyle::BLINK),
            "reverse" => Some(PeaStyle::REVERSE),
            "hidden" => Some(PeaStyle::HIDDEN),
            "strikethrough" => Some(PeaStyle::STRIKETHROUGH),
            _ => None, // Default to RESET if no match
        }
    }

    pub fn get_code(&self) -> u8 {
        match self {
            PeaStyle::BOLD => 1,
            PeaStyle::DIM => 2,
            PeaStyle::ITALIC => 3,
            PeaStyle::UNDERLINE => 4,
            PeaStyle::BLINK => 5,
            PeaStyle::REVERSE => 7,
            PeaStyle::HIDDEN => 8,
            PeaStyle::STRIKETHROUGH => 9,
        }
    }
}

impl std::fmt::Display for PeaStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.get_code();
        write!(f, "{}", code)
    }
}
