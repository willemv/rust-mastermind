import * as wasm from "mastermind-wasm";
import * as shuffle from "shuffle-array";

wasm.init();

let all_colors = ["R", "G", "Y", "B", "C", "P"];

function generate_secret() {
    return shuffle.pick(all_colors, { 'picks': 4 }).join("");
}

let secret = generate_secret();

let results = document.getElementById("results");

let grade_template = document.getElementById("template_grade").innerHTML;

document.getElementById("button_generate_secret").onclick = function (event) {
    secret = generate_secret();
    console.log(secret);
    while (results.hasChildNodes()) {
        results.removeChild(results.firstChild);
    }
}

function check(guess) {
    
    let grade = wasm.grade(guess, secret);
    if (!grade.match(/^[XO.]+$/)) {
        console.error(grade);
        return false;
    }
    console.log(grade);

    let filled_grade_template = grade_template;
    for (let i = 1; i < 5; i++) {
        let guess_color = (guess.charAt(i - 1));
        filled_grade_template = filled_grade_template.replace("guess_symbol_" + i, guess_color);
        if (guess_color == "R") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "red");
        }
        if (guess_color == "G") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "green");
        }
        if (guess_color == "Y") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "yellow");
        }
        if (guess_color == "B") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "blue");
        }
        if (guess_color == "C") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "cyan");
        }
        if (guess_color == "P") {
            filled_grade_template = filled_grade_template.replace("guess_" + i, "purple");
        }
    }

    for (let i = 1; i < 5; i++) {
        let grade_result = grade.charAt(i-1);
        filled_grade_template.replace("grade_symbol_" + i, grade_result);
        if (grade_result == "X") {
            filled_grade_template = filled_grade_template.replace("grade_" + i, "correct_position");
        }
        if (grade_result == "O") {
            filled_grade_template = filled_grade_template.replace("grade_" + i, "correct_color");
        }
        if (grade_result == ".") {
            filled_grade_template = filled_grade_template.replace("grade_" + i, "wrong");
        }
    }


    console.log(filled_grade_template);
    results.innerHTML = results.innerHTML + filled_grade_template;
    return true;

}

document.getElementById("check_form").onsubmit = function (event) {
    let guess = this.guess.value;
    let check_valid = check(guess);
    if (check_valid) {
        this.guess.value = "";
    }
    event.preventDefault();
    return true;
}
