mod types;
mod parser;
mod syntax;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    parser.parse();

    let mut syntax = syntax::SyntaxParser::new(parser.tokens());
}

fn main() {
    parse("1024");
}