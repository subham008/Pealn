use Peacock::{ peacock};

fn main() {
  
   let name = "Subham Shaw";
   let age = 89;

   peacock!(
    "
     yellow(Name) : cyan({})
     yellow(Age) : cyan({})
     red(This is called from main function!)
    
   ",   name, age
   );

}