use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::expression::ExpressionParser;
use crate::syntax::newline::NewlineParser;
use crate::syntax::util::map_parser;
use crate::compiler::ast::KaramelAstType;
use crate::syntax::statement::StatementParser;
use crate::syntax::function_defination::FunctionDefinationParser;

struct BlockParser;
pub struct SingleLineBlockParser;
pub struct MultiLineBlockParser;

impl SyntaxParserTrait for SingleLineBlockParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        BlockParser::parse(parser, false)
    }
}


impl SyntaxParserTrait for MultiLineBlockParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        BlockParser::parse(parser, true)
    }
}


impl BlockParser {
    fn parse(parser: &SyntaxParser, multiline: bool) -> AstResult {
        let mut block_asts: Vec<Rc<KaramelAstType>> = Vec::new();
        let current_indentation = parser.get_indentation();

        loop {
            parser.indentation_check()?;
            let ast = map_parser(parser, &[FunctionDefinationParser::parse, StatementParser::parse, ExpressionParser::parse, NewlineParser::parse])?;
    
            match ast {
                KaramelAstType::None =>  break,
                KaramelAstType::NewLine =>  (),
                _ => block_asts.push(Rc::new(ast))
            };

            if !multiline { break; }
            
            parser.cleanup();
            if !parser.is_same_indentation(current_indentation) {
                break;
            }
        }

        return match block_asts.len() {
            0 => Ok(KaramelAstType::None),
            1 => Ok((&*block_asts[0]).clone()),
            _ => Ok(KaramelAstType::Block(block_asts.to_vec()))
        }
    }
}
