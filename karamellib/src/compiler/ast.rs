use std::vec::Vec;
use std::sync::Arc;

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

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BramaDictItem {
    pub key: Arc<BramaPrimative>,
    pub value: Arc<BramaAstType>
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
        func_name_expression: Box<BramaAstType>,
        arguments: Vec<Box<BramaAstType>>,
        assign_to_temp: bool
    },
    AccessorFuncCall {
        source: Box<BramaAstType>,
        indexer: Box<BramaAstType>,
        assign_to_temp: bool
    },
    Primative(Arc<BramaPrimative>),
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
        variable: Box<BramaAstType>,
        operator: BramaOperatorType,
        expression: Box<BramaAstType>
    },
    IfStatement {
        condition: Box<BramaAstType>,
        body: Box<BramaAstType>,
        else_body: Option<Box<BramaAstType>>,
        else_if: Vec<Box<BramaIfStatementElseItem>>
    },
    FunctionDefination {
        name: String,
        arguments: Vec<String>,
        body: Arc<BramaAstType>
    },
    Symbol(String),
    FunctionMap(Vec<String>),
    Load(Vec<String>),
    List(Vec<Box<BramaAstType>>),
    Dict(Vec<Box<BramaDictItem>>),
    Indexer { body: Box<BramaAstType>, indexer: Box<BramaAstType> },
    Return(Box<BramaAstType>),
    Break,
    Continue,
    EndlessLoop(Box<BramaAstType>),
    WhileLoop {
        control: Box<BramaAstType>,
        body: Box<BramaAstType>
    }
}
