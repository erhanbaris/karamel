use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::control::ExpressionParser;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::newline::NewlineParser;
use crate::syntax::util::map_parser;
use crate::compiler::ast::BramaAstType;

pub struct BlockParser;

impl SyntaxParserTrait for BlockParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut block_asts: Vec<BramaAstType> = Vec::new();

        loop {
            let ast = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse, NewlineParser::parse])?;
            match ast {
                BramaAstType::None =>  break,
                BramaAstType::NewLine =>  (),
                _ => block_asts.push(ast)
            };

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