use std::{cmp::Ordering, io};    
use rand::Rng;


fn main() {

    let correct_answer = rand::thread_rng().gen_range(1..=10);
    println!("Correct Answer: {correct_answer}");


    loop {

        let mut guess = String::new();

        println!("Guess a number: 1-10 ");
        io::stdin()
            .read_line(&mut guess)
            .expect("error reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(e) => {
                println!("Error with parsing, {e}");
                continue;
            }
        };

        let message = match guess.cmp(&correct_answer) {
            Ordering::Greater => "Too High",
            Ordering::Less => "Too Low",
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        };

        println!("{message}");
    }
    
}
