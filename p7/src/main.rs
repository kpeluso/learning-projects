use std::io;

const MIN: u32 = 1;
const MAX: u32 = 100;
const MAX_GUESSES: u32 = 5;
const NUMBER: u32 = 42;

fn main() {
    println!("You have 5 chances to guess the number between {} and {}.", MIN, MAX);
    let mut guesses: u32 = 0;
    while guesses < MAX_GUESSES {
        println!("Guess a natrual number >>> ");
        let mut u = String::new();
        io::stdin()
            .read_line(&mut u)
            .expect("Can't read input!");
        let inp = u.trim().parse::<u32>().expect("Not an integer!");
        if inp > MAX || inp < MIN {
            println!("That was outside the acceptable range of {} and {}!", MIN, MAX);
            guesses += 1;
            continue;
        }
        match inp {
            NUMBER => {
                println!("Woah! You won! Congrats!");
                return;
            },
            _ => {
                if inp > NUMBER {
                    println!("Sorry, that was too high, try again...");
                } else {
                    println!("Sorry, that was too low, try again...");
                }
            }
        }
        guesses += 1;
    }
    println!("Sorry! Looks like you lost! Come back soon!");
}
