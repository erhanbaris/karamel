mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

use compiler::*;


fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = types::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    let syntax_result = syntax.parse();

    if let Ok(ast) = syntax_result {
        let opcode_compiler      = compiler::InterpreterCompiler {};
        let mut compiler_options = BramaCompilerOption::new();

        opcode_compiler.prepare_variable_store(&ast, &mut compiler_options);
        
        if let Ok(_) = opcode_compiler.compile(&ast, &mut compiler_options) {
            vm::vm::run_vm(&mut compiler_options);
        }
    }
}

fn main() {
    parse("erhan = 'erhan' + 'barış' + 'aysel'");
}
