mod types;
mod parser;
mod syntax;
mod vm;
mod compiler;
mod core;

use crate::compiler::*;

fn main() {
    let mut collection: core::ModuleCollection = core::ModuleCollection::new();
    collection.add_module(&core::buildin::BuildinModule { });
    let func = collection.get_function("buildin".to_string(), "print".to_string());
    
    let mut storage = compiler::StaticStorage::new();
    match func {
        Some(function) => {
            function([].to_vec(), &mut storage);
        },
        None => ()
    };

    println!("{:?}", storage.get_variable_value(&"erhan".to_string()));
    vm::executer::code_executer(&r#"print('merhaba dünya')"#.to_string());
}
