mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::console_executer();
}
/*
c = a(1) + a(2)
1
2
*/