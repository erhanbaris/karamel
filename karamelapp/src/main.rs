extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;
use karamellib::error::BramaErrorType;

fn main() {

    println!("{}", BramaErrorType::FunctionCallSyntaxNotValid.as_text());
    let result = karamellib::vm::executer::code_executer(&r#"gç::sa tıryaz("Sonsuza kadar devam")"#.to_string(), &CONSOLE_LOGGER);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

