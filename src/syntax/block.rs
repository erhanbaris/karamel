use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::control::ExpressionParser;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::util::map_parser;

pub struct BlockParser;

impl SyntaxParserTrait for BlockParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse]);
    }
}