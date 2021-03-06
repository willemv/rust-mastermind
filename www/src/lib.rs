extern crate js_sys;
extern crate mastermind;
extern crate web_sys;

#[macro_use]
mod js_utils;
mod utils;

use js_utils::*;
use mastermind::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, NodeList};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init(doc: &Document) {
    utils::set_panic_hook();

    setup(doc);
}

#[wasm_bindgen(raw_module = "../state")]
extern "C" {
    #[wasm_bindgen(js_name = get_secret)]
    fn _get_secret() -> String;
}

#[wasm_bindgen(raw_module = "../view")]
extern "C" {
    #[wasm_bindgen(js_name = display_grade)]
    fn _display_grade(grade: &str, guess: &str);
}

#[allow(unused_unsafe)]
//this is for the benefit or rust-analyzer, who marks all usages of the regular log_1 as unsafe
fn get_secret() -> String {
    unsafe { _get_secret() }
}

#[allow(unused_unsafe)]
//this is for the benefit or rust-analyzer, who marks all usages of the regular log_1 as unsafe
fn display_grade(grade: &str, guess: &str) {
    unsafe {
        _display_grade(grade, guess);
    }
}

fn setup(doc: &Document) {
    let guess_form: Element = doc.get_element_by_id("guess_form").expect("could not find #guess_form");
    let guess_pegs: NodeList = guess_form
        .query_selector_all(".peg")
        .expect("could not find guess pegs");

    let cb = add_callback(&guess_form, "submit", move |event| {
        console_log!("[RUST] clicked guess button, event {:?}", event);

        event.prevent_default();

        let mut guess_from_ui: Vec<Option<Color>> = Vec::with_capacity(guess_pegs.length() as usize);
        for i in 0..guess_pegs.length() {
            let peg_node = guess_pegs.item(i).expect("indexing the NodeList should work");
            let peg = peg_node
                .dyn_ref::<HtmlElement>()
                .expect("all guess pegs should be HtmlElements");
            let color = peg.dataset().get("color").and_then(|name| color_name_to_color(&name));
            guess_from_ui.push(color);
        }

        let guess_from_ui: Option<Vec<Color>> = guess_from_ui.into_iter().fold(Some(vec![]), |acc, val| match acc {
            None => None,
            Some(mut vec) => match val {
                None => None,
                Some(value) => {
                    vec.push(value);
                    Some(vec)
                }
            },
        });
        console_log!("[RUST] Guess from UI : {:?}", guess_from_ui);
        console_log!("[RUST] Compare to secret '{}'", get_secret());

        let solution = get_secret();
        let solution = parse_colors(&solution);

        match (guess_from_ui, solution) {
            (Some(guess), Ok(solution)) => {
                let grade = mastermind::grade(&guess, &solution);
                console_log!("[RUST] grade: {:?}", grade);
                let grade_string = grade_to_string(&grade);
                let guess_string = guess_to_string(&guess);

                display_grade(&grade_string, &guess_string);
            }
            _ => console_log!("not ok"),
        };
    });

    //TODO keep these registrations in a collection, and drop when appropriate
    cb.forget();
}

fn guess_to_string(guess: &[Color]) -> String {
    guess.iter().fold(String::new(), |mut string, color| {
        string.push(match *color {
            Color::Red => 'R',
            Color::Green => 'G',
            Color::Blue => 'B',
            Color::Yellow => 'Y',
            Color::Cyan => 'C',
            Color::Purple => 'P',
        });
        string
    })
}

fn grade_to_string(grade: &Grade) -> String {
    match *grade {
        Grade::Correct => "XXXX".to_string(),
        Grade::Incorrect {
            correct_position,
            correct_color,
            wrong,
        } => {
            let mut result = String::new();
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

#[wasm_bindgen]
pub fn trigger_panic() {
    panic!("Testing panic in consoles: {}", "test formatting");
}

fn parse_colors(string: &str) -> Result<Vec<Color>, &str> {
    let mut result = Vec::with_capacity(string.len());
    for char in string.chars() {
        match char {
            'R' => result.push(Color::Red),
            'Y' => result.push(Color::Yellow),
            'G' => result.push(Color::Green),
            'B' => result.push(Color::Blue),
            'C' => result.push(Color::Cyan),
            'P' => result.push(Color::Purple),
            _ => return Err("invalid color"),
        };
    }
    Ok(result)
}

#[allow(unused)]
fn color_symbol_to_color_name(symbol: &str) -> String {
    match symbol {
        "R" => "red",
        "G" => "green",
        "Y" => "yellow",
        "B" => "blue",
        "P" => "purple",
        "C" => "cyan",
        _ => "unknown",
    }
    .into()
}

#[allow(unused)]
fn color_name_to_color_symblol(color_name: &str) -> String {
    match color_name {
        "red" => "R",
        "green" => "G",
        "yellow" => "Y",
        "blue" => "B",
        "purple" => "P",
        "cyan" => "C",
        _ => "",
    }
    .into()
}

fn color_name_to_color(color_name: &str) -> Option<Color> {
    match color_name {
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "purple" => Some(Color::Purple),
        "cyan" => Some(Color::Cyan),
        _ => None,
    }
}

#[allow(unused)]
fn grade_to_class(grade: &str) -> String {
    match grade {
        "X" => "correct_position",
        "O" => "correct_color",
        "." => "wrong",
        _ => "",
    }
    .into()
}
