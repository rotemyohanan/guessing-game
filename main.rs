#![allow(unused)]
extern crate rand;

use std::io;
//use io::Result; // this is an enumeration
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failes to read line");
    println!("You guessed: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You Win"),
    }
}

// fn main() {
//     println!("Guess the number");
//     println!("Please input a number:");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("Failed to read line");
//     println!("You guess: {}", guess);
// }


// fn main() {
//     let x = 6;
//     let y = 9;

//     println!("x = {} and y = {}", x, y);
// }