use crate::vm::interpreter::run_vm;
use crate::types::*;
use crate::parser::*;
use crate::compiler::*;

use std::io::{self, BufRead};

#[warn(dead_code)]
pub fn code_executer(data: &String) {
    let mut compiler_options: BramaCompilerOption<DynamicStorage> = BramaCompilerOption::new();

    let mut parser = Parser::new(&data[..]);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    let syntax_result = syntax.parse();

    if let Ok(ast) = syntax_result {
        let opcode_compiler = InterpreterCompiler {};

        opcode_compiler.prepare_variable_store(&ast, &mut compiler_options);
        
        if let Ok(_) = opcode_compiler.compile(&ast, &mut compiler_options) {
            run_vm(&mut compiler_options);
        }
    }
}

#[warn(dead_code)]
pub fn command_line_executer() {
    let data = "";

    let mut compiler_options: BramaCompilerOption<DynamicStorage> = BramaCompilerOption::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        let mut parser = Parser::new("");
        match parser.parse() {
            Err(_) => (),
            _ => ()
        };

        let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
        let syntax_result = syntax.parse();

        if let Ok(ast) = syntax_result {
            let opcode_compiler = InterpreterCompiler {};

            opcode_compiler.prepare_variable_store(&ast, &mut compiler_options);
            
            if let Ok(_) = opcode_compiler.compile(&ast, &mut compiler_options) {
                run_vm(&mut compiler_options);
            }
        }
    }
}

