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
pub fn greet() {
    let guess: [Color; 4] = [RED, YELLOW, GREEN, BLUE];
    let solution: [Color; 4] = [PURPLE, YELLOW, GREEN, BLUE];
    match grade(&guess, &solution) {
        Grade::Correct => {
            alert("Guess was correct");
        }
        Grade::Incorrect { .. } => {
            alert("Incorrect guess");
        }
        Grade::Invalid(_) => {
            alert("Invalid guess");
        }
    }
}
