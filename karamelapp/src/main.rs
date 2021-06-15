use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
extern crate karamellib;

use karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

fn main() {
    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(r#"
fonk recur_fibo(n):
    n <= 1 ise:
        döndür n
    veya:
        döndür (recur_fibo(n-1) + recur_fibo(n-2))
gç::satıryaz(recur_fibo(41))  
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

