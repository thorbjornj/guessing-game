use rand::Rng; // random Range       
use std::cmp::Ordering;  // standard library compare  
use std::io; // standard library input/output

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);  // Retrieve the lazily-initialized thread-local random number generator, seeded by the system. https://docs.rs/rand/0.6.5/rand/fn.thread_rng.html
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your number!");

        let mut guess = String::new();  //let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String

        io::stdin()  // Now we’ll call the stdin function from the io module. If we hadn’t put the use std::io line at the beginning of the program, we could have written this function call as std::io::stdin. 
            .read_line(&mut guess)  // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times..... For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
            
            .expect("Failed to read line"); //When you call a method with the .method_name() syntax, it’s often wise to introduce a newline and other whitespace to help break up long lines. We could have written this code as: io::stdin().read_line(&mut guess).expect("Failed to read line");  https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
            
            
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
