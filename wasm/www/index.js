import * as wasm from "mastermind-wasm";
import * as shuffle from "shuffle-array";

wasm.init();

let all_colors = ["R", "G", "Y", "B", "C", "P"];

function generate_secret() {
    return shuffle.pick(all_colors, { 'picks': 4 }).join("");
}

let secret = generate_secret();

let results = document.getElementById("results");

document.getElementById("button_generate_secret").onclick = function(event) {
    secret = generate_secret();
    console.log(secret);
    while(results.hasChildNodes()) {
        results.removeChild(results.firstChild);
    }
}

document.getElementById("check_form").onsubmit = function(event) {
    let guess = this.guess.value;
    this.guess.value = "";
    let grade = wasm.grade(guess, secret)

    let new_result = document.createElement("div");
    new_result.innerText = guess + " -> " + grade;
    results.appendChild(new_result);

    event.preventDefault();
    return true;
}
