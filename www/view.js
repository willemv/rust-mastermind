import * as templ from "./microtemplating";
import * as state from "./state";

let results = undefined;
let color_chooser = undefined;

function init(document) {
    results = document.getElementById("results");
    color_chooser = document.getElementById("color_chooser");

    document.getElementById("button_new_game").addEventListener("click", event => {
        let secret = state.update_secret();
        console.log(secret);
        while (results.hasChildNodes()) {
            results.removeChild(results.firstChild);
        }
    });

    let guess_form = document.getElementById("guess_form");
    let guess_pegs = guess_form.querySelectorAll(".peg");

    for (let color_node of color_chooser.getElementsByTagName("span")) {
        let node_color = color_node.dataset.color;
        color_node.onclick = e => {
            color_chooser.style.display="none";
            console.log(node_color);
            let c = new CustomEvent("color", {detail: node_color});
            color_chooser.dispatchEvent(c);
        }
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

}

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

function color_name_to_color_symbol(color_name) {
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

function display_grade(grade, guess) {
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

function show_color_chooser(x, y, onColor) {
    color_chooser.style.display = "block";

    let bounds = color_chooser.getBoundingClientRect();

    color_chooser.style.left = (x - (bounds.width / 2)) + "px";
    color_chooser.style.top = (y - (bounds.height / 2)) + "px";

    color_chooser.addEventListener("color", event => onColor(event.detail), {once: true});
}


export {
    init,
    display_grade,
    color_name_to_color_symbol
}