extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
fonk test:
    fonk test_erhan:
        döndür 'erhan'
    döndür test_erhan
sonuç = test()().büyükharf()
gç::satıryaz(sonuç)"#.to_string()),
        return_opcode: true,
        return_output: true
    };
    
    let result = karamellib::vm::executer::code_executer(parameters);
    match result.executed {
        true => println!("Success"),
        false => println!("Fail")
    };
}

