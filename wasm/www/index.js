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

document.getElementById("button_new_game").addEventListener("click", event => {
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
        return "R";
    }
    if (color_name == "green") {
        return "G";
    }
    if (color_name == "yellow") {
        return "Y";
    }
    if (color_name == "blue") {
        return "B";
    }
    if (color_name == "cyan") {
        return "C";
    }
    if (color_name == "purple") {
        return "P";
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
        guess: guess.split('').map(symbol => { return { color: color_symbol_to_color_name(symbol), symbol: symbol } }),
        result: grade.split('').map(grade => { return { symbol: grade, grade: grade_to_class(grade) } }),
    };
    results.innerHTML = results.innerHTML + templ.tmpl("template_grade", template_data);
    return true;
}

function next_color_after(color) {
    let current_index = all_color_names.findIndex(c => c === color);
    if (current_index >= 0) {
        return all_color_names[(current_index + 1) % all_color_names.length];
    } else {
        return undefined;
    }
}

let guess_form = document.getElementById("guess_form");
let guess_pegs = guess_form.querySelectorAll(".peg");
let color_chooser = document.getElementById("color_chooser");

for (let color_node of color_chooser.getElementsByTagName("span")) {
    let node_color = color_node.dataset.color;
    color_node.onclick = e => {
        color_chooser.style.display="none";
        console.log(node_color);
        let c = new CustomEvent("color", {detail: node_color});
        color_chooser.dispatchEvent(c);
    }
}

function show_color_chooser(x, y, onColor) {
    color_chooser.style.display = "block";

    let bounds = color_chooser.getBoundingClientRect();

    color_chooser.style.left = (x - (bounds.width / 2)) + "px";
    color_chooser.style.top = (y - (bounds.height / 2)) + "px";

    color_chooser.addEventListener("color", event => onColor(event.detail), {once: true});
}

guess_pegs.forEach(peg => {
    peg.addEventListener("click", function (event) {
        event.preventDefault();

        let peg_bounds = this.getBoundingClientRect();
        show_color_chooser(
            peg_bounds.left + (peg_bounds.width / 2),
            peg_bounds.top + (peg_bounds.height / 2),
            color => this.dataset.color = color
        );

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
