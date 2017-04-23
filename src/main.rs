extern crate rand;
use rand::{thread_rng, sample};
use std::io;

fn main() {

    let all_colors: Vec<char> = vec!['R', 'G', 'Y', 'B', 'C', 'O'];

    println!("Available colors:");

    for color in &all_colors {
        print!("{}, ", color);
    }
    println!("");

    //random subset
    let mut rng = thread_rng();
    let secret = sample(&mut rng, all_colors, 4);

    let mut guess = String::with_capacity(6);

    let mut guess_count = 0;
    let mut guess_correct = false;
     
    while !guess_correct && guess_count < 6 {
        println!("Enter your guess");
        guess.clear();
        let mut correct_position = 0;
        let mut correct_color = 0;
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let clean_guess = guess.trim().to_uppercase();
                println!("Your guess: {}", clean_guess);
                for (guess_position, color_guess) in clean_guess.chars().enumerate() {
                    match secret.iter().position(|&c| c == color_guess) {
                        Some(position) => {
                            // println!("Color {} found in secret at position {}", color_guess, position);
                            if guess_position == position {
                                correct_position = correct_position + 1;
                            } else {
                                correct_color = correct_color + 1;
                            }
                        }
                        None => {}
                    }
                }
                if correct_position == 4 {
                    guess_correct = true;
                }
                println!("Correct positions: {}, correct colors: {}", correct_position, correct_color);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
        guess_count = guess_count + 1;
    }
    if guess_correct {
        println!("Congratulations, you got the correct colors!");
    } else {
        println!("Too bad, better luck next time!");
        println!("The secret was:");
    }
    for color in secret {
        print!("{}, ", color);
    }
    println!("");
}
