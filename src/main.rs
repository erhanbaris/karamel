mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

fn main() {
    vm::executer::code_executer(&r#"print('merhaba dünya')"#.to_string());
}
