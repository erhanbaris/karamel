extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;

fn main() {
    let result = karamellib::vm::executer::code_executer(&r#"gç::satıryaz("Sonsuza kadar devam")"#.to_string(), &CONSOLE_LOGGER);
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}

