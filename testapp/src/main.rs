use pealn::{pealn , pealn_writeln};
use std::fmt;

struct Person {
    name: String,
    age: u32,
    phone: String,
    email: String,
}


impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        pealn_writeln!(
            f,
            "[yellow](Name): [bold]({})\n [yellow](Age): [bold]({}) \n [yellow](Phone): [bold]({}) \n [yellow](Email): [bold]({})",
            self.name, self.age, self.phone, self.email
         )

    }
}



fn main() {
    let person = Person {
        name: String::from("John Doe"),
        age: 30,
        phone: String::from("123-456-7890"),
        email: String::from("kanknkcc")};

    println!("Person : {}" , person);
}
