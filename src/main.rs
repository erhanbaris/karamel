mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

fn main() {
    vm::executer::code_executer(&r#"erhan = 1 + 12
baris=2 * 12
erhanbaris = erhan + baris"#.to_string());
}
