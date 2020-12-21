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
    vm::executer::code_executer(&r#"eğer 10 > 0:
    erhan=123
    io::printline('Oldu')
yada:
    erhan=321
    io::printline('Olmadı')
erhan+=1
io::printline('erhan barış')"#.to_string());
} 