// #[macro_use]
// extern crate clap;

extern crate rand;
extern crate term;

use rand::prelude::*;
use rand::seq::SliceRandom;
use std::io;

type StdOut = Box<term::StdoutTerminal>;

#[derive(Debug, Copy, Clone)]
struct Color {
    name: char,
    color: term::color::Color,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn into_colors(guess: Vec<char>, all_colors: &Vec<Color>) -> Result<Vec<Color>, String> {
    let mut result: Vec<Color> = Vec::with_capacity(guess.len());
    for &g in guess.iter() {
        match all_colors.iter().find(|ref c| c.name == g) {
            Some(color) => {
                result.push(*color);
            }
            None => {
                return Err(format!("Invalid color: {}", g))
            }
        }
    }
    
    return Ok(result);
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

// there seems to be a bug in the term crate where it always pretends to be an xterm TERM
// this makes delete_line unsupported
// bypass the terminfo discovery, and just make a WinConsole directly
#[cfg(windows)]
pub fn stdout() -> Option<Box<term::StdoutTerminal>> {
    term::WinConsole::new(io::stdout())
    .ok()
    .map(|t| Box::new(t) as Box<term::StdoutTerminal>)
}

#[cfg(not(windows))]
pub fn stdout() -> Option<Box<term::StdoutTerminal>> {
    term::stdout()
}

enum Grade {
    Correct,
    Incorrect(String),
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
    
    let mut t = stdout().unwrap();
    
    print_intro(&all_colors, &mut t);

    let debug = true;
    let max_attempts = 6;
    let mut guess_count = 0;

    let secret = create_secret(&all_colors);
    if debug {
        print_colors(&secret, &mut t);
        println!();
    }
    loop {
        println!("Enter your guess (attempt {} of {}): ", guess_count + 1, max_attempts);
        match read_guess() {
            Ok(guess) => {
                erase_guess_from_terminal(&mut t).unwrap();
                match parse_guess(&guess, &all_colors, secret.len()) {
                    Err(message) => {
                        println!();
                        println!("Invalid guess: {}", message);
                        continue;
                    }
                    Ok(guessed_colors) => {
                        print_colors(&guessed_colors, &mut t);
                        match grade(&guessed_colors, &secret) {
                            Grade::Correct => {
                                println!();
                                println!("Congratulations, you got the correct colors!");
                                print_colors(&secret, &mut t);
                                println!();
                                break;
                            }
                            Grade::Incorrect(key) => {
                                guess_count += 1;
                                if guess_count < max_attempts {
                                    println!("{}", key);
                                    continue;
                                } else {
                                    println!();
                                    println!("Too bad, better luck next time!");
                                    println!("The secret was:");
                                    print_colors(&secret, &mut t);
                                    println!();
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
    
    println!();
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

fn erase_guess_from_terminal(t: &mut StdOut) -> Result<(), term::Error> {
    t.cursor_up()?;
    t.delete_line()?;
    t.cursor_up()?;
    t.delete_line()?;
    Ok(())
}

fn parse_guess(guess: &String, all_colors: &Vec<Color>, max_len: usize) -> Result<Vec<Color>, String> {
    let clean_guess = guess.trim().to_uppercase();
    let parsed = into_colors(clean_guess.chars().collect(), &all_colors)?;
    if parsed.len() != max_len { 
        return Err("Incorrect number of colors".to_string()) 
    }
    return Ok(parsed);
}

fn grade(guessed_colors: &Vec<Color>, secret: &Vec<Color>) -> Grade {
    let mut correct_position = 0;
    let mut correct_color = 0;
    for (guess_position, color_guess) in guessed_colors.iter().enumerate() {
        match secret.iter().position(|ref c| *c == color_guess) {
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
        return Grade::Correct;
    }

    let mut key=  String::new();
    print!("     ");
    for _ in 0..correct_position {
        key += "X ";
    }
    for _ in 0..correct_color {
        key += "O ";
    }
    for _ in 0..(4-correct_position - correct_color) {
        key += ". ";
    }
    return Grade::Incorrect(key);

}

fn create_secret(all_colors: &Vec<Color>) -> Vec<Color> {
    let mut rng = thread_rng();
    let secret = all_colors.choose_multiple(&mut rng, 4);
    return secret.cloned().collect();
}