use std::vec::Vec;
use std::rc::Rc;
use std::ptr;

use ast::BramaDictItem;

use crate::types::*;
use crate::compiler::*;
use crate::buildin::*;
use crate::compiler::value::BramaPrimative;
use crate::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
use crate::compiler::storage_builder::StorageBuilder;

#[derive(Clone)]
pub struct Scope {
    pub memory: Vec<VmObject>, 
    pub stack: Vec<VmObject>, 
    pub location: usize,
    pub memory_index: usize,
    pub call_return_assign_to_temp: bool,
    pub const_size: u8
}

impl Scope {
    pub fn empty() -> Scope {
        Scope { 
            const_size: 0, 
            call_return_assign_to_temp: false, 
            memory_index: 0, 
            location: 0, 
            memory: Vec::new(), 
            stack: Vec::new()
        }
    }
}

pub struct BramaCompiler {
    pub opcodes : Vec<u8>,
    pub storages: Vec<StaticStorage>,
    pub modules: ModuleCollection,
    pub opcode_index: usize,
    pub functions_compiled: bool,
    pub loop_breaks: Vec<usize>,
    pub loop_continues: Vec<usize>,
    pub scopes: Vec<Scope>,
    pub current_scope: *mut Scope,
    pub scope_index: usize
}

pub struct FunctionDefine {
    name: String,
    arguments: Vec<String>,
    body: Rc<BramaAstType>,
    storage_index: usize
}

impl  BramaCompiler {
    pub fn new() -> BramaCompiler {
        let mut compiler = BramaCompiler {
            opcodes: Vec::new(),
            storages: vec![StaticStorage::new()],
            modules: ModuleCollection::new(),
            opcode_index: 0,
            functions_compiled: false,
            loop_breaks: Vec::new(),
            loop_continues: Vec::new(),
            scopes: Vec::new(),
            current_scope: ptr::null_mut(),
            scope_index: 0
        };

        for _ in 0..32{
            compiler.scopes.push(Scope::empty());
        }
        
        compiler.current_scope = &mut compiler.scopes[0] as *mut Scope;
        compiler
    }

    pub fn reset(&mut self) {
        self.opcodes = Vec::new();
    }
}

pub trait Compiler
{
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompiler) -> CompilerResult;
}


pub struct InterpreterCompiler;
impl Compiler for InterpreterCompiler {   
    fn compile(&self, ast: &BramaAstType, options: &mut BramaCompiler) -> CompilerResult {
        let storage_builder: StorageBuilder = StorageBuilder::new();
        storage_builder.prepare_variable_store(ast, options);
        let mut functions_compiled = false;

        /* Jump over all function definations to main function */
        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(0 as u8);
        options.opcodes.push(0 as u8);
        
        /* First part of the codes are functions */

        let mut functions = Vec::new();
        self.find_function_definations(ast, &mut functions, options, 0);
        self.generate_functions(&functions, options)?;

        /* Update all opcode references */
        /* Loop all storages */
        for storage in options.storages.iter_mut() {
            
            /* Update function referred location. Look one by one each constants */
            for (constant_index, function_reference) in storage.get_function_constants() {
                if let BramaPrimative::Function(reference) = &*function_reference.deref() {
                    
                    match storage.get_function(&reference.name) {
                        Some(func_information) => {
                            /* Update opcode function location at opcode array */
                            let mut new_reference = reference.clone();
                            new_reference.callback = FunctionType::Opcode(func_information.opcode_location.get() as u16);
                            let new_object = BramaPrimative::Function(new_reference);
                            storage.update_constant(constant_index, VmObject::from(Rc::new(new_object)));
                        },
                        _ => ()
                    };
                }
            }
        }

        options.functions_compiled = true;
        
        /* If there are no function defination, remove previous opcodes */
        if options.opcodes.len() == 3 {
            options.opcodes.clear();
        }
        else {
            /* Prepare jump code for main function */
            let current_location = options.opcodes.len();
            options.opcodes[1] = current_location as u8;
            options.opcodes[2] = (current_location >> 8) as u8;
            functions_compiled = true;
        }

        /* Generate main function code */
        self.generate_opcode(ast, &BramaAstType::None, options, 0)?;

        /* Fix function references */
        if functions_compiled {
            for storage in &options.storages {
                for (_, function) in &storage.functions {
                    let first     = function.opcode_location.get() as u8;
                    let second  = (function.opcode_location.get() >> 8) as u8;

                    for location in function.used_locations.borrow().iter() {
                        options.opcodes[*location as usize]        = first;
                        options.opcodes[(*location + 1)  as usize] = second;
                    }
                }
            }
        }
        Ok(0)
    }
}

