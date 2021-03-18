use crate::{vm::interpreter::run_vm};
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;
use crate::error::generate_error_message;

use std::io::{self};
use std::io::Write;

use log;
use crate::types::VmObject;
use log::Log;

#[derive(Default)]
pub struct ExecutionStatus {
    pub compiled: bool,
    pub executed: bool,
    pub memory_output: Option<Vec<VmObject>>,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>
}

pub fn code_executer(data: &String, logger: &'static dyn Log) -> ExecutionStatus {
    let mut status = ExecutionStatus::default();
    match log::set_logger(logger) {
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
        Err(error) => {
            log::error!("{}", generate_error_message(data, &error));
            return status;
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    let ast = match syntax.parse() {
        Ok(ast) => ast,
        Err(error) => {
            log::error!("{}", generate_error_message(data, &error));
            return status;
        }
    };

    let opcode_compiler = InterpreterCompiler {};
    let mut compiler_options: BramaCompiler = BramaCompiler::new();

    let execution_memory = match opcode_compiler.compile(&ast, &mut compiler_options) {
        Ok(_) => unsafe { run_vm(&mut compiler_options) },
        Err(message) => {
            log::error!("Program hata ile sonlandirildi: {}", message);
            return status;
        }
    };
    log::info!("Program basariyla calistirildi");

    match execution_memory {
        Ok(memory) => {
            status.compiled = true;
            status.executed = true;
            status.memory_output = Some(memory)
        },
        Err(error) => {
            log::error!("Hafizada ki bilgileri okuma sitasinda hata olustur fakat program hatasiz olarak calisti. Hata mesaji: {}", error);
        }
    };

    status
}

#[allow(dead_code)]
fn console_welcome() {
    println!("Karamel Programlama Dili (KPD) komut satırı");
    print!("-> ");
    io::stdout().flush().unwrap();
}
