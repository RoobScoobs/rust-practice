/***
 * 
 * 
    IMPLEMENTING THE WASM-BINDGEN CRATE TO WRITE WASM

    The code exposed to JavaScript is as simple as adding the wasm_bindgen attribute

    Able to use the wasm-pack to build a JavaScript package that contains the compiled Wasm code with one simple command:
        wasm-pack build

    The wasm-pack build process will produce the following output:

        hello-bindgen/pkg/
            ├── hello_bindgen.d.ts
            ├── hello_bindgen.js
            ├── hello_bindgen_bg.d.ts
            ├── hello_bindgen_bg.wasm
            └── package.json

    Easiest way to see that the greet function was exported is to look at the generated TypeScript definition file

    LINKING WITH JAVASCRIPT

    npm init wasm-app app (name of the folder)

    THE WASM-BINDGEN CRATE

    The crate generates some Rust code based on where the attribute is placed

    The greet function was annotated so the crate will generate a wrapper function
    that handles marshalling the complex data types,
    &str and String in this case, into integers that Wasm can handle

    You can put the wasm_bindgen attribute on structs, impl blocks,
    and a variety of other Rust items to expose them to JavaScript

    Refer to: https://rustwasm.github.io/docs/wasm-bindgen/

    The next step is that wasm-pack uses the wasm-bindgen CLI to
    generate JavaScript wrapper code based on items annotated with the wasm_bindgen attribute
***/

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}