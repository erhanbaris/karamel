extern crate bitflags;
extern crate termion;
extern crate log_update;

mod macros;
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
    let result = vm::executer::code_executer(&r#"
fn test():
    erhan=123
erhan = test
ee = erhan

gç::satıryaz(ee)
"#.to_string());
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}

