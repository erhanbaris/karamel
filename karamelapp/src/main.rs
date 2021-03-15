extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;
use karamellib::types::BramaError;

fn main() {

    println!("{}", BramaError::FunctionCallSyntaxNotValid.as_text());
    let result = karamellib::vm::executer::code_executer(&r#"gç::satıryaz("Sonsuza kadar devam")"#.to_string(), &CONSOLE_LOGGER);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

