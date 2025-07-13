use quote::ToTokens;



pub enum  PealnColorMode{
     ALWAYS, // print with color always
     NEVER, // print without color always
     AUTO // print with color if terminal supports it
}

//implement expr to ColorMode
impl PealnColorMode {
    pub fn to_colormode(expr:&syn::Expr) -> PealnColorMode{
        match expr.to_token_stream().to_string().as_str() {
           "PealnColorMode::AlWAYS" => PealnColorMode::ALWAYS,
           "PealnColorMode::NEVER" => PealnColorMode::NEVER,
           "PealnColorMode::AUTO" => PealnColorMode::AUTO,
           _=>{
                panic!("Invalid pealn color mode: {}", expr.to_token_stream().to_string());
           }
        }
    }
}