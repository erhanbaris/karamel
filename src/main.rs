mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

fn main() {
    vm::executer::console_executer();
}
