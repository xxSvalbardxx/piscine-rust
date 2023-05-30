/* Write a program that prints a riddle, receives input from the user and checks that the answer is correct.

The program must allow an indefinite number of trials and only quit after the correct answer is given.

Every time the user introduces an incorrect answer the program must print the riddle again and after the user gives the correct answer the program must print the number of tries that took to get the correct answer.

Riddle: I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?

Answer: The letter e */

use std::io;

fn main() {
    let mut tries = 0;
    let mut answer = String::new();
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim() == "The letter e" {
            tries += 1;
            println!(
                "Number of trials: {}",
                tries
            );
            break;
        } else {
            tries += 1;
            //println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
            answer.clear();
        }
    }
}
