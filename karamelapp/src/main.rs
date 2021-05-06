extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
data = 'erhan'
data[0] = '1'
gç::satıryaz('erhan'[0])
"#.to_string()),
        return_opcode: true,
        return_output: true
    };
    
    let result = karamellib::vm::executer::code_executer(parameters);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

