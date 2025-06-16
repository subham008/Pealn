#![allow(non_snake_case)]


#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
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



pub fn parse_pea_style(input: &str) -> Vec<PeaStyle> {
   let re_word = regex::Regex::new(r"\b[a-zA-Z_]+\b").unwrap();
    let re_rgb = regex::Regex::new(r"\(\d{1,3},\d{1,3},\d{1,3}\)").unwrap();

    let mut styles = Vec::new();

    for mat in re_word.find_iter(input) {
        let word = mat.as_str();
        if !re_rgb.is_match(word) {
            styles.push(PeaStyle::from(word));
        }
    }

    styles
}


pub fn get_codes(styles: &Vec<PeaStyle>) -> String {
   
    let mut codes = Vec::new();
    for style in styles {
        codes.push(style.get_code().to_string());
    }
    codes.join(";")
}