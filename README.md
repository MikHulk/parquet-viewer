# USAGE

all build step can be performed from npm in `./www`.

## Build Elm UI 

```
[/www]$ npm run build-ui

> parquet-view@1.0.0 build-ui
> ./build-ui

Success! Compiled 1 module.

    Main ───> ../elm.js
```

## Build WASM package


```
[/www]$ npm run build-wasm

> parquet-view@1.0.0 build-wasm
> wasm-pack build --out-dir wasm-pkg

[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
warning: unused manifest key: wasm-bindgen
   Compiling parquet-viewer v0.1.0 ()
    Finished release [optimized] target(s) in 0.06s
[INFO]: ⬇  Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 0.17s
[INFO]: 📦   Your wasm pkg is ready to publish at /wasm-pkg.
```

## Serve app
```
[/www]$ npm run serve

> parquet-view@1.0.0 serve
> ./dev_server.js

^C
```
