mod types;
mod parser;
mod syntax;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new();
    parser.parse(&data);

    let mut syntax = syntax::SyntaxParser::new(parser.tokens());
}

fn main() {
    parse("123.4e+4");
}