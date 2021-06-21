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

use super::BramaCompiler;
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
                let mut modules : HashMap<String, Rc<dyn Module>> = HashMap::new();
                find_function_definition_type(&ast, &mut functions, options, 0)?;
                find_module_load_type(&ast, &mut modules, options, 0)?;
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
            match load_module(module_name, options) {
                Ok(module) => {
                    modules.push(Rc::new(module));
                    find_load_type(ast, options, modules, depth_level + 1)?;
                },
                Err(error) => return Err(error)
            };
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

fn find_module_load_type(ast: &BramaAstType, modules: &mut HashMap<String, Rc<dyn Module>>, options: &mut BramaCompiler, depth_level: usize) -> CompilerResult {
    if depth_level > 1 {
        return Ok(());
    }
/*
    match ast {
        BramaAstType::Load(module_name) => {
            let function_reference = FunctionReference::opcode_function(name.to_string(), arguments.to_vec(), Vec::new(),  0, 0);
            modules.insert(name.to_string(), function_reference);
        },
        BramaAstType::Block(blocks) => {
            for block in blocks {
                find_module_load_type(block, modules, options, depth_level + 1)?;
            }
        },
        _ => ()
    }*/

    Ok(())
}

fn find_function_definition_type(ast: &BramaAstType, functions: &mut HashMap<String, Rc<FunctionReference>>, options: &mut BramaCompiler, depth_level: usize) -> CompilerResult {
    if depth_level > 1 {
        return Ok(());
    }

    match ast {
        BramaAstType::FunctionDefination { name, arguments, body: _  } => {
            let function_reference = FunctionReference::opcode_function(name.to_string(), arguments.to_vec(), Vec::new(),  0, 0);
            functions.insert(name.to_string(), function_reference);
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

    fn setup() {

        let current_exe= std::env::current_exe().unwrap();

        println!("{:?}", current_exe);
        println!("{:?}", current_exe.parent());
        println!("setup");
    }

    fn teardown(to_be_removed: Vec<String>) {
        let parent_path = std::env::current_exe().unwrap().parent().unwrap();
        for file in to_be_removed.iter() {
            let path = parent_path.clone().join(file);
            std::fs::remove_file(path);
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

    fn generate_file_name(file_name: &'static str) -> String {
        let parent = match std::env::current_exe() {
            Ok(path) => match path.parent() {
                Some(parent_path) => parent_path,
                _ => Path::new(".")
            },
            _ => Path::new(".")
        };
        
        parent.clone().join(file_name).to_str().unwrap().to_string()
    }

    #[test]
    fn test_1() {

        let module_1 = r#"
fonk topla(bir, iki): dondur bir + iki"#;
        write_to_file(module_1, "topla");

        run_test(|| {    
            assert!(false);
        }, [String::from("foo.tpd")].to_vec());
    }
}