use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

fn main() {
    check("Hello".to_string());
    check("Hello");
    
    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
dosya yükle
fonk func:
    fonk inner_1:
        fonk inner_2:
            döndür 'oldu'
        döndür inner_2
    döndür inner_1
gç::satıryaz(func()()())
gç::satıryaz(dosya::topla())
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

