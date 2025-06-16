use regex::Regex;

pub mod pea_color;
pub mod pea_styles;


pub struct PeaCompiled {
    pub modifier: String,
    pub foreground: Option<pea_color::PeaColor>,
    pub background: Option<pea_color::PeaColor>,
    pub styles: Vec<pea_styles::PeaStyle>,
}

impl PeaCompiled {
    pub fn from_modifier(modifier:&String) -> Self {

        //now modifier will be compiled to get colors and styles

        //FIX ME 
        PeaCompiled {
            modifier: String::new(),
            foreground: None,
            background: None,
            styles: Vec::new(),
        }

    }
}