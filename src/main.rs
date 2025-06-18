use peacock::{pealn};
mod pea_compiled;

fn main() {

    let name  = "Subham Shaw";
    let age = 25;

    pealn!("[yellow,bold](Name) : [bold,red,italic,strikethrough,blue]({}) " , name );
    
    pealn!("[yellow,bold](Age) : [bold,strikethrough]({}) " , age );

}

