
use pealn::pealn;

fn main() {

    let name  = "Subham Shaw";
    let age = 25;

    pealn!("[yellow,blue,bold](Name) : [bold,purple]({}) " , name );
    
    pealn!("[yellow,bold](Age) : [bold]({}) " , age );


}

