

#[derive(Debug, Clone, PartialEq)]
pub enum PeaModifier{
    LINK(String), // Link name 
    INVALID, // Invalid modifier
}

impl std::convert::From<&str> for  PeaModifier {

     fn from(s: &str) -> Self {
           //modifier syntax  modifier name with arguments [link(name)](user string)
         let modifier_name = s.split('(').next().unwrap_or("").trim().to_lowercase();
         let modifier_arg = s.split('(').nth(1).unwrap_or("").trim().trim_end_matches(')').to_lowercase();

           match modifier_name.as_str() {
              "link" => PeaModifier::LINK(String::from(modifier_arg)),
               _ =>PeaModifier::INVALID
           }
     }
}