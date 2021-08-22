use std::borrow::Borrow;
use std::{iter::Skip, rc::Rc, vec::Vec};
use std::cell::RefCell;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Take;
use bitflags::bitflags;

use crate::buildin::{DummyModule, Module};
use crate::compiler::scope::Scope;
use crate::error::KaramelErrorType;
use crate::{inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::*;
use crate::compiler::context::KaramelCompilerContext;

use super::module::OpcodeModule;
use super::{KaramelPrimative, StaticStorage};
use super::ast::KaramelAstType;
use super::storage_builder::StorageBuilder;

pub type NativeCallResult = Result<VmObject, KaramelErrorType>;
pub type NativeCall       = fn(FunctionParameter) -> NativeCallResult;
pub type IndexerGetCall   = fn (VmObject, f64) -> NativeCallResult ;
pub type IndexerSetCall   = fn (VmObject, f64, VmObject) -> NativeCallResult ;

#[derive(Debug)]
pub struct FunctionParameter<'a> {
    stack: &'a [VmObject], 
    source: Option<VmObject>, 
    last_position: usize, 
    arg_size: u8,
    stdout: &'a Option<RefCell<String>>,
    stderr: &'a Option<RefCell<String>>
}

pub struct FunctionParameterIterator<'a> {
    iter: Take<Skip<Iter<'a, VmObject>>>
}

impl<'a> FunctionParameter<'a> {
    pub fn new(stack: &'a [VmObject], source: Option<VmObject>, last_position: usize, arg_size: u8, stdout: &'a Option<RefCell<String>>, stderr: &'a Option<RefCell<String>>) -> Self {
        FunctionParameter { stack, source, last_position, arg_size, stdout, stderr }
    }

    pub fn source(&self) -> Option<VmObject> {
        match &self.source {
            Some(primative) => Some(*primative),
            None => None
        }
    }

    pub fn length(&self) -> u8 {
        self.arg_size
    }

    pub fn write_to_stdout<'b>(&self, data: &'b str) {
        match self.stdout {
            Some(out) => match out.try_borrow_mut() {
                Ok(mut out_mut) => out_mut.push_str(data),
                _ => println!("{}", data)
            },
            _ => println!("{}", data)
        };
    }

    pub fn iter(&self) -> FunctionParameterIterator {
        FunctionParameterIterator 
        { 
            iter: self.stack.iter().skip((self.last_position as usize - 1) - (self.arg_size as usize - 1)).take(self.arg_size as usize).clone()
        }
    }
}

impl<'a> Iterator for FunctionParameterIterator<'a> {
    type Item = &'a VmObject;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

bitflags! {
    #[derive(Default)]
    pub struct FunctionFlag: u32 {
        const NONE         = 0b00000000;
        const STATIC       = 0b00000001;
        const IN_CLASS     = 0b00000010;
        const MODULE_LEVEL = 0b00000100;
    }
}

#[derive(Clone)]
pub struct FunctionReference {
    pub callback: FunctionType,
    pub flags: FunctionFlag,
    pub name: String,
    pub arguments: Vec<String>,
    pub defined_storage_index: usize,
    pub storage_index: usize,
    pub opcode_location: Cell<usize>,
    pub used_locations: RefCell<Vec<u16>>,
    pub opcode_body: Option<Rc<KaramelAstType>>,
    pub module: Rc<dyn Module>
}

unsafe impl Send for FunctionReference {}
unsafe impl Sync for FunctionReference {}

#[derive(Clone)]
pub enum FunctionType {
    Native(NativeCall),
    Opcode
}

impl Default for FunctionType {
    fn default() -> Self { FunctionType::Opcode }
}

impl FunctionReference {
    pub fn execute(&self, compiler: &mut KaramelCompilerContext, base: Option<VmObject>) -> Result<(), KaramelErrorType>{
        unsafe {
            match self.callback {
                FunctionType::Native(func) => FunctionReference::native_function_call(&self, func, compiler, base),
                FunctionType::Opcode => FunctionReference::opcode_function_call(&self,  compiler)
            }
        }
    }

