extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;
use karamellib::error::BramaErrorType;

fn main() {
    println!("{:?}", "\r\nfn test(a):\r\n".split(|c| c == '\n').collect::<Vec<_>>());
    println!("{}", BramaErrorType::FunctionCallSyntaxNotValid.as_text());
    let result = karamellib::vm::executer::code_executer(&r#"
fn test
    erhan=123"#.to_string(), &CONSOLE_LOGGER);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

