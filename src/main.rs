use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guessing game!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
 //   println!("Secrent number {}",secret_number);
    loop {
        
        println!("Enter a number between 1 and 101: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{println!("{}","Enter valid input.".red()); continue},
        };
        println!("Your guess is {}",guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".blue()),
            Ordering::Greater => println!("{}","Too big".blue()),
            Ordering::Equal =>{println!("{}","You win !".green());break;},
        } 
      
    }
}
