
use pealn::{pealn};
fn main() {

    let name  = "Subham Shaw";
    let age = 23;
    let phone  = "987xxxxx0";
    let email ="johndoe2002@gmail.com";

    pealn!("[yellow,bold](Name)   : [bold,default,purple]({}) " , name);
    pealn!("[yellow,bold](Age)    : [bold]({}) " , age );
    pealn!("[yellow,bold](Phone)  : [orange,orange,bold]({}) " , phone );
    pealn!("[yellow,bold](Email)  : [bold,italic]({}) " , email );


 }


