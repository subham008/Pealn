use regex::Regex;

use std::fmt;
use crate::pea_compiled::pea_styles::PeaStyle;

pub mod pea_color;
pub mod pea_styles;


#[derive(Debug, Clone)]
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
          let mut  color:Option<(u8,u8,u8)> = None;

          if background.is_none() || foreground.is_none(){
              match pea_color::PeaColor::from(arg.as_str()).rgb() {
                  Ok(rgb) => { color = Some(rgb); },
                  Err(_)=>{ color = None; }
              }
          }

            if foreground.is_none() && color.is_some() {
                foreground = color;
            }
            else if background.is_none() && color.is_some() {
                background = color;
             }
            else 
            {
                //now extrcating styles
              let style =  PeaStyle::from(arg.as_str());
              if style  !=  PeaStyle::RESET {
                styles.push(style);
              }
            
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