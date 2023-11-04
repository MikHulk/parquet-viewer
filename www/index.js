import { get_content } from "./build/wasm-pkg/parquet_viewer.js";
import { Elm } from "./build/elm.js";

customElements.define(
  'wasm-wrapper',
  class extends HTMLElement {
    constructor() {
      super();
      this.content = null;
    }
    connectedCallback() { this.setContent(); }
    attributeChangedCallback() { this.setContent(); }
    static get observedAttributes() { return ['name']; }
    
    setContent()
    {
      if (this.content) {
        this.removeChild(this.content);
      }
      this.content = get_content(this.getAttribute('name'));
      this.appendChild(this.content);
    }
  }
);

var app = Elm.Main.init({
  node: document.getElementById('myapp')
});
