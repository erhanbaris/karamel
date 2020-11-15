mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;

use types::*;
use compiler::*;

fn parse(data: &'static str) {
    let mut parser = parser::Parser::new(&data);
    match parser.parse() {
        Err(_) => (),
        _ => ()
    };

    let syntax = types::SyntaxParser::new(Box::new(parser.tokens().to_vec()));
    let syntax_result = syntax.parse();

    if let Ok(ast) = syntax_result {
        let opcode_compiler      = compiler::InterpreterCompiler {};
        let mut compiler_options = BramaCompilerOption::new();

        opcode_compiler.prepare_variable_store(&ast, &mut compiler_options);
        
        if let Ok(_) = opcode_compiler.compile(&ast, &mut compiler_options) {
            vm::vm::run_vm(&mut compiler_options);
        }
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
    let data = unsafe { Box::from_raw(ptr as *mut compiler::VmObject) };
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
*/
    //vm::vm::run_vm(&opcodes);
    parse("değil true");
}