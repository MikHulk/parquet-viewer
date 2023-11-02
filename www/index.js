import { greet } from "./wasm-pkg/parquet_viewer.js";
import { Elm } from "./elm.js";

var app = Elm.Main.init({
  node: document.getElementById('myapp')
});
greet();