impl InterpreterCompiler {
    fn find_function_definations(&self, ast: &BramaAstType, functions: &mut Vec<FunctionDefine>, options: &mut BramaCompiler, storage_index: usize) {
        match ast {
            BramaAstType::FunctionDefination { name, arguments, body  } => {

                if let Some(information) = options.storages[storage_index].get_function(name) {
                    functions.push(FunctionDefine {
                        name: name.to_string(),
                        arguments: arguments.to_vec(),
                        body: body.clone(),
                        storage_index: storage_index
                    });
                    self.find_function_definations(body, functions, options, information.storage_index as usize);
                }
            },
            BramaAstType::Block(blocks) => {
                for block in blocks {
                    self.find_function_definations(&block, functions, options, storage_index);
                }
            },
            _ => ()
        };
    }

    fn generate_functions(&self, functions: &Vec<FunctionDefine>, options: &mut BramaCompiler) -> CompilerResult {
        
        for function in functions {
            if let Some(information) = options.storages[function.storage_index].get_function(&function.name) {
                options.opcodes.push(VmOpCode::Func as u8);
                (*information).opcode_location.set(options.opcodes.len() as u16);
                options.opcodes.push(information.storage_index as u8);
                options.opcodes.push((information.storage_index >> 8) as u8);
                options.opcodes.push(function.arguments.len() as u8);

                if function.arguments.len() > 0 {
                    options.opcodes.push(VmOpCode::InitArguments as u8);
                    options.opcodes.push(function.arguments.len() as u8);
                }

                self.generate_opcode(&function.body, &function.body, options, information.storage_index as usize)?;
            } 
        }

        Ok(0)
    }

