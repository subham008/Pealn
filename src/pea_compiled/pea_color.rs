#![allow(non_snake_case)]

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PeaColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
    White,
    Purple,
    Orange,
    Default,         // Default color
    RGB(u8, u8, u8), // RGB variant to hold custom RGB values
}

impl PeaColor {
    pub fn from(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "red" => Some(PeaColor::Red),
            "green" => Some(PeaColor::Green),
            "blue" => Some(PeaColor::Blue),
            "yellow" => Some(PeaColor::Yellow),
            "cyan" => Some(PeaColor::Cyan),
            "magenta" => Some(PeaColor::Magenta),
            "black" => Some(PeaColor::Black),
            "white" => Some(PeaColor::White),
            "purple" => Some(PeaColor::Purple),
            "orange" => Some(PeaColor::Orange),
            "default" => Some(PeaColor::Default),
            _ => {
                //check if it is an RGB value using regex
                let re =
                    regex::Regex::new(r"^\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$")
                        .unwrap();
                if re.is_match(s) {
                    // Extract RGB values
                    let caps = re.captures(s).unwrap();
                    let r: u8 = caps[1].parse().unwrap_or(0);
                    let g: u8 = caps[2].parse().unwrap_or(0);
                    let b: u8 = caps[3].parse().unwrap_or(0);
                    return Some(PeaColor::RGB(r, g, b)); // Return RGB variant or handle as needed
                } else {
                    None // Return None if no match found
                }
            } // default or handle error as needed
        }
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            PeaColor::Red => (255, 0, 0),
            PeaColor::Green => (0, 255, 0),
            PeaColor::Blue => (0, 0, 255),
            PeaColor::Yellow => (255, 255, 0),
            PeaColor::Cyan => (0, 255, 255),
            PeaColor::Magenta => (255, 0, 255),
            PeaColor::Black => (0, 0, 0),
            PeaColor::White => (255, 255, 255),
            PeaColor::Purple => (128, 0, 128),
            PeaColor::Orange => (235, 143, 52),
            PeaColor::Default => (0, 0, 0), // Default RGB value, can be customized
            PeaColor::RGB(r, g, b) => (r.clone(), g.clone(), b.clone()), // Default RGB value, can be customized
        }
    }
}

impl std::fmt::Display for PeaColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeaColor::Red => write!(f, "255;0;0"),
            PeaColor::Green => write!(f, "0;255;0"),
            PeaColor::Blue => write!(f, "0;0;255"),
            PeaColor::Yellow => write!(f, "255;255;0"),
            PeaColor::Cyan => write!(f, "0;255;255"),
            PeaColor::Magenta => write!(f, "255;0;255"),
            PeaColor::Black => write!(f, "0;0;0"),
            PeaColor::White => write!(f, "255;255;255"),
            PeaColor::Purple => write!(f, "128;0;128"),
            PeaColor::Orange => write!(f, "235;143;52"),
            PeaColor::Default => write!(f, "0;0;0"),
            PeaColor::RGB(r, g, b) => write!(f, "{};{};{}", r, g, b),
        }
    }
}
