import * as wasm from "mastermind-wasm";

wasm.init();

document.getElementById("button_check").onclick = function(event) {
    wasm.greet();
}

document.getElementById("button_panic").onclick = function(event) {
    wasm.trigger_panic();
}
