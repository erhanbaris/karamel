use std::{iter::Skip, rc::Rc, vec::Vec};
use std::cell::RefCell;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Take;
use bitflags::bitflags;

use crate::{inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::*;
use crate::compiler::value::EMPTY_OBJECT;
use crate::compiler::{BramaCompiler, Scope};

pub type NativeCallResult = Result<VmObject, String>;
pub type NativeCall       = fn(FunctionParameter) -> NativeCallResult;
pub type IndexerGetCall   = fn (VmObject, f64) -> NativeCallResult ;
pub type IndexerSetCall   = fn (VmObject, f64, VmObject) -> NativeCallResult ;

pub struct FunctionParameter<'a> {
    stack: &'a Vec<VmObject>, 
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
    pub fn new(stack: &'a Vec<VmObject>, source: Option<VmObject>, last_position: usize, arg_size: u8, stdout: &'a Option<RefCell<String>>, stderr: &'a Option<RefCell<String>>) -> Self {
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
        const NONE     = 0b00000000;
        const STATIC   = 0b00000001;
        const IN_CLASS = 0b00000010;
    }
}

#[derive(Clone)]
#[derive(Default)]
pub struct FunctionReference {
    pub callback: FunctionType,
    pub flags: FunctionFlag,
    pub framework: String,
    pub module_path: Vec<String>,
    pub name: String,
    pub arguments: Vec<String>,
    pub defined_storage_index: usize,
    pub storage_index: usize,
    pub opcode_location: Cell<usize>,
    pub used_locations: RefCell<Vec<u16>>
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
    pub fn execute(&self, compiler: &mut BramaCompiler, base: Option<VmObject>) -> Result<(), String>{
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
            framework: "".to_string(),
            module_path: Vec::new(),
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0
        };
        Rc::new(reference)
    }

    pub fn native_function(func: NativeCall, name: String, module_path: Vec<String>, framework: String) -> Rc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            flags: FunctionFlag::STATIC,
            framework,
            module_path,
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0
        };
        Rc::new(reference)
    }

    pub fn opcode_function(name: String, arguments: Vec<String>, module_path: Vec<String>, framework: String, storage_index: usize, defined_storage_index: usize) -> Rc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Opcode,
            flags: FunctionFlag::STATIC,
            framework,
            module_path,
            name,
            arguments,
            storage_index,
            defined_storage_index,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new())
        };
        Rc::new(reference)
    }

    unsafe fn native_function_call(reference: &FunctionReference, func: NativeCall, compiler: &mut BramaCompiler, source: Option<VmObject>) -> Result<(), String> {            
        let total_args                 = *compiler.opcodes_ptr.offset(1);
        let call_return_assign_to_temp = *compiler.opcodes_ptr.offset(2) != 0;
        let parameter = match reference.flags {
            FunctionFlag::IN_CLASS => FunctionParameter::new(&(*compiler.current_scope).stack, source, get_memory_index!(compiler) as usize, total_args, &compiler.stdout, &compiler.stderr),
            _ => FunctionParameter::new(&(*compiler.current_scope).stack, source, get_memory_index!(compiler) as usize, total_args, &compiler.stdout, &compiler.stderr)
        };
        
        match func(parameter) {
            Ok(result) => {
                dec_memory_index!(compiler, total_args as usize);

                if call_return_assign_to_temp {
                    *(*compiler.current_scope).stack_ptr = result;
                    inc_memory_index!(compiler, 1);
                }

                compiler.opcodes_ptr = compiler.opcodes_ptr.offset(2);
                Ok(())
            },
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }

    fn opcode_function_call(reference: &FunctionReference, options: &mut BramaCompiler) -> Result<(), String> {
        unsafe {
            let argument_size              = *options.opcodes_ptr.offset(1);
            let call_return_assign_to_temp = *options.opcodes_ptr.offset(2) != 0;
            let old_index                  = options.opcodes_ptr.offset(2);
            options.opcodes_ptr            = options.opcodes.as_mut_ptr().offset(reference.opcode_location.get() as isize);
            options.scope_index           += 1;

            if argument_size != *options.opcodes_ptr {
                return Err("Function argument error".to_string());
            }

            let memory_index = get_memory_index!(options) as usize;
            let arguments = &(*options.current_scope).stack[memory_index - argument_size as usize..memory_index];
            dec_memory_index!(options, argument_size.into());

            if options.scopes.len() <= options.scope_index {
                options.scopes.resize(options.scopes.len() * 2, Scope::empty());
            }

            let mut scope = &mut options.scopes[options.scope_index];
            let storage = &mut options.storages[reference.storage_index];
            
            /*
            TODO: fast but has bug
            if scope.storage_index == -1 {
                scope.memory = storage.get_memory();
                scope.stack.resize(storage.get_temp_size() as usize, EMPTY_OBJECT);
                scope.storage_index = reference.storage_index as isize;
            }*/

            scope.memory = storage.get_memory();
            scope.stack.resize(storage.get_temp_size() as usize, EMPTY_OBJECT);
            scope.storage_index = reference.storage_index as isize;

            scope.stack_ptr = scope.stack.as_mut_ptr();
            scope.memory_ptr = scope.memory.as_mut_ptr();

            scope.location                   = old_index;
            scope.const_size                 = storage.get_constant_size();
            scope.call_return_assign_to_temp = call_return_assign_to_temp;

            options.current_scope = scope;
            

            if argument_size > 0 {
                for index in 0..argument_size {
                    *scope.stack_ptr = arguments[argument_size as usize-index as usize - 1];
                    inc_memory_index!(options, 1);
                }
            }
        }
        Ok(())
    }
}
