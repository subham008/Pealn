use peacock::{pealn};
mod pea_compiled;

fn main() {

    let name  = "Subham Shaw";
    let age = 25;

    pealn!("[yellow, bold](Name) : [italic]({}) " , name );
    
    pealn!("[yellow, bold](Age) : [bold]({}) " , age );

}

