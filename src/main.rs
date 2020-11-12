mod types;
mod parser;
mod syntax;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = syntax::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    println!("{:?}", syntax.parse());


    //info!("{:?}", syntax.primary_expr());
    //syntax.primary_expr();
}

fn main() {
    parse("data++");
}