mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

fn main() {
    vm::executer::code_executer(&"erhan = 'asd'".to_string());
}
