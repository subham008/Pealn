use regex::Regex;

pub mod pea_parse;
pub mod pea_compiled;

use crate::pea_compiled::PeaCompiled;
use crate::pea_compiled::pea_styles::{PeaStyle , get_codes};

#[macro_export]
macro_rules! pealn {
    // Handle format string with arguments
    ($fmt:expr, $($arg:expr),*) => {
        {
            let formatted = format!($fmt, $($arg),*);
            $crate::peacock_impl(&formatted , true);
        }
    };
    // Handle format string without arguments
    ($msg:expr) => {
        $crate::peacock_impl($msg , true);
    };

}


#[macro_export]
macro_rules! pea {
    // Handle format string with arguments
    ($fmt:expr, $($arg:expr),*) => {
        {
            let formatted = format!($fmt, $($arg),*);
            $crate::peacock_impl(&formatted , false);
        }
    };
    // Handle format string without arguments
    ($msg:expr) => {
        $crate::peacock_impl($msg , false);
    };

}

pub fn peacock_impl(input: &str , ln:bool) {
    let output = parse_peacock_format(input);
    if  ln {
        println!("{}", output);
    } else {
        print!("{}", output);
        
    }
}





fn parse_peacock_format(input: &str) -> String {
  
   let mut result = input.to_string();
    
    let re = Regex::new(r"\[([^\]]*)\]\(([^)]*)\)").unwrap();
    
    // Iterate over all captures and create PeaParse list
    let mut parse_list:Vec<pea_parse::PeaParsed> = Vec::new();

    for cap in re.captures_iter(&result) {
      let full_match = cap.get(0).unwrap(); // Get the full match to access start/end indices
      parse_list.push(
        pea_parse::PeaParsed {
            startIndex: full_match.start(),
            endIndex: full_match.end(),
            fullMatch: full_match.as_str().to_string(),
            modifier: cap[1].to_string(),
            value: cap[2].to_string()
          }
       );
   } 

    //now format the result string
    let mut formatted_result:Vec<(&pea_parse::PeaParsed ,String)> = Vec::new();
    
    for parsed in &parse_list {
       
       let pea_compiled = PeaCompiled::from_modifier(&parsed.modifier);


        let style_vec : Vec<PeaStyle> = pea_compiled.styles;
        let pea_styles =get_codes(&style_vec);

        let (r,g,b) =match pea_compiled.foreground {
            Some((r, g, b)) => (r, g, b),
            None => (255,255,255), // panic if the color is invalid
        };

        formatted_result.push((parsed ,format!("\x1b[{};38;2;{};{};{}m{}\x1b[0m",pea_styles ,r, g, b, parsed.value)));
    }


    

    // Replace the original formatted parts in the result string
    for (parsed, formatted) in formatted_result.iter().rev() {
        let start = parsed.startIndex;
        let end = parsed.endIndex;
        result.replace_range(start..end, &formatted);
    }
    
   result
}



