use regex::Regex;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Token, LitStr, parse::{Parse , ParseStream}, punctuated::Punctuated};
use quote::quote;


struct PrintlnInput {
    fmt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for PrintlnInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fmt: LitStr = input.parse()?;
        let mut args = Punctuated::new();

        while input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
            let expr: Expr = input.parse()?;
            args.push(expr);
        }

        Ok(PrintlnInput { fmt, args })
    }
}


mod pea_parse;
mod pea_compiled;

use crate::pea_compiled::PeaCompiled;


///
///print new line  with colored and styles
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, purple , magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// #### *Some Styles are not supported in all terminals
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn};
/// let name  = "Subham Shaw";
/// pealn!("[yellow](Name) : [green]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn};
/// pealn!("[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// #### *First defined color will be used as foreground and second as background
/// 
/// 
/// you can use RGB color 
/// 
///```rust
/// use pealn::{pealn};
/// pealn!("[(25,45,78)](Hello) [(34,67,78)](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn};
/// 
/// pealn!("[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn};
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```


#[proc_macro]
pub fn pealn(item: TokenStream) -> TokenStream {
     let PrintlnInput { fmt , args } = parse_macro_input!(item as PrintlnInput);

    let pea_code =fmt.value();
    
    let formatted = parse_pealn_format(&pea_code);

    let expanded = quote! {
        println!(#formatted, #args);
    };

    expanded.into()
}


///
///print on same line  with colored and styles
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, purple ,  magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// #### *Some Styles are not supported in all terminals
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn_write};
/// let name  = "Subham Shaw";
/// pealn_write!("[yellow](Name) : [green]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn_write};
/// pealn_write!("[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// 
/// #### *First defined color will be used as foreground and second as background
/// 
/// 
/// you can use RGB color 
/// 
///```rust
/// use pealn::{pealn_write};
/// pealn_write!("[(25,45,78)](Hello) [(34,67,78)](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn_write};
/// 
/// pealn_write!("[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn_write};
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn_write!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```
/// 
#[proc_macro]
pub fn pea(item: TokenStream) -> TokenStream {
    let PrintlnInput { fmt , args } = parse_macro_input!(item as PrintlnInput);
    
    let pea_code =fmt.value();
    
    let formatted = parse_pealn_format(&pea_code);

    let expanded = quote! {
        print!(#formatted, #args);
    };

    expanded.into()
}






struct WriteInput {
    writer: Expr,
    fmt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for WriteInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let writer: Expr = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let fmt: LitStr = input.parse()?;
        let mut args = Punctuated::new();

        while input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
            let expr: Expr = input.parse()?;
            args.push(expr);
        }

        Ok(WriteInput { writer, fmt, args })
    }
}



///
///pealn_write is an alternative to write! macro.
///Writes formatted data into a buffer.
/// 
///This macro accepts a 'writer', a format string, and a list of arguments.
///Arguments will be formatted according to the specified format string and the result will be passed to the writer. 
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, purple ,  magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// #### *Some Styles are not supported in all terminals
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn_write};
/// let name  = "Subham Shaw";
/// let f = std::io::stdout();
/// pealn_write!(f,"[yellow](Name) : [green]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn_write};
/// let f = std::io::stdout();
/// pealn_write!(f,"[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// 
/// #### *First defined color will be used as foreground and second as background
/// 
/// 
/// you can use RGB color 
/// 
///```rust
/// use pealn::{pealn_write};
/// let f = std::io::stdout();
/// pealn_write!(f,"[(25,45,78)](Hello) [(34,67,78)](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn_write};
/// let f = std::io::stdout();
/// pealn_write!(f,"[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn_write};
/// let f = std::io::stdout();
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn_write!(f,"[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```
/// 
#[proc_macro]
pub fn pealn_write( item: TokenStream) -> TokenStream {
    let WriteInput { writer, fmt, args } = parse_macro_input!(item as WriteInput);

    let pea_code = fmt.value();
    let formatted = parse_pealn_format(&pea_code);

    let expanded = quote! {
        write!(#writer, #formatted, #args)
    };

    expanded.into()

}


///
///pealn_format! is an alternative to format! macro.
///Creates a String using interpolation of runtime expressions.
///The first argument format! receives is a format string. This must be a string literal.
///  The power of the formatting string is in the {}s contained.
///  Additional parameters passed to format! replace the {}s within the formatting string in the order given unless named or positional parameters are used.
/// ## Format
/// 
/// [[foreground,background,styles....]](text)
/// 
/// ### Available Colors 
/// red, green, blue, yellow, cyan, purple , magenta, black, white
/// 
/// 
/// ### Available Styles 
/// bold, dim, italic, underline, blink, reverse, hidden, strikethrough
/// 
/// #### *Some Styles are not supported in all terminals
/// 
/// ## Examples
/// 
/// To print text with foreground
/// ```
/// use pealn::{pealn_format};
/// let name  = "Subham Shaw";
/// pealn_format!("[yellow](Name) : [green]({}) " , name );
/// ```
/// 
/// To print text with foreground and background
/// ```
/// use pealn::{pealn_format};
/// pealn_format!("[yellow,white](Hello) [green,white](World)!");
/// ```
/// 
/// #### *First defined color will be used as foreground and second as background
/// 
/// 
/// you can use RGB color 
/// 
///```rust
/// use pealn::{pealn_format};
/// pealn_format!("[(25,45,78)](Hello) [(34,67,78)](World)!");
/// ```
/// 
/// To print text with styles
/// ```
/// use pealn::{pealn_format};
/// 
/// pealn_format!("[bold,underline](Hello) [italic](World)!");
/// ```
/// 
/// 
/// To print text with color and styles
/// ```
/// use pealn::{pealn_format};
/// //here order of colors and styles does not matter, 
/// //first color will be used as foreground and second as background
/// pealn_format!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
/// ```
#[proc_macro]
pub fn pealn_format(  item: TokenStream) -> TokenStream {
   let PrintlnInput { fmt , args } = parse_macro_input!(item as PrintlnInput);
    
    let pea_code =fmt.value();
    
    let formatted = parse_pealn_format(&pea_code);

    let expanded = quote! {
        format!(#formatted, #args)
    };

    expanded.into()
}





fn parse_pealn_format(input: &str) -> String {
  
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
        
        let mut prefix  = String::new();
        let mut suffix  = String::new();
       
        let pea_compiled = PeaCompiled::from_modifier(&parsed.modifier ,&parsed.fullMatch);
        

        prefix.push_str("\x1b["); // ANSI escape code prefix
        //add styles to the prefix
        if !pea_compiled.styles.is_empty() {
            prefix.push_str(&pea_compiled.get_style_coded());
        }  

        //adding foreground color to the prefix

        if let Some((r, g, b)) = pea_compiled.foreground {
            if !pea_compiled.styles.is_empty() {
                prefix.push(';'); // Add a semicolon if styles are present
            }
            prefix.push_str(&format!("38;2;{};{};{}", r, g, b));
        }

        //adding background color to the prefix
        if let Some((r, g, b)) = pea_compiled.background {
            if !pea_compiled.styles.is_empty() || pea_compiled.foreground.is_some()  {
                prefix.push(';'); // Add a semicolon if styles are present
            }
            prefix.push_str(&format!("48;2;{};{};{}", r, g, b));
        }
        prefix.push('m'); // ANSI escape code suffix
        suffix.push_str("\x1b[0m"); // ANSI escape code prefix
    
                                             //peastyels | ; is FG exists| foreground | ; if BG exists | background | text
        let formatted_string = format!("{} {} {}", prefix, parsed.value, suffix);

        formatted_result.push((parsed ,formatted_string));
    } // end of for loop

    // Replace the original formatted parts in the result string
    for (parsed, formatted) in formatted_result.iter().rev() {
        let start = parsed.startIndex;
        let end = parsed.endIndex;
        result.replace_range(start..end, &formatted);
    }
    
   result
}



