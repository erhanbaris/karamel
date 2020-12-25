#[macro_use]
extern crate bitflags;

mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod buildin;

#[cfg(feature = "wasmBuild")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasmBuild")]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[cfg(feature = "wasmBuild")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


#[cfg(not(feature = "wasmBuild"))]
fn main() {
    vm::executer::code_executer(&r#"
fn test:
    gç::satıryaz("123")
    erhan=123
    gç::satıryaz(1024 * 12)
    gç::satıryaz(erhan)

gç::satıryaz('Oncesi')
test()
gç::satıryaz('Sonrası')
"#.to_string());
}