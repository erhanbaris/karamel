extern crate karamellib;

use karamellib::logger::CONSOLE_LOGGER;

fn main() {
    let result = karamellib::vm::executer::code_executer(&r#"[123"#.to_string(), &CONSOLE_LOGGER);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

