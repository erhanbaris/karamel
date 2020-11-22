use crate::vm::interpreter::run_vm;
use crate::types::*;
use crate::parser::*;
use crate::compiler::*;

use std::io::{self};
use std::io::Write;

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

fn console_welcome() {
    println!("Türkçe Programlama Dili (TPD) komut satırı");
    print!("-> ");
    io::stdout().flush().unwrap();
}

#[warn(dead_code)]
pub fn console_executer() {
    console_welcome();
    
    let mut compiler_options: BramaCompilerOption<DynamicStorage> = BramaCompilerOption::new();
    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            _     => {
                println!("ERR >");
                return;
            }
        };

        if line.trim().len() == 0 {
            println!("EOL >");
            return;
        }

        let mut parser = Parser::new(&line);
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

        print!("-> ");
        io::stdout().flush().unwrap();
    }
}

