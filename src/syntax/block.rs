use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::newline::NewlineParser;
use crate::syntax::util::map_parser;
use crate::compiler::ast::BramaAstType;
use crate::syntax::if_condition::IfConditiontParser;

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
        let mut block_asts: Vec<BramaAstType> = Vec::new();

        loop {
            let ast = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse, IfConditiontParser::parse, NewlineParser::parse])?;
            match ast {
                BramaAstType::None =>  break,
                BramaAstType::NewLine =>  (),
                _ => block_asts.push(ast)
            };

            if !multiline { break; }

            if let Ok(_) = parser.peek_token() {
                parser.indentation_check()?;
            }
            else {
                break;
            }
        }

        return match block_asts.len() {
            0 => Ok(BramaAstType::None),
            1 => Ok(block_asts[0].clone()),
            _ => Ok(BramaAstType::Block(block_asts.to_vec()))
        }
    }
}
