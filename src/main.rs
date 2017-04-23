extern crate rand;
extern crate term;

use rand::{thread_rng, sample};
use std::io;

#[derive(Debug, Copy, Clone)]
struct Color {
    name: char,
    color: term::color::Color,
}

fn into_colors(guess: Vec<char>, all_colors: &Vec<Color>) -> Option<Vec<Color>> {
    let mut result: Vec<Color> = Vec::with_capacity(guess.len());
    for &g in guess.iter() {
        match all_colors.iter().find(|ref c| c.name == g) {
            Some(color) => {
                result.push(*color);
            }
            None => {
                // println!("Could not find color {}", g);
            }
        }
    }
    
    return Some(result);
}

fn print_color_names(colors: &Vec<Color>, t: &mut Box<term::StdoutTerminal>) {
    for color in colors {
        t.fg(color.color).unwrap();
        write!(t, "{} ", color.name).unwrap();
    }
    t.reset().unwrap();
}

fn print_colors(colors: &Vec<Color>, t: &mut Box<term::StdoutTerminal>) {
    let circle = '\u{25CF}';
    for color in colors {
        t.fg(color.color).unwrap();
        write!(t, "{} ", circle).unwrap();
    }
    t.reset().unwrap();
}

fn main() {
    let all_colors: Vec<Color> = vec![
        Color{name: 'R', color: term::color::RED},
        Color{name: 'G', color: term::color::GREEN},
        Color{name: 'Y', color: term::color::YELLOW},
        Color{name: 'B', color: term::color::BLUE},
        Color{name: 'C', color: term::color::CYAN},
        Color{name: 'P', color: term::color::MAGENTA}
    ];
    
    let mut t = term::stdout().unwrap();
    println!("Grading: X = correct position, O = correct color, . = wrong");
    println!("Available colors:");
    print_color_names(&all_colors, &mut t);
    println!();
    print_colors(&all_colors, &mut t);
    println!();


    println!("");
    let mut rng = rand::os::OsRng::new().unwrap();
    let secret = sample(&mut rng, &all_colors, 4);

    let mut guess = String::with_capacity(6);

    let mut guess_count = 0;
    let mut guess_correct = false;
     
    while !guess_correct && guess_count < 6 {
        println!("Enter your guess: ");
        guess.clear();
        let mut correct_position = 0;
        let mut correct_color = 0;
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                t.cursor_up().unwrap();
                t.delete_line().unwrap();
                t.cursor_up().unwrap();
                t.delete_line().unwrap();
                let clean_guess = guess.trim().to_uppercase();
                let guessed_colors = into_colors(clean_guess.chars().collect(), &all_colors).unwrap();
                print_colors(&guessed_colors, &mut t);
                for (guess_position, color_guess) in clean_guess.chars().enumerate() {
                    match secret.iter().position(|ref c| c.name == color_guess) {
                        Some(position) => {
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

                print!("     ");
                for _ in (0..correct_position) {
                    print!("X ")
                }
                for _ in (0..correct_color) {
                    print!("O ")
                }
                for _ in (0..(4-correct_position - correct_color)) {
                    print!(". ");
                }
                println!();
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
        print!("{}, ", color.name);
    }
    println!("");
}
