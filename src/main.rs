use rand::Rng; // random Range     ex: let mut guess = String::new();   The :: syntax in the ::new line indicates that new is an associated function of the String type. 
use std::cmp::Ordering;  // standard library compare  
use std::io; // standard library input/output

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);  // Retrieve the lazily-initialized thread-local random number generator, seeded by the system. https://docs.rs/rand/0.6.5/rand/fn.thread_rng.html
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your number!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)  // &mut 
            .expect("Failed to read line");
            
            
            //  match Ok(num) ger en siffra eller Err(_) klumpar ihop feldumpar och fortsätter
         let guess: u32 = match guess.trim().parse() {  
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                 println!("You win!");
                 break;
        }
            
        }
    }
}
