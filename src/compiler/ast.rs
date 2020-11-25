use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::value::BramaPrimative;
use crate::types::BramaOperatorType;

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    Block(Vec<BramaAstType>),
    FunCall {
        name: String,
        arguments: Vec<Box<BramaAstType>>
    },
    Primative(Rc<BramaPrimative>),
    Binary {
        left: Box<BramaAstType>, 
        operator: BramaOperatorType, 
        right: Box<BramaAstType>
    },
    Control {
        left: Box<BramaAstType>, 
        operator: BramaOperatorType, 
        right: Box<BramaAstType>
    },
    /*Control,*/
    PrefixUnary(BramaOperatorType, Box<BramaAstType>),
    SuffixUnary(BramaOperatorType, Box<BramaAstType>),
    Assignment {
        variable: Rc<String>,
        operator: BramaOperatorType,
        expression: Box<BramaAstType>
    },
    /*Loop,
    IfStatement,*/
    Symbol(String)
}
