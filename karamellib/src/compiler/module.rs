use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::rc::Rc;

use crate::buildin::{Class, Module};
use crate::error::generate_error_message;
use crate::parser::Parser;
use crate::syntax::SyntaxParser;
use crate::types::CompilerResult;

use super::{BramaCompiler, BramaPrimative, StaticStorage};
use super::ast::BramaAstType;
use super::function::FunctionReference;

pub struct OpcodeModule {
    pub name: String,
    pub module_path: String,
    pub main_ast: BramaAstType,
    pub functions: HashMap<String, Rc<FunctionReference>>,
    pub modules: HashMap<String, Rc<dyn Module>>
}

impl OpcodeModule {
    pub fn new(name: String, module_path: String, main_ast: BramaAstType, functions: HashMap<String, Rc<FunctionReference>>, modules: HashMap<String, Rc<dyn Module>>) -> OpcodeModule {
        OpcodeModule {
            name, 
            module_path, 
            main_ast,
            functions,
            modules
        }
    }
}

impl Module for OpcodeModule {
    fn get_module_name(&self) -> String {
        self.name.to_string()
    }

    fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
        self.functions.get(name).map(|method| method.clone())
    }

    fn get_module(&self, _: &str) -> Option<Rc<dyn Module>> {
        None
    }

    fn get_methods(&self) -> Vec<(&String, Rc<FunctionReference>)> {
        self.functions.iter().map(|(key, value)| (key, value.clone())).collect::<Vec<(&String, Rc<FunctionReference>)>>()
    }

    fn get_modules(&self) -> HashMap<String, Rc<dyn Module>> {
        HashMap::new()
    }

    fn get_classes(&self) -> Vec<Rc<dyn Class>> {
        Vec::new()
    }
}

pub fn load_module(params: &[String], options: &mut BramaCompiler) -> Result<OpcodeModule, String> {
    let mut path = PathBuf::from(&options.script_path[..]);
    let module = params[(params.len() - 1)].to_string();

    for item in params.iter().take(params.len() - 1) {
        path.push(item);
    }

    path.push(format!("{}.tpd", module));

    if path.is_file() {
        let content = match File::open(path.to_str().unwrap()) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            },
            Err(error) => return Err(format!("Dosya okuma hatası oldu. Hata : {:?}", error))
        };

        let mut parser = Parser::new(&content);
        match parser.parse() {
            Err(error) => return Err(generate_error_message(&content, &error)),
            _ => ()
        };

        let syntax = SyntaxParser::new(parser.tokens().to_vec());
        return match syntax.parse() {
            Ok(ast) => {
                let mut functions : HashMap<String, Rc<FunctionReference>> = HashMap::new();
                let modules : HashMap<String, Rc<dyn Module>> = HashMap::new();
                find_function_definition_type(&ast, &mut functions, options, 0)?;
                let module = OpcodeModule::new(module, path.to_str().unwrap().to_string(), ast, functions, modules);
                Ok(module)
            },
            Err(error) => return Err(generate_error_message(&content, &error))
        };
    }

    Err(format!("'{}' modül bulunamadı", module))
}

fn find_load_type(ast: &BramaAstType, options: &mut BramaCompiler, modules: &mut Vec<Rc<OpcodeModule>>, depth_level: usize) -> CompilerResult {
    if depth_level > 1 {
        return Ok(())
    }

    match ast {
        BramaAstType::Load(module_name) => {
            modules.push(Rc::new(load_module(module_name, options)?));
            find_load_type(ast, options, modules, depth_level + 1)?;
        },
        BramaAstType::Block(blocks) => {
            for block in blocks {
                find_load_type(block, options, modules, depth_level + 1)?;
            }
        },
        _ => ()
    }

    Ok(())
}

fn find_function_definition_type(ast: &BramaAstType, functions: &mut HashMap<String, Rc<FunctionReference>>, options: &mut BramaCompiler, depth_level: usize) -> CompilerResult {
    if depth_level > 1 {
        return Ok(());
    }

    match ast {
        BramaAstType::FunctionDefination { name, arguments, body  } => {
            /* Create new storage for new function */
            let new_storage_index = options.storages.len();
            let function = FunctionReference::opcode_function(name.to_string(), arguments.to_vec(), body.clone(), Vec::new(), new_storage_index, 0);
            functions.insert(name.to_string(), function.clone());
            
            //options.storages[storage_index].add_constant(functi
            options.storages.push(StaticStorage::new());
            options.storages[new_storage_index].set_parent_location(0);
            options.storages[0].add_static_data(name, Rc::new(BramaPrimative::Function(function.clone(), None)));

            for argument in arguments {
                options.storages[new_storage_index].add_variable(argument);
            }
        },
        BramaAstType::Block(blocks) => {
            for block in blocks {
                find_function_definition_type(block, functions, options, depth_level + 1)?;
            }
        },
        _ => ()
    }

    Ok(())
}

pub fn get_modules(main_ast: &BramaAstType, options: &mut BramaCompiler) -> Result<Vec<Rc<OpcodeModule>>, String> {
    let mut modules: Vec<Rc<OpcodeModule>> = Vec::new();
    match find_load_type(main_ast, options, &mut modules, 0) {
        Ok(()) => Ok(modules),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    use crate::compiler::BramaCompiler;
    use crate::compiler::module::load_module;

    fn setup() {
        println!("setup");
    }

    fn teardown(to_be_removed: Vec<String>) {
        for file in to_be_removed.iter() {
            std::fs::remove_file(file).unwrap();
        }
        println!("teardown");
    }

    fn run_test<T>(test: T, to_be_removed: Vec<String>) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        setup();

        let result = panic::catch_unwind(|| {
            test()
        });

        teardown(to_be_removed);

        assert!(result.is_ok())
    }

    fn write_to_file(content: &'static str, file_name: &'static str) -> String {
        let file_name = generate_file_name(file_name);
        let mut file = File::create(&file_name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_name
    }

    fn get_parent() -> String {
        match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(parent_path) => parent_path.to_str().unwrap().to_string(),
                _ => String::from(".")
            },
            _ => String::from(".")
        }
    }

    fn generate_file_name(file_name: &'static str) -> String {
        match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(parent_path) => parent_path.clone().join(file_name).to_str().unwrap().to_string(),
                _ => Path::new(".").join(file_name).to_str().unwrap().to_string()
            },
            _ => Path::new(".").join(file_name).to_str().unwrap().to_string()
        }
    }

    #[test]
    fn test_1() {

        let module_1 = r#"
fonk topla(bir, iki): dondur bir + iki"#;
        let topla_path = write_to_file(module_1, "topla.tpd");

        run_test(|| {
            let mut options = BramaCompiler::new();
            options.script_path = get_parent();
            match load_module(&[String::from("topla")].to_vec(), &mut options) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
        }, [topla_path].to_vec());
    }

    #[test]
    fn test_2() {
        let module_1 = r#"
fonk topla(bir, iki): dondur bir + iki"#;
        let module_2 = r#"
module_1 yükle
fonk topla2(bir, iki): dondur module_1::topla(bir + iki)"#;
        let module_1_path = write_to_file(module_1, "module_1.tpd");
        let module_2_path = write_to_file(module_2, "module_2.tpd");

        run_test(|| {
            let mut options = BramaCompiler::new();
            options.script_path = get_parent();
            match load_module(&[String::from("module_1")].to_vec(), &mut options) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
            match load_module(&[String::from("module_2")].to_vec(), &mut options) {
                Ok(_) => (),
                Err(error) => assert!(false, "{}", error)
            };
        }, [module_1_path, module_2_path].to_vec());
    }
}