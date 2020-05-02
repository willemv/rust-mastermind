extern crate mastermind;

mod utils;

use mastermind::Color::*;
use mastermind::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn trigger_panic() {
    panic!("Testing panic in consoles: {}", "test formatting");
}

#[wasm_bindgen]
pub fn grade(guess: String, solution: String) -> String {
    let guess  = parse_colors(&guess);
    let solution = parse_colors(&solution);

    match (guess, solution) {
        (Ok(guess), Ok(solution)) => {
            match mastermind::grade(&guess, &solution) {
                Grade::Correct => "CORRECT".to_string(),
                Grade::Incorrect{
                    correct_position,
                    correct_color,
                    wrong
                } => {
                    let mut result = String::with_capacity(guess.len());
                    for _ in 0..correct_position {
                        result.push('X');
                    }
                    for _ in 0..correct_color {
                        result.push('O');
                    }
                    for _ in 0..wrong {
                        result.push('.');
                    }
                    result
                }
                Grade::Invalid(_) => "INVALID".to_string(),
            }
        }
        (Ok(_), Err(message)) => "PARSE_ERROR GUESS".to_string(),
        (Err(message), Ok(_)) => "PARSE_ERROR SOLUTION".to_string(),
        (Err(m1), Err(m2)) => "PARSE_ERROR_GUESS_AND_SOLUTION".to_string(),
    }
   

}

fn parse_colors(string: &str) -> Result<Vec<Color>, &str> {
    let mut result = Vec::with_capacity(string.len());
    for char in string.chars() {
         match char {
            'R' => result.push(Color::RED),
            'Y' => result.push(Color::YELLOW),
            'G' => result.push(Color::GREEN),
            'B' => result.push(Color::BLUE),
            'C' => result.push(Color::CYAN),
            'P' => result.push(Color::PURPLE),
            _ => return Err("invalid color")
        };
    }
    Ok(result)
}
