

use regex::Regex;


pub mod pea_color;
pub mod pea_styles;

#[derive(Debug, Clone)]
pub struct PeaCompiled {
    pub foreground: Option<(u8, u8, u8)>, // RGB values for foreground color
    pub background: Option<(u8, u8, u8)>, // RGB values for background color
    pub styles: Vec<pea_styles::PeaStyle>, // all styles applied
}


enum PealnError {
    InvalidArgument,
    Repeated,
}


fn panic_pealn_error(error:PealnError ,arg: &str, code: &str) {
   
    match error {
        PealnError::InvalidArgument => {
            panic!( "pealn error : invalid argument ` {} ` at {}",  arg,   code  );
        },
        PealnError::Repeated => {
            panic!( "pealn error : repeated argument ` {} ` at {}",  arg,   code   );
        },
        
    }
   
}

impl PeaCompiled {
    pub fn from_modifier(modifier:&String , full_code:&String) -> Self {

        //now modifier will be compiled to get colors and styles
        let re = Regex::new(r"\((?:[^\(\)]|(?R))*\)|[^,\[\]\s][^,\[\]]*").unwrap();
        let args: Vec<String> = re
            .find_iter(modifier)
            .map(|mat| mat.as_str().trim().to_string())
            .collect();

        // Process the args to extract foreground, background, and styles
        let mut foreground: Option<(u8,u8,u8)> = None;
        let mut background: Option<(u8,u8,u8)> = None;
        let mut styles: Vec<pea_styles::PeaStyle> = Vec::new();
       
   
        //firt two argumeny are colors, then styles :  [foreground, background, link(Crates.io) styles...] if something like link is found it will be called as modifier
        for arg in args {
          
          if let Some(color) = pea_color::PeaColor::from(arg.as_str()){
                if foreground.is_none() {
                    foreground = Some(color.rgb());
                }
                else if background.is_none() {
                    background = Some(color.rgb());
                }
                else {
                    panic_pealn_error( PealnError::Repeated,&arg, &full_code);
                }
          }
          else if let Some(style) = pea_styles::PeaStyle::from(arg.as_str()){
                 styles.push(style); 
          }
        
          else {
               panic_pealn_error( PealnError::InvalidArgument,&arg, &full_code);
          }

            
    } //end of loop
             
       //TODO  return valid modifiers
        PeaCompiled {
            foreground:  foreground,
            background: background,
            styles: styles ,
        }

    }
    

    //it retunrns format : // "1;2;3" for styles
    pub fn get_style_coded(&self) -> String {
        
      let mut codes = Vec::new();
      for style in self.styles.iter() {
         codes.push(style.get_code().to_string());
       }
       codes.join(";")
    }



}