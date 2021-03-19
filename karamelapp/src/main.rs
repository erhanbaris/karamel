extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;

fn main() {
    let result = karamellib::vm::executer::code_executer(&r#"gç::satıryaz('faktoriyel 10 => ', 10)"#.to_string(), &CONSOLE_LOGGER);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

