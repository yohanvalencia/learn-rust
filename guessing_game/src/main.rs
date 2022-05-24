use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {


    let mut name = String::new();


    println!("Type your name: ");
    io::stdin()
      .read_line(&mut name)
      .expect("Failed to read line");
   

    println!("I'm choosing a random number between 1 and 100...");
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("What number did I choose?");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,          
        };


        if guess > 100 {
          println!("Max number is 100!");
          continue;
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
              println!("You win {}", name.green());
              break;
            }
        }
    }
}
