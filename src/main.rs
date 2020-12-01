mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&r#"erhan=num::parse('1024')-num::parse('24')-num::parse('500')-num::parse('255')"#.to_string());
}
/*
c = a(1) + a(2)
1
2
*/