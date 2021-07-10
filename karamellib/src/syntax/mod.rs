pub mod primative;
pub mod unary;
pub mod util;
pub mod binary;
pub mod control;
pub mod block;
pub mod assignment;
pub mod func_call;
pub mod newline;
pub mod if_condition;
pub mod statement;
pub mod function_defination;
pub mod function_return;
pub mod loops;
pub mod loop_item;
pub mod expression;
pub mod load_module;

use std::borrow::Borrow;
use std::rc::Rc;
use std::vec::Vec;
use std::cell::Cell;

use crate::types::*;
use self::block::MultiLineBlockParser;
use crate::compiler::ast::KaramelAstType;
use crate::error::*;

use bitflags::bitflags;

pub type ParseType = fn(parser: &SyntaxParser) -> AstResult;

pub struct SyntaxParser {
    pub tokens: Vec<Token>,
    pub index: Cell<usize>,
    pub indentation: Cell<usize>,
    pub flags: Cell<SyntaxFlag>
}

bitflags! {
    pub struct SyntaxFlag: u32 {
        const NONE                = 0b00000000;
        const FUNCTION_DEFINATION = 0b00000001;
        const LOOP                = 0b00000010;
        const IN_ASSIGNMENT       = 0b00000100;
        const IN_EXPRESSION       = 0b00001000;
        const IN_FUNCTION_ARG     = 0b00010000;
        const IN_RETURN           = 0b00100000;
        const IN_DICT_INDEXER     = 0b01000000;
    }
}

pub trait SyntaxParserTrait {
    fn parse(parser: &SyntaxParser) -> AstResult;
}

pub trait ExtensionSyntaxParser: Sized {
    fn parsable    (parser: &SyntaxParser) -> bool;
    fn parse_suffix(ast: &mut KaramelAstType, parser: &SyntaxParser) -> AstResult;
}

impl SyntaxParser {
    pub fn new(tokens: Vec<Token>) -> SyntaxParser {
        SyntaxParser {
            tokens,
            index: Cell::new(0),
            indentation: Cell::new(0),
            flags: Cell::new(SyntaxFlag::NONE)
        }
    }

    pub fn parse(&self) -> Result<Rc<KaramelAstType>, KaramelError> {
        return match MultiLineBlockParser::parse(&self) {
            Ok(ast) => {
                self.cleanup();
                
                if let Ok(token) = self.peek_token() {
                    log::debug!("We forget this : {:?}", token);
                    return Err(KaramelError {
                        error_type: KaramelErrorType::SyntaxError,
                        line: token.line,
                        column: token.start
                    });
                }
                Ok(Rc::new(ast))
            },
            Err(error) => {
                if let Ok(token) = self.valid_token() {
                    log::debug!("Syntax parse failed : {:?}", token);
                    return Err(KaramelError {
                        error_type: error,
                        line: token.line,
                        column: token.end
                    });
                }

                return Err(KaramelError {
                    error_type: error,
                    line: 0,
                    column: 0
                });
            }
        };
    }

    pub fn set_indentation(&self, indentation: usize) {
        self.indentation.set(indentation);
    }

    pub fn get_indentation(&self) -> usize {
        self.indentation.get()
    }

    pub fn set_index(&self, index: usize) {
        self.index.set(index);
    }

    pub fn get_index(&self) -> usize {
        self.index.get()
    }

    pub fn is_same_indentation(&self, indentation: usize) -> bool {
        if let Err(_) = self.indentation_check() {
            return false;
        }
        else if indentation != self.get_indentation() {
            return false;
        }
        else if self.peek_token().is_err() {
            return false;
        }
        else if self.peek_token().unwrap().start != indentation as u32 {
            return false;
        }

        return true;
    }

