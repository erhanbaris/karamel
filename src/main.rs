mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&"print(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)".to_string());
}
