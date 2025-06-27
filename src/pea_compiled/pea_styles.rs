#![allow(non_snake_case)]

use core::fmt;


#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy , PartialEq)]
pub enum PeaStyle{
    BOLD,
    DIM,
    ITALIC,
    UNDERLINE,
    BLINK,
    REVERSE,
    HIDDEN,
    STRIKETHROUGH,
    RESET,
}


impl PeaStyle {
    pub fn get_code(&self) -> u8 {
        match self {
            PeaStyle::BOLD         => 1,
            PeaStyle::DIM          => 2,
            PeaStyle::ITALIC       => 3,
            PeaStyle::UNDERLINE    => 4,
            PeaStyle::BLINK        => 5,
            PeaStyle::REVERSE      => 7,
            PeaStyle::HIDDEN       => 8,
            PeaStyle::STRIKETHROUGH=> 9,
            PeaStyle::RESET        => 0, // Default style
        }
    }
}

impl fmt::Display for PeaStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.get_code();
        write!(f, " {} ", code)
    }
    
}

impl std::convert::From<&str> for  PeaStyle{
        fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bold"         => PeaStyle::BOLD,
            "dim"          => PeaStyle::DIM,
            "italic"       => PeaStyle::ITALIC,
            "underline"    => PeaStyle::UNDERLINE,
            "blink"        => PeaStyle::BLINK,
            "reverse"      => PeaStyle::REVERSE,
            "hidden"       => PeaStyle::HIDDEN,
            "strikethrough"=> PeaStyle::STRIKETHROUGH,
            _              => PeaStyle::RESET, // Default to RESET if no match
        }
    }   
}


