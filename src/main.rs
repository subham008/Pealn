use pealn::{pealn,pea};

fn main() {

    let name  = "Subham Shaw";
    let age = 25;

    pea!("[yellow,bold](Name) : [bold,underline]({}) " , name );
    
    pealn!("[yello,bold](Age) : [bold]({}) " , age );

}

