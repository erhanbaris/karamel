use std::borrow::Borrow;
use std::cell::RefCell;

use crate::compiler::context::{ExecutionPathInfo, KaramelCompilerContext};
use crate::file::read_module_or_script;
use crate::{types::Token, vm::interpreter::run_vm};
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;
use crate::logger::{CONSOLE_LOGGER, write_stderr};
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
    pub return_output: bool,
    pub dump_opcode: bool,
    pub dump_memory: bool
}

#[derive(Default)]
pub struct ExecutionStatus {
    pub compiled: bool,
    pub executed: bool,
    pub memory_output: Option<Vec<VmObject>>,
    pub stdout: Option<RefCell<String>>,
    pub stderr: Option<RefCell<String>>,
    pub opcodes: Option<Vec<Token>>,
    pub memory_dump: Option<String>,
    pub opcode_dump: Option<String>
}

pub fn get_execution_path<T: Borrow<ExecutionSource>>(source: T) -> ExecutionPathInfo {
    ExecutionPathInfo {
        path: match source.borrow() {
            ExecutionSource::Code(_) => match std::env::current_exe() {
                Ok(path) => match path.parent() {
                    Some(parent_path) => parent_path.to_str().unwrap().to_string(),
                    _ => String::from(".")
                },
                _ => String::from(".")
            },
            ExecutionSource::File(file_name) => file_name.to_string()
        },
        script: None
    }
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

    let mut context: KaramelCompilerContext = KaramelCompilerContext::new();
    context.execution_path = get_execution_path(&parameters.source);
    log::debug!("Execution path: {}", context.execution_path.path);

    if parameters.return_output {
        context.stdout = Some(RefCell::new(String::new()));
        context.stderr = Some(RefCell::new(String::new()));
    }

    let data = match parameters.source {
        ExecutionSource::Code(code) => code,
        ExecutionSource::File(filename) => {
            match read_module_or_script(filename, &context) {
                Ok(content) => content,
                Err(error) => {
                    write_stderr(&context, format!("Program hata ile sonlandırıldı: {}", error));
                    log::error!("Program hata ile sonlandırıldı: {}", error);
                    status.stdout = context.stdout;
                    status.stderr = context.stderr;
                    
                    status.executed = false;
                    return status
                }
            }
        }
    };

    let mut parser = Parser::new(&data);
    match parser.parse() {
        Err(error) => {
            write_stderr(&context, generate_error_message(&data, &error));
            log::error!("{}", generate_error_message(&data, &error));
            status.stdout = context.stdout;
            status.stderr = context.stderr;

            return status;
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    let ast = match syntax.parse() {
        Ok(ast) => ast,
        Err(error) => {
            write_stderr(&context, generate_error_message(&data, &error));
            log::error!("{}", generate_error_message(&data, &error));
            status.stdout = context.stdout;
            status.stderr = context.stderr;

            return status;
        }
    };

    let opcode_compiler = InterpreterCompiler {};
    let execution_status = match opcode_compiler.compile(ast.clone(), &mut context) {
        Ok(_) => unsafe { run_vm(&mut context, parameters.dump_opcode, parameters.dump_memory) },
        Err(message) => {
            write_stderr(&context, format!("Program hata ile sonlandırıldı: {}", message));
            log::error!("Program hata ile sonlandırıldı: {}", message);
            status.stdout = context.stdout;
            status.stderr = context.stderr;

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
            write_stderr(&context, format!("Program hata ile sonlandırıldı: {}", error));
            log::error!("Program hata ile sonlandırıldı: {}", error);
            status.stdout = context.stdout;
            status.stderr = context.stderr;

            return status;
        }
    };

    log::info!("Program başarıyla çalıştırıldı");
    if parameters.return_opcode {
        status.opcodes = Some(parser.tokens());
    }

    status.stdout      = context.stdout;
    status.stderr      = context.stderr;
    status.memory_dump = context.memory_dump;
    status.opcode_dump = context.opcode_dump;

    status
}
