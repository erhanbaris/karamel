mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

fn main() {
    vm::executer::code_executer(&r#"erhan='erhan' + 'barış' + 'erhan'+'barış'"#.to_string());
}
/*
c = a(1) + a(2)
1
2
*/