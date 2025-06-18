use regex::Regex;

mod pea_parse;
mod pea_compiled;

use crate::pea_compiled::PeaCompiled;
use crate::pea_compiled::pea_styles::{ get_codes};


///
///print new line  with colored and styles
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn};
/// pealn!("[yellow](Hello) [green](World)!");
/// let name  = "Subham Shaw";
/// pealn!("[yellow,bold](Name) : [bold,hidden]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn};
/// pealn!("[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn};
/// 
/// pealn!("[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn};
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```

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




///
///print on same line  with colored and styles
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn};
/// pealn!("[yellow](Hello) [green](World)!");
/// let name  = "Subham Shaw";
/// pealn!("[yellow,bold](Name) : [bold,hidden]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn};
/// pealn!("[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn};
/// 
/// pealn!("[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn};
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```
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

/// Prints the formatted string to the console.
/// If `ln` is true, it prints with a newline at the end; otherwise, it prints without a newline.
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
       
        let pea_compiled = PeaCompiled::from_modifier(&parsed.modifier ,&parsed.fullMatch);

        let pea_styles = get_codes(&pea_compiled.styles);
        
        let background_colored = match pea_compiled.background {
            Some((r, g, b)) => {
                format!("48;2;{};{};{}", r, g, b)
            },
            None => "".to_string(), // Default to black if no background color is set
        };
        
        let foreground_colored = match pea_compiled.foreground {
            Some((r, g, b)) => {
                format!("38;2;{};{};{}", r, g, b)
            },
            None => "".to_string(), // Default to black if no background color is set
        };

                                             //peastyels | ; is FG exists| foreground | ; if BG exists | background | text
        let formatted_string = format!("\x1b[{}{}{}{}{}m{}\x1b[0m",pea_styles,if pea_compiled.foreground.is_some(){";"}else{""} ,foreground_colored,if pea_compiled.background.is_some(){";"}else{""} ,background_colored, parsed.value);

        formatted_result.push((parsed ,formatted_string));
    }


    

    // Replace the original formatted parts in the result string
    for (parsed, formatted) in formatted_result.iter().rev() {
        let start = parsed.startIndex;
        let end = parsed.endIndex;
        result.replace_range(start..end, &formatted);
    }
    
   result
}



