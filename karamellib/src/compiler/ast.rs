use std::cell::Cell;
use std::vec::Vec;
use std::rc::Rc;

use crate::compiler::value::KaramelPrimative;
use crate::syntax::loops::LoopType;
use crate::types::KaramelOperatorType;

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct KaramelIfStatementElseItem {
    pub condition: Rc<KaramelAstType>,
    pub body: Rc<KaramelAstType>
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
    pub fn new(condition: Rc<KaramelAstType>, body: Rc<KaramelAstType>) -> KaramelIfStatementElseItem {
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
        func_name_expression: Rc<KaramelAstType>,
        arguments: Vec<Rc<KaramelAstType>>,
        assign_to_temp: Cell<bool>
    },
    AccessorFuncCall {
        source: Rc<KaramelAstType>,
        indexer: Rc<KaramelAstType>,
        assign_to_temp: Cell<bool>
    },
    Primative(Rc<KaramelPrimative>),
    Binary {
        left: Rc<KaramelAstType>, 
        operator: KaramelOperatorType, 
        right: Rc<KaramelAstType>
    },
    Control {
        left: Rc<KaramelAstType>, 
        operator: KaramelOperatorType, 
        right: Rc<KaramelAstType>
    },
    /*Control,*/
    PrefixUnary { 
        operator: KaramelOperatorType, 
        expression: Rc<KaramelAstType>, 
        assign_to_temp: Cell<bool>
    },
    SuffixUnary(KaramelOperatorType, Rc<KaramelAstType>),
    Assignment {
        variable: Rc<KaramelAstType>,
        operator: KaramelOperatorType,
        expression: Rc<KaramelAstType>
    },
    IfStatement {
        condition: Rc<KaramelAstType>,
        body: Rc<KaramelAstType>,
        else_body: Option<Rc<KaramelAstType>>,
        else_if: Vec<Rc<KaramelIfStatementElseItem>>
    },
    FunctionDefination {
        name: String,
        arguments: Vec<String>,
        body: Rc<KaramelAstType>
    },
    Symbol(String),
    ModulePath(Vec<String>),
    Load(Vec<String>),
    List(Vec<Rc<KaramelAstType>>),
    Dict(Vec<Rc<KaramelDictItem>>),
    Indexer { body: Rc<KaramelAstType>, indexer: Rc<KaramelAstType> },
    Return(Rc<KaramelAstType>),
    Break,
    Continue,
    Loop {
        loop_type: LoopType,
        body: Rc<KaramelAstType>
    }
}
