
#[macro_export]
macro_rules! peacock {
    // Handle format string with arguments
    ($fmt:expr, $($arg:expr),*) => {
        {
            let formatted = format!($fmt, $($arg),*);
            $crate::myprintln_impl(&formatted);
        }
    };
    // Handle format string without arguments
    ($msg:expr) => {
        $crate::myprintln_impl($msg);
    };

}

pub fn myprintln_impl(input: &str) {
    let output = parse_custom_format(input);
    println!("{}", output);
}


fn parse_custom_format(input: &str) -> String {
    use regex::Regex;

    let styles = [
        ("green", "\x1b[32m"),
        ("red", "\x1b[31m"),
        ("yellow", "\x1b[33m"),
        ("blue", "\x1b[34m"),
        ("magenta", "\x1b[35m"),
        ("cyan", "\x1b[36m"),
        ("white", "\x1b[37m"),

    ];
  
   let mut result = input.to_string();

   for (key, code) in styles.iter() {
        // Handle variables: yellow({}) → \x1b[33m{}\x1b[0m
        let variable_pattern = Regex::new(&format!(r#"{0}\(\{{\}}\)"#, key)).unwrap();
        result = variable_pattern
            .replace_all(&result, format!("{code}{{}}\x1b[0m"))
            .to_string();
            
        // Handle static text: yellow(text) → \x1b[33mtext\x1b[0m  
        let static_pattern = Regex::new(&format!(r#"{0}\((.*?)\)"#, key)).unwrap();
        result = static_pattern
            .replace_all(&result, format!("{code}$1\x1b[0m"))
            .to_string();
    }

    result
}


