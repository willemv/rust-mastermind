import * as wasm from "mastermind-wasm";
import * as state from "./state";

wasm.init(document);

let results = document.getElementById("results");

document.getElementById("button_new_game").addEventListener("click", event => {
    let secret = state.update_secret();
    console.log(secret);
    while (results.hasChildNodes()) {
        results.removeChild(results.firstChild);
    }
});

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
