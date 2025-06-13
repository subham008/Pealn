#![allow(non_snake_case)]



#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub enum PeaColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White,
}

impl PeaColor {
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            PeaColor::Red     => (255, 0, 0),
            PeaColor::Green   => (0, 255, 0),
            PeaColor::Blue    => (0, 0, 255),
            PeaColor::Yellow  => (255, 255, 0),
            PeaColor::Cyan    => (0, 255, 255),
            PeaColor::Magenta => (255, 0, 255),
            PeaColor::Black   => (0, 0, 0),
            PeaColor::White   => (255, 255, 255),
        }
    }
}


impl std::convert::From<&str> for PeaColor {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "red"     => PeaColor::Red,
            "green"   => PeaColor::Green,
            "blue"    => PeaColor::Blue,
            "yellow"  => PeaColor::Yellow,
            "cyan"    => PeaColor::Cyan,
            "magenta" => PeaColor::Magenta,
            "black"   => PeaColor::Black,
            "white"   => PeaColor::White,
            _         => PeaColor::White, // default or handle error as needed
        }
    }
}
