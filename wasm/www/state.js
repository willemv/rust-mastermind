import shuffle from "shuffle-array";

let all_colors = ["R", "G", "Y", "B", "C", "P"];
let all_color_names = ["red", "green", "yellow", "blue", "cyan", "purple"];

function generate_secret() {
    return shuffle.pick(all_colors, { 'picks': 4 }).join("");
}

let secret = generate_secret();

function get_secret() {
    return secret;
}

function update_secret() {
    secret = generate_secret();
    return secret;
}

export {
    get_secret,
    update_secret
}