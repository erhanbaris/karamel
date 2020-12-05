mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

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
    vm::executer::code_executer(&r#"10 == 40.0 - 10"#.to_string());
}