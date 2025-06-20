use regex::Regex;

use crate::pea_compiled::pea_styles::PeaStyle;

pub mod pea_color;
pub mod pea_styles;


#[derive(Debug, Clone)]
pub struct PeaCompiled {
    pub foreground: Option<(u8, u8, u8)>, // RGB values for foreground color
    pub background: Option<(u8, u8, u8)>, // RGB values for background color
    pub styles: Vec<pea_styles::PeaStyle>,
}



fn panic_peacock_error(arg: &str, code: &str) {
    // ANSI escape codes for colors
    let red = "\x1b[38;2;255;0;0m";
    let yellow = "\x1b[38;2;255;255;0m";
    let cyan = "\x1b[38;2;0;255;255m";
    let reset = "\x1b[0m";
    
    panic!(
        "{}pealn error{}: {}invalid argument{} {}` {} `{} {}at{} {}{}{}",
        red, reset,
        yellow, reset,
        cyan, arg, reset,
        yellow, reset,
        cyan, code, reset
    );
}

impl PeaCompiled {
    pub fn from_modifier(modifier:&String , full_code:&String) -> Self {

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
                //now extracting styles
              let style =  PeaStyle::from(arg.as_str());
              if style  !=  PeaStyle::RESET {
                styles.push(style);
              }
              else {
                  panic_peacock_error(&arg, &full_code);
              }
            
           }
            
        } //end of loop
       

        PeaCompiled {
            foreground:  foreground,
            background: background,
            styles: styles ,
        }

    }
}