mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&"-10*-10".to_string());
}
