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
    Purple,
    RGB (u8, u8, u8), // RGB variant to hold custom RGB values
    None
}

impl PeaColor {
    pub fn rgb(&self) ->Result<(u8, u8, u8), &'static str> {
        match self {
            PeaColor::Red     => Ok((255, 0, 0)),
            PeaColor::Green   => Ok((0, 255, 0)),
            PeaColor::Blue    => Ok((0, 0, 255)),
            PeaColor::Yellow  => Ok((255, 255, 0)),
            PeaColor::Cyan    => Ok((0, 255, 255)),
            PeaColor::Magenta => Ok((255, 0, 255)),
            PeaColor::Black   => Ok((0, 0, 0)),
            PeaColor::White   => Ok((255, 255, 255)),
            PeaColor::Purple  => Ok((128, 0, 128)),
            PeaColor::RGB (r,g,b)    =>Ok((r.clone(), g.clone(), b.clone())), // Default RGB value, can be customized
            PeaColor::None    => Err("Invalid color" ), // Default RGB value for None
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
            "purple"  => PeaColor::Purple,
            _         => {
                //check if it is an RGB value using regex
                let re = regex::Regex::new(r"^\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$").unwrap();
                if re.is_match(s) {
                    // Extract RGB values
                    let caps = re.captures(s).unwrap();
                    let r: u8 = caps[1].parse().unwrap_or(0);
                    let g: u8 = caps[2].parse().unwrap_or(0);
                    let b: u8 = caps[3].parse().unwrap_or(0);
                    return  PeaColor::RGB(r,g,b); // Return RGB variant or handle as needed
                }
                else {
                   
                    PeaColor::None // Return None if no match found
                }
                
            }, // default or handle error as needed
        }
    }
    
}



