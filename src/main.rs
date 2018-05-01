extern crate rand;
extern crate term;

use rand::{thread_rng, sample};
use std::io;

type StdOut = Box<term::StdoutTerminal>;

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

fn print_color_names(colors: &Vec<Color>, stdout: &mut StdOut) {
    for color in colors {
        stdout.fg(color.color).unwrap();
        write!(stdout, "{} ", color.name).unwrap();
    }
    stdout.reset().unwrap();
}

fn print_colors<'a, I>(colors: I, stdout: &mut StdOut)
    where I: IntoIterator<Item=&'a Color>
{
    let circle = '\u{25CF}';
    for color in colors {
        stdout.fg(color.color).unwrap();
        write!(stdout, "{} ", circle).unwrap();
    }
    stdout.reset().unwrap();
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
    
    print_intro(&all_colors, &mut t);

    // let mut guess = String::with_capacity(6);

    let mut guess_count = 0;
    let mut guess_correct = false;

    let secret = create_secret(&all_colors);
     
    while !guess_correct && guess_count < 6 {
        println!("Enter your guess: ");
        // guess.clear();
        match read_guess() {
            Ok(guess) => {
                erase_guess_from_terminal(&mut t);
                check_guess(&guess, &all_colors, &mut t, &secret, &mut guess_correct);
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
    print_colors(secret.iter().cloned(), &mut t);
}

fn print_intro(all_colors: &Vec<Color>, t: &mut StdOut) { 

    println!("Grading: X = correct position, O = correct color, . = wrong");
    println!("Available colors:");
    print_color_names(all_colors, t);
    println!();
    print_colors(all_colors, t);
    println!();

    println!("");
}

fn read_guess() -> io::Result<String> {
   let mut guess = String::with_capacity(6);
   io::stdin().read_line(&mut guess).and(Ok(guess))
}

fn erase_guess_from_terminal(t: &mut StdOut) {
    t.cursor_up().unwrap();
    t.delete_line().unwrap();
    t.cursor_up().unwrap();
    t.delete_line().unwrap();
}

fn check_guess(guess: &String, all_colors: &Vec<Color>, t: &mut StdOut, secret: &Vec<&Color>, guess_correct: &mut bool) {
    let mut correct_position = 0;
    let mut correct_color = 0;
    let clean_guess = guess.trim().to_uppercase();
    let guessed_colors = into_colors(clean_guess.chars().collect(), &all_colors).unwrap();
    print_colors(&guessed_colors, t);
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
        *guess_correct = true;
    }

    print!("     ");
    for _ in 0..correct_position {
        print!("X ")
    }
    for _ in 0..correct_color {
        print!("O ")
    }
    for _ in 0..(4-correct_position - correct_color) {
        print!(". ");
    }
    println!();

}

fn create_secret(all_colors: &Vec<Color>) -> Vec<&Color> {
    let mut rng = thread_rng();
    let secret = sample(&mut rng, all_colors, 4);
    return secret;
}