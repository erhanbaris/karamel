#[macro_use]
pub mod macros;
pub mod parser;
pub mod syntax;
pub mod types;
pub mod vm;
pub mod compiler;
pub mod buildin;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn execute_code(name: &str) {
    let result = vm::executer::code_executer(&name.to_string());
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}
