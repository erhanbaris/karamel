use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::value::BramaPrimative;
use crate::types::BramaOperatorType;

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BramaIfStatementElseItem {
    pub condition: Box<BramaAstType>,
    pub body: Box<BramaAstType>
}

impl BramaIfStatementElseItem {
    pub fn new(condition: Box<BramaAstType>, body: Box<BramaAstType>) -> BramaIfStatementElseItem {
        BramaIfStatementElseItem {
            condition,
            body,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaAstType {
    None,
    NewLine,
    Block(Vec<BramaAstType>),
    FuncCall {
        names: Vec<String>,
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
    /*Loop,*/
    IfStatement {
        condition: Box<BramaAstType>,
        body: Box<BramaAstType>,
        else_body: Option<Box<BramaAstType>>,
        else_if: Vec<Box<BramaIfStatementElseItem>>
    },
    Symbol(String),
    Indexer { body: Box<BramaAstType>, indexer: Box<BramaAstType> }
}
