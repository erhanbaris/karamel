use std::{iter::Skip, sync::Arc, vec::Vec};
use std::cell::RefCell;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Take;
use bitflags::bitflags;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{inc_memory_index, dec_memory_index, get_memory_index};
use crate::types::*;
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
    pub fn execute<'a>(&self, compiler: &'a mut BramaCompiler, base: Option<VmObject>) -> Result<(), String>{
        unsafe {
            match self.callback {
                FunctionType::Native(func) => FunctionReference::native_function_call(&self, func, compiler, base),
                FunctionType::Opcode => FunctionReference::opcode_function_call(&self,  compiler, base)
            }
        }
    }

    pub fn buildin_function(func: NativeCall, name: String, flags: FunctionFlag) -> Arc<FunctionReference> {
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
        Arc::new(reference)
    }

    pub fn native_function(func: NativeCall, name: String, module_path: Vec<String>, framework: String) -> Arc<FunctionReference> {
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
        Arc::new(reference)
    }

    pub fn opcode_function(name: String, arguments: Vec<String>, module_path: Vec<String>, framework: String, storage_index: usize, defined_storage_index: usize) -> Arc<FunctionReference> {
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
        Arc::new(reference)
    }

    unsafe fn native_function_call<'a>(reference: &FunctionReference, func: NativeCall, compiler: &'a mut BramaCompiler, source: Option<VmObject>) -> Result<(), String> {            
        
        let mut scope = compiler.current_scope.borrow_mut();
        let total_args = compiler.opcodes[compiler.opcode_index + 1];
        let call_return_assign_to_temp = compiler.opcodes[compiler.opcode_index + 2] != 0;
        let stack = &scope.stack;
        let parameter = match reference.flags {
            FunctionFlag::IN_CLASS => FunctionParameter::new(stack, source, scope.memory_index.load(Ordering::Relaxed), total_args, &compiler.stdout, &compiler.stderr),
            _ => FunctionParameter::new(stack, source, scope.memory_index.load(Ordering::Relaxed), total_args, &compiler.stdout, &compiler.stderr)
        };
        
        let func_call_result = func(parameter);
        match func_call_result {
            Ok(result) => {
                scope.memory_index.fetch_sub(total_args as usize, Ordering::Relaxed);

                if call_return_assign_to_temp {
                    scope.stack[get_memory_index!(compiler)] = result;
                    scope.memory_index.fetch_add(total_args as usize, Ordering::Relaxed);
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

    fn opcode_function_call<'a>(reference: &FunctionReference, options: &'a mut BramaCompiler, source: Option<VmObject>) -> Result<(), String> {

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
                dec_memory_index!(options, 1);
                args.push(options.current_scope.borrow().stack[get_memory_index!(options)]);
            }
        }

        if options.scopes.len() <= options.scope_index {
            let empty_scope = Arc::new(RefCell::new(Scope::empty()));
            options.scopes.resize(options.scopes.len() * 2, empty_scope.clone());
        }

        let memory = options.storages[reference.storage_index].get_memory().borrow().clone();
        let stack = options.storages[reference.storage_index].get_stack().borrow().clone();

        let new_scope = options.heap.add_scope(Arc::new(RefCell::new(Scope {
            memory: memory,
            stack: stack,
            location: old_index,
            memory_index: AtomicUsize::new(get_memory_index!(options)),
            const_size: options.storages[reference.storage_index].get_constant_size(),
            call_return_assign_to_temp
        })));

        options.scopes[options.scope_index] = new_scope.clone();

        options.current_scope = options.scopes[options.scope_index].clone();
        options.current_scope.borrow().memory_index.store(0, Ordering::Relaxed);

        if argument_size > 0 {
            for _ in 0..argument_size {
                options.current_scope.borrow_mut().stack[get_memory_index!(options)] = args[get_memory_index!(options)];
                inc_memory_index!(options, 1);
            }
        }
        Ok(())
    }
}
