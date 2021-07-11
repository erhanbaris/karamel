use std::borrow::Borrow;
use std::{cell::RefCell, ptr, rc::Rc};
use crate::buildin::num::{NumModule};

use crate::{buildin::{Class, Module, ModuleCollection, base_functions, class::{dict, get_empty_class, list, number, proxy, text}, debug, io}, compiler::scope::Scope};

use super::generator::OpcodeGenerator;
use super::{KaramelPrimative, StaticStorage, function::{FunctionReference, FunctionType, FunctionFlag}, module::OpcodeModule};

#[derive(Default)]
pub struct ExecutionPathInfo {
    pub path: String,
    pub script: Option<String>
}

pub struct KaramelCompilerContext {
    pub execution_path: ExecutionPathInfo,
    pub opcodes : Vec<u8>,
    pub storages: Vec<StaticStorage>,
    pub main_module: *mut OpcodeModule,
    pub modules: ModuleCollection,
    pub scopes: Vec<Scope>,
    pub current_scope: *mut Scope,
    pub scope_index: usize,
    pub functions : Vec<Rc<FunctionReference>>,
    pub classes : Vec<Rc<dyn Class >>,
    pub stdout: Option<RefCell<String>>,
    pub stderr: Option<RefCell<String>>,
    pub opcodes_ptr: *mut u8,
    pub primative_classes: Vec<Rc<dyn Class>>,
    pub opcode_generator: OpcodeGenerator
}

impl  KaramelCompilerContext {
    pub fn new() -> KaramelCompilerContext {
        let mut compiler = KaramelCompilerContext {
            execution_path: ExecutionPathInfo::default(),
            opcodes: Vec::new(),
            storages: vec![StaticStorage::new(0)],
            modules: ModuleCollection::new(),
            scopes: Vec::new(),
            current_scope: ptr::null_mut(),
            scope_index: 0,
            functions: Vec::new(),
            classes: Vec::new(),
            stdout: None,
            stderr: None,
            opcodes_ptr: ptr::null_mut(),
            primative_classes: Vec::new(),
            main_module: ptr::null_mut(),
            opcode_generator: OpcodeGenerator::new()
        };

        compiler.primative_classes.push(number::get_primative_class());
        compiler.primative_classes.push(text::get_primative_class());
        compiler.primative_classes.push(list::get_primative_class());
        compiler.primative_classes.push(dict::get_primative_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(get_empty_class());
        compiler.primative_classes.push(proxy::get_primative_class());
        compiler.primative_classes.push(get_empty_class());

        compiler.add_module(base_functions::BaseFunctionsModule::new());
        compiler.add_module(io::IoModule::new());
        compiler.add_module(NumModule::new());
        compiler.add_module(debug::DebugModule::new());

        for _ in 0..32{
            compiler.scopes.push(Scope::empty());
        }
        
        compiler.current_scope = &mut compiler.scopes[0] as *mut Scope;
        compiler
    }

    pub fn has_module(&self, module_path: &Vec<String>) -> bool {
        self.modules.has_module(module_path)
    }

    pub fn add_module(&mut self, module: Rc<dyn Module>) {
        self.modules.add_module(module.clone());

        for reference in module.clone().get_methods().iter() {
            self.add_function(reference.clone());
        }
    }

    pub fn add_function(&mut self, information: Rc<FunctionReference>) {
        self.functions.push(information);
    }

    pub fn add_class(&mut self, class_info: Rc<dyn Class + Sync + Send>) {
        self.classes.push(class_info.clone());
    }

    pub fn get_function<T: Borrow<String>>(&self, name: T, module_path: &Vec<String>, start_storage_index: usize) -> Option<Rc<FunctionReference>> {
        let mut search_storage = start_storage_index;
        loop {
            /* Search function with storage */
            for (_, module) in self.modules.iter() {
                for function_reference in module.get_methods().iter() {
                    let result = match &function_reference.callback {
                        FunctionType::Native(_) =>
                            function_reference.module.get_path() == module_path && 
                            &function_reference.name == name.borrow(),
                        FunctionType::Opcode => 
                            &function_reference.name == name.borrow() && 
                            function_reference.module.get_path() == module_path && 
                            (function_reference.defined_storage_index == search_storage || function_reference.flags.contains(FunctionFlag::MODULE_LEVEL))
                    };

                    if result {
                        return Some(function_reference.clone())
                    }
                }
            }
            
            search_storage = match self.storages[search_storage].get_parent_location() {
                Some(parent_storage_index) => match parent_storage_index == search_storage {
                    true => return None,
                    false => parent_storage_index.into()
                },
                None => return None
            };
        }
    }

    pub fn get_class(&self, value: &KaramelPrimative) -> Rc<dyn Class > {
        unsafe {
            self.primative_classes.get_unchecked(value.discriminant()).clone()
        }
    }

    pub fn find_class(&self, name: String, _module_path: &Vec<String>, _start_storage_index: usize) -> Option<Rc<dyn Class >> {
        let primative_search = self.primative_classes.iter().find(|&item| item.get_class_name() == name);
        match primative_search {
            Some(class) => Some(class.clone()),
            None => None
        }
    }

    pub fn reset(&mut self) {
        self.opcodes = Vec::new();
    }
}