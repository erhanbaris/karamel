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
    fn test_erhan:
        döndür 'erhan'

    fn test_barış:
        döndür 'barış'

    döndür test_erhan() + " " + test_barış()
hataayıklama::doğrula(test(), 'erhan barış')
"#.to_string());
}