    pub fn peek_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get()) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }

    pub fn valid_token(&self) -> Result<&Token, ()> {
        let mut index = self.index.get() + 1;
        
        while index != 0 {
            match index.checked_sub(1) {
                Some(index) => match self.tokens.get(index) {
                    Some(token) => match token.token_type {
                        KaramelTokenType::NewLine(_) => (),
                        KaramelTokenType::WhiteSpace(_) => (),
                        _ => return Ok(token)
                    },
                    None => ()
                },
                None => return Err(())
            };
            
            index -= 1;
        }
        Err(())
    }

    pub fn next_token(&self) -> Result<&Token, ()> {
        match self.tokens.get(self.index.get() + 1) {
            Some(token) => Ok(token),
            None => Err(())
        }
    }
    
    pub fn consume_token(&self) -> Option<&Token> {
        self.index.set(self.index.get() + 1);
        self.tokens.get(self.index.get())
    }

    pub fn match_keyword(&self, keyword: KaramelKeywordType) -> bool {
        if self.check_keyword(keyword) {
            self.consume_token();
            return true;
        }
        return false;
    }

    pub fn check_keyword<T: Borrow<KaramelKeywordType>>(&self, keyword: T) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match &token.unwrap().token_type {
            KaramelTokenType::Keyword(token_keyword) => {
                if keyword.borrow() == token_keyword {
                    return true;
                }
                return false;
            },
            _ => false
        }
    }

    fn get_newline(&self) -> (bool, usize) {
        let token = self.peek_token();
        if token.is_err() { return (false, 0); }
        return match token.unwrap().token_type {
            KaramelTokenType::NewLine(size) => (true, size as usize),
            _ => (false, 0)
        }
    }

    fn check_operator(&self, operator: &KaramelOperatorType) -> bool {
        let token = self.peek_token();
        if token.is_err() { return false; }
        return match token.unwrap().token_type {
            KaramelTokenType::Operator(token_operator) => *operator == token_operator,
            _ => false
        }
    }

    fn match_operator(&self, operators: &[KaramelOperatorType]) -> Option<KaramelOperatorType> {
        for operator in operators {
            if self.check_operator(operator) {
                self.consume_token();
                return Some(*operator);
            }
        }

        return None;
    }

    pub fn match_keywords(&self, keywords: &[KaramelKeywordType]) -> Option<KaramelKeywordType> {
        for keyword in keywords {
            if self.check_keyword(keyword) {
                self.consume_token();
                return Some(*keyword);
            }
        }

        return None;
    }

    fn cleanup_whitespaces(&self) {
        loop {
            if let Ok(current_token) = self.peek_token() {
                let done = match current_token.token_type {
                    KaramelTokenType::WhiteSpace(_) => false,
                    _ => true
                };

                if done {
                    break;
                }

                self.consume_token();
            }
            else {
                break;
            }
        }
    }

    fn cleanup(&self) {
        if self.peek_token().is_err() {
            return;
        }

        loop {
            if let Ok(current_token) = self.peek_token() {                
                match current_token.token_type {
                    KaramelTokenType::NewLine(_) =>  true,
                    KaramelTokenType::WhiteSpace(_) => true,
                    _ => break
                };

                self.consume_token();
            }
            else {
                break;
            }
        }
    }

    fn indentation_check(&self) -> AstResult {
        if self.next_token().is_err() {
            return Ok(KaramelAstType::None);
        }

        while let Ok(current_token) = self.peek_token() {                
            let success = match current_token.token_type {
                KaramelTokenType::NewLine(size) => {
                    let token_type = &self.next_token().unwrap().token_type;
                    if let KaramelTokenType::NewLine(_) = token_type {
                        /* If next token is newline, no need to check */
                        true
                    }
                    else {
                        /* Next token not a new line but space should be bigger than the current */
                        size == self.indentation.get() as u8 
                    }
                },
                
                KaramelTokenType::WhiteSpace(size) => {
                    size == self.indentation.get() as u8 
                },
                _ => break
            };

            if !success {
                return Err(KaramelErrorType::IndentationIssue);
            }

            self.consume_token();
        }

        Ok(KaramelAstType::None)
    }

    fn in_indication(&self) -> AstResult {
        if self.next_token().is_err() {
            return Ok(KaramelAstType::None);
        }

        while let Ok(current_token) = self.peek_token() {               
            let success = match current_token.token_type {
                KaramelTokenType::NewLine(size) => {
                    let token_type = &self.next_token().unwrap().token_type;
                    if let KaramelTokenType::NewLine(_) = token_type {
                        /* If next token is newline, no need to check */
                        true
                    }
                    else {
                        /* Next token not a new line but space should be bigger than the current */
                        if size > self.indentation.get() as u8 {
                            self.set_indentation(size as usize);
                            true
                        }
                        else {
                            false
                        }
                    }
                },
                _ => break
            };

            if !success {
                return Err(KaramelErrorType::IndentationIssue);
            }

            self.consume_token();
        }

        Ok(KaramelAstType::None)
    }
}