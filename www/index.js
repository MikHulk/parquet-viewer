import {
  get_content,
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
    static get observedAttributes() { return ['name']; }
    
    
    setContent() {
      console.log("up");
      let oldEl = document.getElementById("greeting");
      let content = get_content(this.getAttribute('name'));
      if (oldEl)
        this.replaceChild(content, oldEl);
      else
        this.appendChild(content);
      let oldList = document.getElementById("pl-series");
      let colls = this.getAttributeNames().reduce(
        (c, s) => s.startsWith('coll-') ? c.concat([s.slice(5)]) : c,
        []
      );
      let newItem = this.container.to_html(colls=colls);
      if (oldList)
        this.replaceChild(newItem, oldList);
      else
        this.appendChild(newItem);
    }
    
  }
);

var app = Elm.Main.init({
  node: document.getElementById('myapp')
});
