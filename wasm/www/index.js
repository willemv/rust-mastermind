import * as wasm from "mastermind-wasm";
import * as shuffle from "shuffle-array";
import * as templ from "./microtemplating";

wasm.init();

let all_colors = ["R", "G", "Y", "B", "C", "P"];

function generate_secret() {
    return shuffle.pick(all_colors, { 'picks': 4 }).join("");
}

let secret = generate_secret();

let results = document.getElementById("results");

document.getElementById("button_generate_secret").onclick = function (event) {
    secret = generate_secret();
    console.log(secret);
    while (results.hasChildNodes()) {
        results.removeChild(results.firstChild);
    }
}

function symbol_to_color(symbol) {
    
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
    throw new Error("Unknown symbol: " + symbol);
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

function check(guess) {
    
    let grade = wasm.grade(guess, secret);
    if (!grade.match(/^[XO.]+$/)) {
        console.error(grade);
        return false;
    }

    let template_data = {
        guess: guess.split('').map(symbol => {return {color: symbol_to_color(symbol), symbol: symbol}}),
        result: grade.split('').map(grade => {return {symbol: grade, grade: grade_to_class(grade)}}),
    };
    results.innerHTML = results.innerHTML + templ.tmpl("template_grade", template_data);
    return true;
}

document.getElementById("check_form").onsubmit = function (event) {
    let guess = this.guess.value;
    try{
        let check_valid = check(guess);
        if (check_valid) {
            this.guess.value = "";
        }
    } catch (error) {
        console.log(error);
    }
    event.preventDefault();
    return true;
}
