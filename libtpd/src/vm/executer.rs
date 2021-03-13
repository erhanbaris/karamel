use crate::vm::interpreter::run_vm;
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;
use crate::logger::{ConsoleLogger};

use std::io::{self};
use std::io::Write;

use log;
use crate::types::VmObject;

//#[cfg(any(feature = "test", test))]


#[allow(dead_code)]
pub fn code_executer(data: &String) -> Result<Vec<VmObject>, String> {
    match log::set_logger(&ConsoleLogger { }) {
        Ok(_) => {
            if cfg!(debug_assertions) {
                log::set_max_level(log::LevelFilter::Debug)
            } else {
                log::set_max_level(log::LevelFilter::Info)
            }
        },
        _ => ()
    };

    let mut parser = Parser::new(data);
    match parser.parse() {
        Err((message, line, column)) => {
            log::debug!("[{:?}:{:?}] {:?}", line, column, message);
            return Err(message.to_string());
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    let ast = match syntax.parse() {
        Ok(ast) => ast,
        Err((message, line, column)) => {
            log::debug!("[{}:{}] {}", line, column, message);
            return Err(message.to_string());
        }
    };

    let opcode_compiler = InterpreterCompiler {};
    let mut compiler_options: BramaCompiler = BramaCompiler::new();

    match opcode_compiler.compile(&ast, &mut compiler_options) {
        Ok(_) => unsafe { run_vm(&mut compiler_options) },
        Err(message) => Err(message.to_string())
    }
}

#[allow(dead_code)]
fn console_welcome() {
    println!("Türkçe Programlama Dili (TPD) komut satırı");
    print!("-> ");
    io::stdout().flush().unwrap();
}