    pub fn buildin_function(func: NativeCall, name: String, flags: FunctionFlag) -> Rc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            flags: flags,
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0,
            opcode_body: None,
            module: Rc::new(DummyModule::new())
        };
        Rc::new(reference)
    }

    pub fn native_function(func: NativeCall, name: String, module: Rc<dyn Module>) -> Rc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            flags: FunctionFlag::STATIC,
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0,
            opcode_body: None,
            module
        };
        Rc::new(reference)
    }

    pub fn opcode_function(name: String, arguments: Vec<String>, body: Rc<KaramelAstType>, module: Rc<dyn Module>, storage_index: usize, defined_storage_index: usize, module_level: bool) -> Rc<FunctionReference> {
        let mut reference = FunctionReference {
            callback: FunctionType::Opcode,
            flags: FunctionFlag::STATIC,
            module,
            name,
            arguments,
            storage_index,
            defined_storage_index,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            opcode_body: Some(body.clone())
        };

        if module_level {
            reference.flags = reference.flags | FunctionFlag::MODULE_LEVEL;
        }

        Rc::new(reference)
    }

    unsafe fn native_function_call(reference: &FunctionReference, func: NativeCall, compiler: &mut KaramelCompilerContext, source: Option<VmObject>) -> Result<(), KaramelErrorType> {            
        let total_args                 = *compiler.opcodes_ptr.offset(1);
        let call_return_assign_to_temp = *compiler.opcodes_ptr.offset(2) != 0;
        let parameter = FunctionParameter::new(&compiler.stack, source, get_memory_index!(compiler) as usize, karamel_dbg!(total_args), &compiler.stdout, &compiler.stderr);

        dump_data!(compiler, "native_function_call");
        
        match func(parameter) {
            Ok(result) => {
                dec_memory_index!(compiler, total_args as usize);

                if call_return_assign_to_temp {
                    *compiler.stack_ptr = result;
                    inc_memory_index!(compiler, 1);
                }

                compiler.opcodes_ptr = compiler.opcodes_ptr.offset(2);
                Ok(())
            },
            Err(error) => {
                dec_memory_index!(compiler, total_args as usize);
                println!("{:?}", error);
                Err(error)
            }
        }
    }

    fn opcode_function_call(reference: &FunctionReference, options: &mut KaramelCompilerContext) -> Result<(), KaramelErrorType> {
        unsafe {
            let argument_size              = *options.opcodes_ptr.offset(1);
            let call_return_assign_to_temp = *options.opcodes_ptr.offset(2) != 0;
            let old_index                  = options.opcodes_ptr.offset(2);
            let location = reference.opcode_location.get() as isize;
            options.opcodes_ptr            = options.opcodes_top_ptr.offset(location);
            options.scope_index           += 1;

            if argument_size != *options.opcodes_ptr {
                return Err(KaramelErrorType::FunctionArgumentNotMatching {
                    function: reference.name.to_string(),
                    expected: argument_size, 
                    found: *options.opcodes_ptr
                });
            }

            dec_memory_index!(options, argument_size.into());
            dump_data!(options, "Current");

            if options.scopes.len() <= options.scope_index {
                options.scopes.resize(options.scopes.len() * 2, Scope::empty());
                options.scopes_ptr = options.scopes.as_mut_ptr();
            }

            let mut scope = options.scopes_ptr.add(options.scope_index);
            let storage = options.storages_ptr.add(reference.storage_index);

            (*scope).constant_ptr = (*storage).constants.as_ptr();
            (*scope).top_stack = options.stack_ptr;

            (*scope).location                   = old_index;
            (*scope).call_return_assign_to_temp = call_return_assign_to_temp;

            options.current_scope = scope;
            inc_memory_index!(options, argument_size.into());
        }
        Ok(())
    }
}

pub fn find_function_definition_type(module: Rc<OpcodeModule>, ast: Rc<KaramelAstType>, options: &mut KaramelCompilerContext, current_storage_index: usize, module_level: bool) -> CompilerResult {
    match ast.borrow() {
        KaramelAstType::FunctionDefination { name, arguments, body  } => {
            /* Create new storage for new function */
            let new_storage_index = options.storages.len();
            options.storages.push(StaticStorage::new(new_storage_index));
            options.storages[new_storage_index].set_parent_location(current_storage_index);

            let function = FunctionReference::opcode_function(name.to_string(), arguments.to_vec(), body.clone(), module.clone(), new_storage_index, current_storage_index, module_level);
            let old_function = module.functions.borrow_mut().insert(name.to_string(), function.clone());

            if let Some(_) = old_function {
                return Err(KaramelErrorType::FunctionAlreadyDefined(name.to_string()));
            }
            
            find_function_definition_type(module.clone(), body.clone(), options, new_storage_index, false)?;

            let storage_builder = StorageBuilder::new();
            storage_builder.prepare(module.clone(), ast.borrow(), new_storage_index, options)?;

            options.storages[current_storage_index].add_constant(Rc::new(KaramelPrimative::Function(function.clone(), None)));

            for argument in arguments {
                options.storages[new_storage_index].add_variable(argument);
            }
        },
        KaramelAstType::Block(blocks) => {
            for block in blocks {
                find_function_definition_type(module.clone(), block.clone(), options, current_storage_index, module_level)?;
            }
        },
        _ => ()
    }

    Ok(())
}