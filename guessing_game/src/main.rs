use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("welcome to my guess number game!");
    println!("u have 3 times chances to type your number.");
    println!("if the number is matched, u win.");
    println!("otherwise, u lose.");
    println!("let's Go");
    let secret_number = rand::thread_rng().gen_range(0..101);
    let mut count: i8 = 3;

    loop {
        println!("the {} times chance", count);
        println!("please input the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("secret number is {}", secret_number);
                println!("you ara win!");
                break;
            }
        }
        count = count - 1;
        if count == 0 {
            println!("secret number is {}", secret_number);
            println!("u lose");
            break;
        }
    }
}
