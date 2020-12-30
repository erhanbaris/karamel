use crate::vm::interpreter::run_vm;
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;

use std::io::{self};
use std::io::Write;

#[allow(dead_code)]
pub fn code_executer(data: &String) -> Result<(), String> {
    let mut compiler_options: BramaCompiler = BramaCompiler::new();

    let mut parser = Parser::new(&data[..]);
    match parser.parse() {
        Err((message, line, column)) => {
            println!("[{:?}:{:?}] {:?}", line, column, message);
            return Err(message.to_string());
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    match syntax.parse() {
        Ok(ast) => {
            let opcode_compiler = InterpreterCompiler {};

            let execution_status = match opcode_compiler.compile(&ast, &mut compiler_options) {
                Ok(_) => unsafe { run_vm(&mut compiler_options) },
                Err(message) => Err(message.to_string())
            };
            execution_status
        },
        Err((message, line, column)) => {
            println!("[{}:{}] {}", line, column, message);
            Err(message.to_string())
        }
    }
}

#[allow(dead_code)]
fn console_welcome() {
    println!("Türkçe Programlama Dili (TPD) komut satırı");
    print!("-> ");
    io::stdout().flush().unwrap();
}

#[allow(dead_code)]
pub fn console_executer() {
    console_welcome();
    
    let mut compiler_options: BramaCompiler = BramaCompiler::new();
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

        let mut parser = Parser::new(&line.trim());
        match parser.parse() {
            Err(error) => {
                println!("Err > {:?}", error);
                return;
            },
            _ => ()
        };

        let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
        let syntax_result = syntax.parse();

        match syntax_result {
            Ok(ast) => {
                let opcode_compiler = InterpreterCompiler {};
                compiler_options.reset();
                
                if let Ok(_) = opcode_compiler.compile(&ast, &mut compiler_options) {
                    if let Err(message) = unsafe { run_vm(&mut compiler_options) } {
                        println!("{}", message);
                    }
                }
            },
            Err((message, _, _)) => {
                println!("Err > {:?}", message);
                return;
            }
        }

        print!("-> ");
        io::stdout().flush().unwrap();
    }
}
