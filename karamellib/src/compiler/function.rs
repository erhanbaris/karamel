use std::{iter::Skip, sync::Arc, vec::Vec};
use std::cell::RefCell;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Take;

use crate::{inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::*;
use crate::compiler::{BramaCompiler, Scope};

pub type NativeCallResult = Result<VmObject, String>;
pub type NativeCall       = fn(FunctionParameter) -> NativeCallResult;

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

#[derive(Clone)]
#[derive(Default)]
pub struct FunctionReference {
    pub callback: FunctionType,
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
    pub fn execute(&self, compiler: &mut BramaCompiler, source: Option<VmObject>) -> Result<(), String>{
        unsafe {
            match self.callback {
                FunctionType::Native(func) => FunctionReference::native_function_call(&self, func, source, compiler),
                FunctionType::Opcode => FunctionReference::opcode_function_call(&self, source,  compiler)
            }
        }
    }

    pub fn buildin_function(func: NativeCall, name: String) -> Arc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            framework: "".to_string(),
            module_path: Vec::new(),
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0
        };
        Arc::new(reference)
    }

    pub fn native_function(func: NativeCall, name: String, module_path: Vec<String>, framework: String) -> Arc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Native(func),
            framework,
            module_path,
            name,
            arguments: Vec::new(),
            storage_index: 0,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new()),
            defined_storage_index: 0
        };
        Arc::new(reference)
    }

    pub fn opcode_function(name: String, arguments: Vec<String>, module_path: Vec<String>, framework: String, storage_index: usize, defined_storage_index: usize) -> Arc<FunctionReference> {
        let reference = FunctionReference {
            callback: FunctionType::Opcode,
            framework,
            module_path,
            name,
            arguments,
            storage_index,
            defined_storage_index,
            opcode_location: Cell::new(0),
            used_locations: RefCell::new(Vec::new())
        };
        Arc::new(reference)
    }

    unsafe fn native_function_call(_: &FunctionReference, func: NativeCall, source: Option<VmObject>, compiler: &mut BramaCompiler) -> Result<(), String> {            
        let total_args = compiler.opcodes[compiler.opcode_index + 1];
        let call_return_assign_to_temp = compiler.opcodes[compiler.opcode_index + 2] != 0;

        let parameter = FunctionParameter::new(&(*compiler.current_scope).stack, source, get_memory_index!(compiler), total_args, &compiler.stdout, &compiler.stderr);
        
        match func(parameter) {
            Ok(result) => {
                dec_memory_index!(compiler, total_args as usize);

                if call_return_assign_to_temp {
                    (*compiler.current_scope).stack[get_memory_index!(compiler)] = result;
                    inc_memory_index!(compiler, 1);
                }

                compiler.opcode_index += 2;
                Ok(())
            },
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }

    fn opcode_function_call(reference: &FunctionReference, _: Option<VmObject>, options: &mut BramaCompiler) -> Result<(), String> {
        let argument_size  = options.opcodes[options.opcode_index + 1];
        let call_return_assign_to_temp = options.opcodes[options.opcode_index + 2] != 0;
        let old_index = options.opcode_index + 2;
        options.opcode_index = reference.opcode_location.get() as usize;
        options.scope_index += 1;

        if argument_size != options.opcodes[options.opcode_index] {
            return Err("Function argument error".to_string());
        }
        let mut args: Vec<VmObject> = Vec::with_capacity(argument_size as usize);

        if argument_size > 0 {
            for _ in 0..argument_size {
                unsafe {
                    dec_memory_index!(options, 1);
                    args.push((*options.current_scope).stack[get_memory_index!(options)]);
                }
            }
        }

        if options.scopes.len() <= options.scope_index {
            options.scopes.resize(options.scopes.len() * 2, Scope::empty());
        }

        unsafe {
            options.scopes[options.scope_index] = Scope {
                memory: options.storages[reference.storage_index].get_memory().borrow().to_vec(),
                stack: options.storages[reference.storage_index].get_stack().borrow().to_vec(),
                location: old_index,
                memory_index: get_memory_index!(options),
                const_size: options.storages[reference.storage_index].get_constant_size(),
                call_return_assign_to_temp
            };

            options.current_scope = &mut options.scopes[options.scope_index] as *mut Scope;
            (*options.current_scope).memory_index = 0;
        }

        if argument_size > 0 {
            for _ in 0..argument_size {
                unsafe {
                    (*options.current_scope).stack[get_memory_index!(options)] = args[get_memory_index!(options)];
                    inc_memory_index!(options, 1);
                }
            }
        }
        Ok(())
    }
}
