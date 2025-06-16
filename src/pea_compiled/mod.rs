use regex::Regex;

use crate::pea_compiled::pea_styles::PeaStyle;

pub mod pea_color;
pub mod pea_styles;


pub struct PeaCompiled {
    pub modifier: String,
    pub foreground: Option<(u8, u8, u8)>, // RGB values for foreground color
    pub background: Option<(u8, u8, u8)>, // RGB values for background color
    pub styles: Vec<pea_styles::PeaStyle>,
}

impl PeaCompiled {
    pub fn from_modifier(modifier:&String) -> Self {

        //now modifier will be compiled to get colors and styles
        let re = Regex::new(r"\([^)]+\)|\b[a-zA-Z_]+\b").unwrap();
        let args: Vec<String> = re
            .find_iter(modifier)
            .map(|mat| mat.as_str().trim().to_string())
            .collect();

        // Process the args to extract foreground, background, and styles
        let mut foreground: Option<(u8,u8,u8)> = None;
        let mut background: Option<(u8,u8,u8)> = None;
        let mut styles: Vec<pea_styles::PeaStyle> = Vec::new();
   
        //firt two argumeny are colors, then styles :  [foreground, background, styles...]
        for arg in args {


           //color set  
           if foreground.is_none() || background.is_none(){

                let color = match pea_color::PeaColor::from(arg.as_str()).rgb()  {
                   Ok(rgb) => Some(rgb),
                   Err(_) => None, // Skip if not a valid color
                };

                if foreground.is_none() {
                    foreground = color;
                }else{
                    background = color;
                }
           }
             
            //now extrcating styles
           let style =  PeaStyle::from(arg.as_str());
              if style  !=  PeaStyle::RESET {
                styles.push(style);
              }

        }
       

        PeaCompiled {
            modifier: modifier.clone(),
            foreground:  foreground,
            background: background,
            styles: styles ,
        }

    }
}