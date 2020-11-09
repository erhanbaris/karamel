mod types;
mod parser;
mod syntax;
use std::mem;
use log::debug;
use log::info;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    parser.parse();

    let mut syntax = syntax::SyntaxParser::new(parser.tokens());
    info!("{:?}", syntax.primary_expr());
    //syntax.primary_expr();
}

fn main() {
    parse("[123,doğru,:erhan_barış,'merhaba dünya',1.3]");
}