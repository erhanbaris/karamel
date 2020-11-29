mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&r#"erhan=num::parse('1024') - 24"#.to_string());
}
