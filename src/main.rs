
use pealn::pealn;
fn main() {

    let name  = "Subham Shaw";
    let age = 25837583;

    pealn!("[yellow,bold](Name) : [bold,purple]({}) " , name );
    pealn!("[yellow,bold](Age)  : [bold,italic]({}) " , age );
    pealn!("[yellow,bold](link) : [bold,green,link(https://www.youtube.com/watch?v=1tcCEzJvcb8)](YouTube) " );
    // let input = "[arg , arg1 , arg2 , (22,44,55), bold(jsjc), underlined]";
    // let re = Regex::new(r"\((?:[^\(\)]|(?R))*\)|[^,\[\]\s][^,\[\]]*").unwrap();

    // let args: Vec<&str> = re
    //     .find_iter(input)
    //     .map(|mat| mat.as_str().trim())
    //     .collect();

    // println!("{:?}", args);
 }


