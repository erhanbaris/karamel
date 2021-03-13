extern crate karamellib;

fn main() {
    let result = karamellib::vm::executer::code_executer(&r#"erhan=100
++erhan
++erhan
++erhan"#.to_string());
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}

