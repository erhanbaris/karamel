use crate::{types::Token, vm::interpreter::run_vm};
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;
use crate::logger::{CONSOLE_LOGGER};
use crate::error::generate_error_message;

use log;
use crate::types::VmObject;


pub enum ExecutionSource {
    Code(String),
    File(String)
}

pub struct ExecutionParameters {
    pub source: ExecutionSource,
    pub return_opcode: bool,
    pub return_output: bool
}

#[derive(Default)]
pub struct ExecutionStatus {
    pub compiled: bool,
    pub executed: bool,
    pub memory_output: Option<Vec<VmObject>>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub opcodes: Option<Vec<Token>>
}

pub fn code_executer(parameters: ExecutionParameters) -> ExecutionStatus {
    let mut status = ExecutionStatus::default();
    match log::set_logger(&CONSOLE_LOGGER) {
        Ok(_) => {
            if cfg!(debug_assertions) {
                log::set_max_level(log::LevelFilter::Debug)
            } else {
                log::set_max_level(log::LevelFilter::Info)
            }
        },
        _ => ()
    };

    let data = match parameters.source {
        ExecutionSource::Code(code) => code,
        ExecutionSource::File(_) => {
            log::error!("Kaynak dosyasından çalıştırma desteklenmektedir.");
            return status
        }
    };

    let mut parser = Parser::new(&data);
    match parser.parse() {
        Err(error) => {
            log::error!("{}", generate_error_message(&data, &error));
            return status;
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    let ast = match syntax.parse() {
        Ok(ast) => ast,
        Err(error) => {
            log::error!("{}", generate_error_message(&data, &error));
            return status;
        }
    };

    let opcode_compiler = InterpreterCompiler {};
    let mut compiler_options: BramaCompiler = BramaCompiler::new();

    if parameters.return_output {
        compiler_options.stdout = Some(String::new());
        compiler_options.stderr = Some(String::new());
    }

    let execution_status = match opcode_compiler.compile(&ast, &mut compiler_options) {
        Ok(_) => unsafe { run_vm(&mut compiler_options) },
        Err(message) => {
            log::error!("Program hata ile sonlandırıldı: {}", message);
            return status;
        }
    };

    match execution_status {
        Ok(memory) => {
            status.compiled = true;
            status.executed = true;
            status.memory_output = Some(memory)
        },
        Err(error) => {
            log::error!("Program hata ile sonlandırıldı: {}", error);
            return status;
        }
    };

    log::info!("Program başarıyla çalıştırıldı");
    if parameters.return_opcode {
        status.opcodes = Some(parser.tokens());
    }

    status.stdout = compiler_options.stdout;
    status.stderr = compiler_options.stderr;

    status
}
