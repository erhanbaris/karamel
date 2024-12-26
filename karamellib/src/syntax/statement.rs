use crate::syntax::assignment::AssignmentParser;
use crate::syntax::function_return::FunctionReturnParser;
use crate::syntax::if_condition::IfConditiontParser;
use crate::syntax::load_module::LoadModuleParser;
use crate::syntax::loop_item::LoopItemParser;
use crate::syntax::loops::WhileLoopParser;
use crate::syntax::util::map_parser;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::types::*;

pub struct StatementParser;

impl SyntaxParserTrait for StatementParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        map_parser(parser, &[LoadModuleParser::parse, LoopItemParser::parse, WhileLoopParser::parse, FunctionReturnParser::parse, AssignmentParser::parse, IfConditiontParser::parse])
    }
}
