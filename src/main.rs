mod plus_one;
use plus_one::plus_one;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    let x = plus_one(5);
    println!("plus is {}", x);
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("secret number is {}", secret_number);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readLine");
        println!("you guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type correct number:");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Please input your guess:");
    }
}
