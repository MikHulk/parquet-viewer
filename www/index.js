import { greet } from "./build/wasm-pkg/parquet_viewer.js";
import { Elm } from "./build/elm.js";

console.log("ho");
greet();
var app = Elm.Main.init({
  node: document.getElementById('myapp')
});
