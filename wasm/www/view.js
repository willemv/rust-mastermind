import * as templ from "./microtemplating";

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

export {
    display_grade,
    color_name_to_color_symbol
}