use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    let secret_number = rand::thread_rng().gen_range(0, 100);
    println!("Secret number {}", secret_number);
 
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
               .expect("Failed to read line");


        //shadows the variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Big")
        }
    
    }
}
