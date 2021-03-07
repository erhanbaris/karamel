extern crate libtpd;

fn main() {
    let result = libtpd::vm::executer::code_executer(&r#"1025*2"#.to_string());
    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Fail ({})", error)
    };
}

