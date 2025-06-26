
use pealn::pealn;

fn main() {

    // let name  = "Subham Shaw";
    // let age = 25837583;

    // pealn!("[yellow,bold](Name) : [bold,purple]({}) " , name );
    // pealn!("[yellow,bold](Age) : [bold,italic]({}) " , age );
    
    let s = "link(Copy to clipboard )";
    let modifier_name = s.split('(').next().unwrap_or("").trim().to_lowercase();
    let modifier_arg = s.split('(').nth(1).unwrap_or("").trim().trim_end_matches(')').to_lowercase();
    println!("modifier_name: {}", modifier_name);
    println!("modifier_arg:{}", modifier_arg);

}

