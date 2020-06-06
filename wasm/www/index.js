import * as wasm from "mastermind-wasm";
import shuffle from "shuffle-array";
import * as templ from "./microtemplating";

wasm.init();

let all_colors = ["R", "G", "Y", "B", "C", "P"];
let all_color_names = ["red", "green", "yellow", "blue", "cyan", "purple"];

function generate_secret() {
    return shuffle.pick(all_colors, { 'picks': 4 }).join("");
}

let secret = generate_secret();

let results = document.getElementById("results");

document.getElementById("button_generate_secret").addEventListener("click", event => {
    secret = generate_secret();
    console.log(secret);
    while (results.hasChildNodes()) {
        results.removeChild(results.firstChild);
    }
});

function color_symbol_to_color_name(symbol) {
    if (symbol == "R") {
        return "red";
    }
    if (symbol == "G") {
        return "green";
    }
    if (symbol == "Y") {
        return "yellow";
    }
    if (symbol == "B") {
        return "blue";
    }
    if (symbol == "C") {
        return "cyan";
    }
    if (symbol == "P") {
        return "purple";
    }
    throw new Error("Unknown color symbol: " + symbol);
}

function color_name_to_color_symblol(color_name) {
    if (color_name == "red") {
        return  "R";
    }
    if (color_name == "green") {
        return  "G";
    }
    if (color_name == "yellow") {
        return  "Y";
    }
    if (color_name == "blue") {
        return  "B";
    }
    if (color_name == "cyan") {
        return  "C";
    }
    if (color_name == "purple") {
        return  "P";
    }
    throw new Error("Unknown color name: " + color_name);
}

function grade_to_class(grade) {
    if (grade == "X") {
        return "correct_position";
    }
    if (grade == "O") {
        return "correct_color";
    }
    if (grade == ".") {
        return "wrong";
    }
    throw new Error("Uknown grade: " + grade);
}

function process_guess(guess) {
    let grade = wasm.grade(guess, secret);
    if (!grade.match(/^[XO.]+$/)) {
        console.error(grade);
        return false;
    }

    let template_data = {
        guess: guess.split('').map(symbol => {return {color: color_symbol_to_color_name(symbol), symbol: symbol}}),
        result: grade.split('').map(grade => {return {symbol: grade, grade: grade_to_class(grade)}}),
    };
    results.innerHTML = results.innerHTML + templ.tmpl("template_grade", template_data);
    return true;
}

function next_color_after(color) {
    let current_index = all_color_names.findIndex(c => c === color);
    if (current_index >= 0) {
        return all_color_names[ (current_index + 1) % all_color_names.length];
    } else {
        return undefined;
    }
}

let guess_form = document.getElementById("guess_form");
let guess_pegs = guess_form.querySelectorAll(".peg");
guess_pegs.forEach(peg => {
    peg.addEventListener("click", function(event) {
        event.preventDefault();
        //todo show color wheel
        let next_color = "red"; //provide a default
        let current_color = this.dataset.color;
        if (current_color) {
            next_color = next_color_after(current_color);
        }
        this.dataset.color = next_color;
        return false;
    });
});

function convert_names_to_guess(color_names) {
    for (let name in color_names) {
        console.log(color_names[name]);
        if (!color_names[name]) return undefined;
    }
    return color_names.map(name => color_name_to_color_symblol(name)).join("");
}

guess_form.addEventListener("submit", event => {
    event.preventDefault();
    let guess_color_names = Array.from(guess_pegs).map(peg => peg.dataset.color);
    let guess = convert_names_to_guess(guess_color_names);

    if (!guess) {
        console.error("Guess a color for each peg");
    } else {
        try {
            process_guess(guess);
        } catch (error) {
            console.log(error);
        }
    }
    return true;
});
