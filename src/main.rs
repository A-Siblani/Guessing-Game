use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret numner is: {secret_number}");
    loop{
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small like your willie!"),
            Ordering::Greater => println!("Too big like my willie!"),
            Ordering::Equal => {println!("You Win little bitch!");
            break;
        }
        }
    }
 
}
