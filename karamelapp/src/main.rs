extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {
    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
soyisim = "barış"
soyisim[0] = "B"
soyisim[3] = "i"
soyisim[4] = "s"
soyisim[-1] = "!"
hataayıklama::doğrula(soyisim, "Baris")
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