    fn generate_opcode(&self, ast: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        match ast {
            BramaAstType::Assignment { variable, operator, expression } => self.generate_assignment(variable.clone(), operator, expression, options, storage_index),
            BramaAstType::Symbol(variable) => self.generate_symbol(variable, upper_ast, options, storage_index),
            BramaAstType::Control { left, operator, right } => self.generate_control(left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Binary { left, operator, right } => self.generate_binary(left, operator, right, upper_ast, options, storage_index),
            BramaAstType::Block(asts) => self.generate_block(asts, upper_ast, options, storage_index),
            BramaAstType::Primative(primative) => self.generate_primative(primative.clone(), upper_ast, options, storage_index),
            BramaAstType::List(list) => self.generate_list(list, upper_ast, options, storage_index),
            BramaAstType::Dict(dict) => self.generate_dict(dict, upper_ast, options, storage_index),
            BramaAstType::FuncCall { names, arguments, assign_to_temp } => self.generate_func_call(names, arguments, *assign_to_temp, upper_ast, options, storage_index),
            BramaAstType::PrefixUnary (operator, expression) => self.generate_prefix_unary(operator, expression, upper_ast, options, storage_index),
            BramaAstType::SuffixUnary (operator, expression) => self.generate_suffix_unary(operator, expression, upper_ast, options, storage_index),
            BramaAstType::NewLine => Ok(0),
            BramaAstType::WhileLoop { control, body } => self.generate_whileloop(control, body, upper_ast, options, storage_index),
            BramaAstType::EndlessLoop(expression) => self.generate_endlessloop(expression, upper_ast, options, storage_index),
            BramaAstType::Break => self.generate_break(upper_ast, options, storage_index),
            BramaAstType::Continue => self.generate_continue(upper_ast, options, storage_index),
            BramaAstType::Return(expression) => self.generate_return(expression, upper_ast, options, storage_index),
            BramaAstType::IfStatement {condition, body, else_body, else_if} => self.generate_if_condition(condition, body, else_body, else_if, upper_ast, options, storage_index),
            BramaAstType::Indexer {body, indexer} => self.generate_indexer(body, indexer, upper_ast, options, storage_index),
            BramaAstType::None => self.generate_none(options, storage_index),
            BramaAstType::FunctionDefination{name: _, arguments: _, body: _} => Ok(0)
        }
    }

    fn generate_primative(&self, primative: Rc<BramaPrimative>, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(primative);
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(index)
            },
            _ => Err("Value not found in storage")
        }
    }

    fn generate_none(&self, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let storage = &options.storages[storage_index];

        let result = storage.get_constant_location(Rc::new(BramaPrimative::Empty));
        match result {
            Some(index) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(index as u8);
                Ok(index)
            },
            _ => Err("Value not found in storage")
        }
    }

    fn generate_list(&self, list: &Vec<Box<BramaAstType>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for item in list.iter().rev() {
            self.generate_opcode(item, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitList as u8);
        options.opcodes.push(list.len() as u8);
        Ok(0)
    }

    fn generate_dict(&self, dict: &Vec<Box<BramaDictItem>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for item in dict.iter().rev() {
            self.generate_primative(item.key.clone(), upper_ast, options, storage_index)?;
            self.generate_opcode(&item.value, upper_ast, options, storage_index)?;
        }
        options.opcodes.push(VmOpCode::InitDict as u8);
        options.opcodes.push(dict.len() as u8);
        Ok(0)
    }

    fn generate_func_call(&self, names: &Vec<String>, arguments: &Vec<Box<BramaAstType>>, assign_to_temp: bool,  upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Build arguments */
        for argument in arguments {
            self.generate_opcode(argument, upper_ast, options, storage_index)?;
        }

        let function_search = options.storages[storage_index].get_function_constant(names[names.len()-1].to_string(), names[0..(names.len()-1)].to_vec(), "".to_string());
        match function_search {
            Some(constant_address) => {
                options.opcodes.push(VmOpCode::Call as u8);
                options.opcodes.push(constant_address as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(assign_to_temp as u8);
                Ok(0 as u8)
            },
            None => {
                match options.storages.get_mut(storage_index).unwrap().get_variable_location(&names[0].to_string()) {
                    /* Variable found */
                    Some(location) => {
                        options.opcodes.push(VmOpCode::Load as u8);
                        options.opcodes.push(location as u8);
                    },
                    /* Variable not found, lets check for function */
                    None => return Err("Value not found in storage")
                }

                options.opcodes.push(VmOpCode::CallStack as u8);
                options.opcodes.push(arguments.len() as u8);
                options.opcodes.push(assign_to_temp as u8);
                Ok(0 as u8)
            }
        }        
    }

    fn generate_break(&self, _: &BramaAstType, options: &mut BramaCompiler, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_breaks.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(0)
    }

    fn generate_continue(&self, _: &BramaAstType, options: &mut BramaCompiler, _: usize) -> CompilerResult {       
        options.opcodes.push(VmOpCode::Jump as u8);
        options.loop_continues.push(options.opcodes.len());
        options.opcodes.push(0);
        options.opcodes.push(0);
        Ok(0)
    }

    fn generate_return(&self, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(expression, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Return as u8);
        Ok(0)
    }

    fn generate_whileloop(&self, control: &BramaAstType, body: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(control, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0 as u8);
        options.opcodes.push(0 as u8);

        self.generate_opcode(body, upper_ast, options, storage_index)?;

        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(start_location as u8);
        options.opcodes.push((start_location >> 8) as u8);

        let current_location = options.opcodes.len(); 

        for break_info in &options.loop_breaks {
            options.opcodes[*break_info]     = current_location as u8;
            options.opcodes[*break_info + 1] = (current_location >> 8) as u8;
        } 

        for continue_info in &options.loop_continues {
            options.opcodes[*continue_info]     = start_location as u8;
            options.opcodes[*continue_info + 1] = (start_location >> 8) as u8;
        } 

        options.loop_breaks    = loop_breaks.to_vec();
        options.loop_continues = loop_continues.to_vec();

        let end_location = current_location - compare_location;
        options.opcodes[compare_location]     = end_location as u8;
        options.opcodes[compare_location + 1] = (end_location >> 8) as u8;

        Ok(0)
    }

    fn generate_endlessloop(&self, expression: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /* Backup loop informations */
        let loop_breaks = options.loop_breaks.to_vec();
        let loop_continues = options.loop_continues.to_vec();
        
        let start_location = options.opcodes.len(); 
        self.generate_opcode(expression, upper_ast, options, storage_index)?;

        /* 
        1. ... Endless loop start location
        2. ...
        3. ...
        4. ...
        5. Jump to  1.*/
        options.opcodes.push(VmOpCode::Jump as u8);
        options.opcodes.push(start_location as u8);
        options.opcodes.push((start_location >> 8) as u8);

        let current_location = options.opcodes.len(); 

        for break_info in &options.loop_breaks {
            options.opcodes[*break_info]     = current_location as u8;
            options.opcodes[*break_info + 1] = (current_location >> 8) as u8;
        } 

        for continue_info in &options.loop_continues {
            options.opcodes[*continue_info]     = start_location as u8;
            options.opcodes[*continue_info + 1] = (start_location >> 8) as u8;
        } 

        options.loop_breaks = loop_breaks.to_vec();
        options.loop_continues = loop_continues.to_vec();

        Ok(0)
    }

    fn generate_symbol(&self, variable: &String, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
            /* Variable found */
            Some(location) => {
                options.opcodes.push(VmOpCode::Load as u8);
                options.opcodes.push(location as u8);
                Ok(location)
            },
            /* Variable not found, lets check for function */
            None => {
                let storage = &options.storages[storage_index];                
                let result = storage.get_function_constant(variable.to_string(), Vec::new(), "".to_string());
                match result {
                    Some(index) => {
                        options.opcodes.push(VmOpCode::Load as u8);
                        options.opcodes.push(index as u8);
                        Ok(0)
                    },
                    _ => return Err("Value not found in storage")
                }
            }
        }
    }

    fn generate_control(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, options, storage_index)?;

        let opcode = match operator {
            BramaOperatorType::Or               => VmOpCode::Or as u8,
            BramaOperatorType::And              => VmOpCode::And as u8,
            BramaOperatorType::Equal            => VmOpCode::Equal as u8,
            BramaOperatorType::NotEqual         => VmOpCode::NotEqual as u8,
            BramaOperatorType::GreaterThan      => VmOpCode::GreaterThan as u8,
            BramaOperatorType::LessThan         => VmOpCode::LessThan as u8,
            BramaOperatorType::GreaterEqualThan => VmOpCode::GreaterEqualThan as u8,
            BramaOperatorType::LessEqualThan    => VmOpCode::LessEqualThan as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_assignment(&self, variable: Rc<String>, operator: &BramaOperatorType, expression_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        let location = options.storages.get_mut(storage_index).unwrap().add_variable(&*variable);
        let storage = &options.storages[storage_index];
        
        if let BramaAstType::Primative(primative) = expression_ast {
            if mem::discriminant(&**primative) != mem::discriminant(&BramaPrimative::List([].to_vec())) {
                if *operator == BramaOperatorType::Assign {
                    let result = storage.get_constant_location(primative.clone());
                    let primative_location = match result {
                        Some(index) => index as u8,
                        _ => return Err("Value not found in storage")
                    };

                    options.opcodes.push(VmOpCode::FastStore as u8);
                    options.opcodes.push(location);
                    options.opcodes.push(primative_location);
                    return Ok(0);
                }
            }
        }

        if *operator != BramaOperatorType::Assign {

            /* Load variable data to stack */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);

            self.generate_opcode(expression_ast, &BramaAstType::None, options, storage_index)?;

            let opcode = match operator {
                BramaOperatorType::AssignAddition       => VmOpCode::Addition as u8,
                BramaOperatorType::AssignDivision       => VmOpCode::Division as u8,
                BramaOperatorType::AssignMultiplication => VmOpCode::Multiply as u8,
                BramaOperatorType::AssignSubtraction    => VmOpCode::Subraction as u8,
                _ => BramaOperatorType::None as u8
            };

            options.opcodes.push(opcode);
        } else {
            self.generate_opcode(expression_ast, &BramaAstType::None, options, storage_index)?;
        }
        
        options.opcodes.push(VmOpCode::Store as u8);
        options.opcodes.push(location);
        
        Ok(0)
    }

    fn generate_binary(&self, left_ast: &BramaAstType, operator: &BramaOperatorType, right_ast: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(left_ast, &BramaAstType::None, options, storage_index)?;
        self.generate_opcode(right_ast, &BramaAstType::None, options, storage_index)?;
        let opcode = match operator {
            BramaOperatorType::Addition       => VmOpCode::Addition as u8,
            BramaOperatorType::Subtraction    => VmOpCode::Subraction as u8,
            BramaOperatorType::Multiplication => VmOpCode::Multiply as u8,
            BramaOperatorType::Division       => VmOpCode::Division as u8,
            _ => VmOpCode::None as u8
        };

        options.opcodes.push(opcode);
        Ok(0)
    }

    fn generate_prefix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        
        if *operator == BramaOperatorType::Not { 
            return self.generate_not(expression, options, storage_index);
        }

        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            /* Load data from memory */
            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
        
            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::CopyToStore as u8);
            options.opcodes.push(location);
            return Ok(0);
        }

        Err("Unary expression not valid")
    }

    fn generate_not(&self, expression: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        self.generate_opcode(expression, &BramaAstType::None, options, storage_index)?;
        options.opcodes.push(VmOpCode::Not as u8);
        return Ok(0);
    }

    fn create_exit_jump(&self, options: &mut BramaCompiler, exit_locations: &mut Vec<usize>) {
        options.opcodes.push(VmOpCode::Jump as u8);
        exit_locations.push(options.opcodes.len());
        options.opcodes.push(0 as u8);
        options.opcodes.push(0 as u8);
    }

    fn create_compare(&self, options: &mut BramaCompiler) -> usize {
        options.opcodes.push(VmOpCode::Compare as u8);
        let compare_location = options.opcodes.len();

        options.opcodes.push(0 as u8);
        options.opcodes.push(0 as u8);

        return compare_location;
    }

    fn build_jump_location(&self, options: &mut BramaCompiler, jump_location: usize) {
        let current_location = options.opcodes.len();
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn build_compare_location(&self, options: &mut BramaCompiler, jump_location: usize) {
        let current_location = options.opcodes.len() - jump_location;
        options.opcodes[jump_location]     = current_location as u8;
        options.opcodes[jump_location + 1] = (current_location >> 8) as u8;
    }

    fn generate_if_condition(&self, condition: &BramaAstType, body: &BramaAstType, else_body: &Option<Box<BramaAstType>>, else_if: &Vec<Box<BramaIfStatementElseItem>>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        /*
        ╔════════════════════╗
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   JUMP TO NEXT     ║
        ║   CASE LOCATION    ║
        ╠════════════════════╣
        ║   TRUE BODY        ║
        ╠════════════════════╣
        ║   JUMP TO OUT OF   ║
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   IF CONDITION     ║
        ╠════════════════════╣
        ║   IF CONDITION     ║
        ╚════════════════════╝
        */
        let mut exit_locations: Vec<usize> = Vec::new();
        
        self.generate_opcode(condition, upper_ast, options, storage_index)?;
        let mut if_failed_location = self.create_compare(options);
        self.generate_opcode(body, upper_ast, options, storage_index)?;

        if else_if.len() > 0 || else_body.is_some() {
            /* After executing body, need to exit from 'if condition'.
               Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        for else_if_item in else_if {
            /* Previous conditon should jump to this location */
            self.build_compare_location(options, if_failed_location);

            /* Build condition */
            self.generate_opcode(&else_if_item.condition, upper_ast, options, storage_index)?;

            if_failed_location = self.create_compare(options);

            self.generate_opcode(&else_if_item.body, upper_ast, options, storage_index)?;

            /* Jump to out of if condition */
            self.create_exit_jump(options, &mut exit_locations);
        }

        if let Some(_else_body) = else_body {
            self.build_compare_location(options, if_failed_location);
            self.generate_opcode(_else_body, upper_ast, options, storage_index)?;
        }
        else {
            self.build_compare_location(options, if_failed_location);
        }

        for exit_location in exit_locations {
            self.build_jump_location(options, exit_location);
        }

        return Ok(0);
    }

    fn generate_indexer(&self, body: &BramaAstType, indexer: &BramaAstType, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        self.generate_opcode(body, upper_ast, options, storage_index)?;
        self.generate_opcode(indexer, upper_ast, options, storage_index)?;
        options.opcodes.push(VmOpCode::GetItem as u8);

        return Ok(0);
    }

    fn generate_suffix_unary(&self, operator: &BramaOperatorType, expression: &BramaAstType, _: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult { 
        if let BramaAstType::Symbol(variable) = expression {
            let location = match options.storages.get_mut(storage_index).unwrap().get_variable_location(variable) {
                Some(location) => location,
                _ => return Err("Variable not found in storage")
            };

            options.opcodes.push(VmOpCode::Load as u8);
            options.opcodes.push(location);
            options.opcodes.push(VmOpCode::Dublicate as u8);

            let opcode = match operator {
                BramaOperatorType::Increment  => VmOpCode::Increment as u8,
                BramaOperatorType::Deccrement => VmOpCode::Decrement as u8,
                BramaOperatorType::Not        => VmOpCode::Not as u8,
                _ => return Err("Unary operator not found")
            };
    
            options.opcodes.push(opcode);
            options.opcodes.push(VmOpCode::Store as u8);
            options.opcodes.push(location);
            return Ok(0);
        }

        Err("Unary expression not valid")
    }

    fn generate_block(&self, asts: &Vec<BramaAstType>, upper_ast: &BramaAstType, options: &mut BramaCompiler, storage_index: usize) -> CompilerResult {
        for ast in asts {
            self.generate_opcode(&ast, upper_ast, options, storage_index)?;
        }
        Ok(0)
    }
}