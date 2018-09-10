extern crate rand;
use rand::{thread_rng, Rng};

#[macro_use] extern crate text_io;

fn read_guess() -> u8 {
    let mut result: u8 = 0;
    
    let mut done = false;
    while !done {
        println!("Gissa ett nummer mellan 1 och 100:");
        let number: Result<u8, _> = try_read!();
        match number {
            Ok(n) => {
                done = true;
                result = n;
            },
            Err(_e) => {
                println!("Nu förstår jag inte.");
            }
        };
    }
    result
}

fn main() {
    println!("Gissa nummer med Datormagazin!");

    // Skapa ett slumptal mellan 1 och 100
    let number: u8 = thread_rng().gen_range(0, 100) + 1;
    println!("Numret: {}", number);
    
    // Loopa tills vi är klara
    let mut done = false;
    while !done {
        // Läs nummer
        let guess = read_guess();
        
        // Kontrollera gissning
        if guess < number {
            println!("Fel, för lite!");
        } else if guess > number {
            println!("Fel, för mycket!");
        } else {
            println!("Rätt!");
            done = true;
        }
    }
}
