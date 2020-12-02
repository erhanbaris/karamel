mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&r#"100+200+300"#.to_string());
}
/*
c = a(1) + a(2)
1
2
*/