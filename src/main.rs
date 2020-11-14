use std::mem;
use std::ptr;

mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

use std::vec::Vec;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = types::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    let syntax_result = syntax.parse();

    println!("{:#?}", syntax_result);
    println!("");

    if let Ok(ast) = syntax_result {
        let mut storage = compiler::InnerStrorage::new();
        compiler::StorageFeeder::add_ast(&ast, &types::BramaAstType::None, &mut storage);
        println!("{:#?}", storage);
    }

    //info!("{:?}", syntax.primary_expr());
    //syntax.primary_expr();
}

fn main() {
    /*let mut ptr: u64 = 0;
    
    unsafe {
        let object = compiler::VmObject {
            marked: false,
            data: compiler::VmObjectType::Double(123456789.0)
        };
       
        ptr = &object as *const _ as u64;
    }
    let data = unsafe { Box::from_raw(ptr as *mut compiler::VmObject) };*/
    let abc = [
        vm::vm::BramaVmOpCode::Addition(1, 1, 1),
        vm::vm::BramaVmOpCode::Divition(1, 1, 1),
        vm::vm::BramaVmOpCode::Modulo(1, 1, 1)];
        
    let mut opcodes = Vec::new();
    opcodes.push(vm::vm::BramaVmOpCode::Addition(1, 1, 1));
    opcodes.push(vm::vm::BramaVmOpCode::Divition(1, 1, 1));
    opcodes.push(vm::vm::BramaVmOpCode::Modulo(1, 1, 1));
    
    for i in 0..opcodes.len() {
        let ptr_2 = &abc[i] as *const _ as u64;
        match opcodes[i] {
            vm::vm::BramaVmOpCode::Addition(a, b, c) => {println!("Addition");},
            vm::vm::BramaVmOpCode::Divition(a, b, c) => {println!("Divition");},
            vm::vm::BramaVmOpCode::Modulo(a, b, c) => {println!("Modulo");},
            _ => ()
        }
        println!("{:?}", ptr_2);
    }

    //vm::vm::run_vm(&opcodes);
    parse("[data2]");
}