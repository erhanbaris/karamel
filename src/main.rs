mod types;
mod parser;
fn parse(data: &'static str) {
    let mut parser = parser::Parser {};
    parser.parse(&data);
}

fn main() {
    parse("erhan barış");
}