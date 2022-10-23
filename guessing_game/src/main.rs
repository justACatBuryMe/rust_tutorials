use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {

    // GENERATING NUMBER

    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    loop {

        // GETTING INPUTS

        println!("input :");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error getting input");

        let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        // COMPARING NUMBERS

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".yellow()),
            Ordering::Equal => { println!("{}", "YOU GOT IT".green()); break },
            Ordering::Greater => println!("{}", "too big".red()),
        }
    }

}
