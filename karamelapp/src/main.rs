extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
listem = ['merhaba', 'dünya']

hataayıklama::doğrula(listem.sil(0), "merhaba")
hataayıklama::doğrula(listem.uzunluk(), 1)

hataayıklama::doğrula(listem.sil(0), "dünya")
hataayıklama::doğrula(listem.uzunluk(), 0)
        
        
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

