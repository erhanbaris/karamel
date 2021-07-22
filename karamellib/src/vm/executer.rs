use std::borrow::Borrow;
use std::cell::RefCell;

use crate::compiler::context::{ExecutionPathInfo, KaramelCompilerContext};
use crate::file::read_module_or_script;
use crate::{types::Token, vm::interpreter::run_vm};
use crate::parser::*;
use crate::compiler::*;
use crate::syntax::SyntaxParser;
use crate::logger::{CONSOLE_LOGGER};
use crate::error::generate_error_message;

use log;
use crate::types::VmObject;


pub enum ExecutionSource<'a> {
    Code(&'a str),
    File(&'a str)
}

pub struct ExecutionParameters<'a> {
    pub source: ExecutionSource<'a>,
    pub return_output: bool
}


pub struct ExecutionStatus<'a> {
    pub compiled: bool,
    pub executed: bool,
    pub memory_output: Option<Vec<VmObject>>,
    pub stdout: Option<RefCell<String>>,
    pub stderr: Option<RefCell<String>>,
    pub context: KaramelCompilerContext<'a>
}

impl<'a> Default for ExecutionStatus<'a> {
    fn default() -> Self {
        ExecutionStatus {
            compiled: false,
            executed: false,
            memory_output: None,
            stderr: None,
            stdout: None,
            context: KaramelCompilerContext::new()
        }
    }
}

pub fn get_execution_path<'a, T: Borrow<ExecutionSource<'a>>>(source: T) -> ExecutionPathInfo {
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

pub fn code_executer<'a>(parameters: &'a ExecutionParameters<'a>) -> ExecutionStatus<'a> {
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

    let mut status = ExecutionStatus::default();
    status.context.execution_path = get_execution_path(&parameters.source);
    log::debug!("Execution path: {}", status.context.execution_path.path);

    let data = match &parameters.source {
        ExecutionSource::Code(code) => *code,
        ExecutionSource::File(filename) => {
            match read_module_or_script(&filename[..], &status.context) {
                Ok(content) => content,
                Err(error) => {
                    log::error!("Program hata ile sonlandırıldı: {}", error);
                    return ExecutionStatus::default();
                }
            }
        }
    };

    let mut parser = Parser::new(&data);
    match parser.parse() {
        Err(error) => {
            log::error!("{}", generate_error_message(&data, &error));
            return ExecutionStatus::default();
        },
        _ => ()
    };

    let syntax = SyntaxParser::new(parser.tokens().to_vec());
    let ast = match syntax.parse() {
        Ok(ast) => ast,
        Err(error) => {
            log::error!("{}", generate_error_message(&data, &error));
            return ExecutionStatus::default();
        }
    };

    let opcode_compiler = InterpreterCompiler::new();
    if parameters.return_output {
        status.context.stdout = Some(RefCell::new(String::new()));
        status.context.stderr = Some(RefCell::new(String::new()));
    }

    let execution_status = match opcode_compiler.compile(ast.clone(), &mut status.context) {
        Ok(_) => unsafe { run_vm(&mut status.context) },
        Err(message) => {
            log::error!("Program hata ile sonlandırıldı: {}", message);
            return ExecutionStatus::default();
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
            return ExecutionStatus::default();
        }
    };

    log::info!("Program başarıyla çalıştırıldı");

    status
}
