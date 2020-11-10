mod types;
mod parser;
mod syntax;
use std::mem;
use log::debug;
use log::info;

use types::SyntaxParserTrait;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    parser.parse();

    let mut syntax = syntax::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    println!("{:?}", syntax::primative::PrimativeParser::parse(&syntax));


    //info!("{:?}", syntax.primary_expr());
    //syntax.primary_expr();
}

fn main() {
    parse("[[]]");
}