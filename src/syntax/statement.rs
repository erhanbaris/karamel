use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::syntax::util::map_parser;
use crate::syntax::if_condition::IfConditiontParser;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::function_return::FunctionReturnParser;

pub struct StatementParser;

impl SyntaxParserTrait for StatementParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        return map_parser(parser, &[FunctionReturnParser::parse, AssignmentParser::parse, IfConditiontParser::parse]);
    }
}
