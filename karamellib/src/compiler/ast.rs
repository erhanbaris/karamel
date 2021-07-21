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
pub struct KaramelIfStatementElseItem<'a> {
    pub condition: Rc<KaramelAstType<'a>>,
    pub body: Rc<KaramelAstType<'a>>
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct KaramelDictItem<'a> {
    pub key: Rc<KaramelPrimative<'a>>,
    pub value: Rc<KaramelAstType<'a>>
}

impl<'a> KaramelIfStatementElseItem<'a> {
    pub fn new(condition: Rc<KaramelAstType<'a>>, body: Rc<KaramelAstType<'a>>) -> KaramelIfStatementElseItem<'a> {
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
pub enum KaramelAstType<'a> {
    None,
    NewLine,
    Block(Vec<Rc<KaramelAstType<'a>>>),
    FuncCall {
        func_name_expression: Rc<KaramelAstType<'a>>,
        arguments: Vec<Rc<KaramelAstType<'a>>>,
        assign_to_temp: Cell<bool>
    },
    AccessorFuncCall {
        source: Rc<KaramelAstType<'a>>,
        indexer: Rc<KaramelAstType<'a>>,
        assign_to_temp: Cell<bool>
    },
    Primative(Rc<KaramelPrimative<'a>>),
    Binary {
        left: Rc<KaramelAstType<'a>>, 
        operator: KaramelOperatorType, 
        right: Rc<KaramelAstType<'a>>
    },
    Control {
        left: Rc<KaramelAstType<'a>>, 
        operator: KaramelOperatorType, 
        right: Rc<KaramelAstType<'a>>
    },
    /*Control,*/
    PrefixUnary { 
        operator: KaramelOperatorType, 
        expression: Rc<KaramelAstType<'a>>, 
        assign_to_temp: Cell<bool>
    },
    SuffixUnary(KaramelOperatorType, Rc<KaramelAstType<'a>>),
    Assignment {
        variable: Rc<KaramelAstType<'a>>,
        operator: KaramelOperatorType,
        expression: Rc<KaramelAstType<'a>>
    },
    IfStatement {
        condition: Rc<KaramelAstType<'a>>,
        body: Rc<KaramelAstType<'a>>,
        else_body: Option<Rc<KaramelAstType<'a>>>,
        else_if: Vec<Rc<KaramelIfStatementElseItem<'a>>>
    },
    FunctionDefination {
        name: String,
        arguments: Vec<String>,
        body: Rc<KaramelAstType<'a>>
    },
    Symbol(String),
    ModulePath(Vec<String>),
    Load(Vec<String>),
    List(Vec<Rc<KaramelAstType<'a>>>),
    Dict(Vec<Rc<KaramelDictItem<'a>>>),
    Indexer { body: Rc<KaramelAstType<'a>>, indexer: Rc<KaramelAstType<'a>> },
    Return(Rc<KaramelAstType<'a>>),
    Break,
    Continue,
    Loop {
        loop_type: LoopType<'a>,
        body: Rc<KaramelAstType<'a>>
    }
}
