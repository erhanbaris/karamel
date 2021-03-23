pub mod number;

use crate::compiler::{BramaCompiler, function::{NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::opcode_class::OpcodeClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;


//static variable_name: &'static [OpcodeClass; 1] = &[number::NUMBER_CLASS];
