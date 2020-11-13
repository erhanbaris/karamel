mod types;
mod parser;
mod syntax;
mod vm;

use std::vec::Vec;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = types::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    println!("{:?}", syntax.parse());


    //info!("{:?}", syntax.primary_expr());
    //syntax.primary_expr();
}

fn main() {
    let mut opcodes = Vec::new();
    opcodes.push(vm::vm::BramaVmOpCode::Addition(1, 1));
    vm::vm::run_vm(&opcodes);
    parse("152%111");
}