use std::cell::Cell;
use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::value::KaramelPrimative;
use crate::types::KaramelOperatorType;

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct KaramelIfStatementElseItem {
    pub condition: Box<KaramelAstType>,
    pub body: Box<KaramelAstType>
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct KaramelDictItem {
    pub key: Rc<KaramelPrimative>,
    pub value: Rc<KaramelAstType>
}

impl KaramelIfStatementElseItem {
    pub fn new(condition: Box<KaramelAstType>, body: Box<KaramelAstType>) -> KaramelIfStatementElseItem {
        KaramelIfStatementElseItem {
            condition,
            body,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KaramelAstType {
    None,
    NewLine,
    Block(Vec<Rc<KaramelAstType>>),
    FuncCall {
        func_name_expression: Box<KaramelAstType>,
        arguments: Vec<Box<KaramelAstType>>,
        assign_to_temp: Cell<bool>
    },
    AccessorFuncCall {
        source: Box<KaramelAstType>,
        indexer: Box<KaramelAstType>,
        assign_to_temp: Cell<bool>
    },
    Primative(Rc<KaramelPrimative>),
    Binary {
        left: Box<KaramelAstType>, 
        operator: KaramelOperatorType, 
        right: Box<KaramelAstType>
    },
    Control {
        left: Box<KaramelAstType>, 
        operator: KaramelOperatorType, 
        right: Box<KaramelAstType>
    },
    /*Control,*/
    PrefixUnary(KaramelOperatorType, Box<KaramelAstType>),
    SuffixUnary(KaramelOperatorType, Box<KaramelAstType>),
    Assignment {
        variable: Box<KaramelAstType>,
        operator: KaramelOperatorType,
        expression: Box<KaramelAstType>
    },
    IfStatement {
        condition: Box<KaramelAstType>,
        body: Box<KaramelAstType>,
        else_body: Option<Box<KaramelAstType>>,
        else_if: Vec<Box<KaramelIfStatementElseItem>>
    },
    FunctionDefination {
        name: String,
        arguments: Vec<String>,
        body: Rc<KaramelAstType>
    },
    Symbol(String),
    ModulePath(Vec<String>),
    Load(Vec<String>),
    List(Vec<Box<KaramelAstType>>),
    Dict(Vec<Box<KaramelDictItem>>),
    Indexer { body: Box<KaramelAstType>, indexer: Box<KaramelAstType> },
    Return(Box<KaramelAstType>),
    Break,
    Continue,
    EndlessLoop(Box<KaramelAstType>),
    WhileLoop {
        control: Box<KaramelAstType>,
        body: Box<KaramelAstType>
    }
}
