
use pealn::pealn;

fn main() {

    let name  = "Subham Shaw";
    let age = 25837583;

    pealn!("[yellow,bold,purple](Name) : [bold,purple,green]({}) " , name );
    pealn!("[yellow,bold](Age) : [bold,italic,strikethrough]({}) " , age );
    


}

