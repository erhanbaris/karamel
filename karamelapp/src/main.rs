extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
a = 1
b = 2
c = 1

a eşittir b ise:
    gç::satıryaz('a eşittir b')
yada a eşittir c ise:  
    gç::satıryaz('a eşittir c')
yoksa:  
    gç::satıryaz('a hiçbirine eşit değil')
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

