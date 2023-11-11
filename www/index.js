import {
  Container,
  set_panic_hook,
} from "./build/wasm-pkg/parquet_viewer.js";
import { Elm } from "./build/elm.js";

set_panic_hook();

customElements.define(
  'wasm-wrapper',
  class extends HTMLElement {
    constructor() {
      super();
      this.container = Container.new();
    }
    connectedCallback() { this.setContent(); }
    attributeChangedCallback() { this.setContent(); }
    static get observedAttributes() { return ['coll-a', 'coll-b', 'coll-c']; }
    
    
    setContent() {
      let oldList = document.getElementById("pl-series");
      let colls = this.getAttributeNames().reduce(
        (c, s) => s.startsWith('coll-') ? c.concat([s.slice(5)]) : c,
        []
      );
      let newItem = this.container.to_html(colls=colls);
      while (oldList = document.getElementById("pl-series"))
        oldList.remove();
      this.appendChild(newItem);
    }
    
  }
);

var app = Elm.Main.init({
  node: document.getElementById('myapp')
});
