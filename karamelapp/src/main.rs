extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {
    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
data = { 'key_1': 'evet' }
data.key_1 = 'hayır'
data.güncelle('key_2', 'erhan')
hataayıklama::doğrula(data.getir('key_2'), 'erhan')
hataayıklama::doğrula(data.uzunluk(), 2)   
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

