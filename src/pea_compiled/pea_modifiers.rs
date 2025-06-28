

#[derive(Debug, Clone, PartialEq)]
pub enum PeaModifier{
    LINK(String), // Link name , ANSI format \e]8;; https://crates.io/ \a Crates.io \e]8;;\a
    INVALID, // Invalid modifier
}

impl PeaModifier {

   pub fn from(modifier: &str ) -> Option<Self> {
           //modifier syntax  modifier name with arguments [link( https://crates.io/)](crates.io)
         let link = modifier.split('(').next().unwrap_or("").trim().to_lowercase();
         let modifier_arg = modifier.split('(').nth(1).unwrap_or("").trim().trim_end_matches(')').to_lowercase();

           match link.as_str() {
              "link" => Some( PeaModifier::LINK(String::from(modifier_arg))),
               _ =>None
           }
     }

     pub fn get_codes(&self) -> (String,String) {
         match self {
             PeaModifier::LINK(link) => ( format!( "\033]8;;{}\033\\\\ " , link) , "\033]8;;\033\\\\".to_string() ),
             PeaModifier::INVALID =>("".to_string() , "".to_string() ) ,
         }
     }
}

impl std::convert::From<&str> for  PeaModifier {

     fn from(modifier: &str ) -> Self {
           //modifier syntax  modifier name with arguments [link( https://crates.io/)](crates.io)
         let link = modifier.split('(').next().unwrap_or("").trim().to_lowercase();
         let modifier_arg = modifier.split('(').nth(1).unwrap_or("").trim().trim_end_matches(')').to_lowercase();

           match link.as_str() {
              "link" => PeaModifier::LINK(String::from(modifier_arg)),
               _ =>PeaModifier::INVALID
           }
     }

}