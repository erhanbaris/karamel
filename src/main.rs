mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&r#"print(1)"#.to_string());
}
