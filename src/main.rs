mod types;
mod parser;
fn parse(data: &'static str) {
    let mut parser = parser::Parser::new();
    parser.parse(&data);
}

fn main() {
    parse("\"erhan barış\"");
}