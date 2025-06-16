use Peacock::{pealn , pea};

mod pea_compiled;

fn main() {
      
      let name = "subham shaw";
      let age  =  294593;
      
     // pea!("Name:[yellow]({})\nAge:[(218, 66, 245) , bold]({})" ,name , age);
      pealn!("Name:[yello]({})\nAge:[(218, 66, 245),bold,underline, strikethrough]({})" ,name , age);
      

}

