use std::rc::Rc;

use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;
use crate::syntax::expression::ExpressionParser;
use crate::syntax::{SyntaxFlag, SyntaxParser, SyntaxParserTrait};
use crate::types::*;

pub struct FunctionReturnParser;

impl SyntaxParserTrait for FunctionReturnParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();

        if parser.match_keyword(KaramelKeywordType::Return) {
            if !parser.flags.get().contains(SyntaxFlag::FUNCTION_DEFINATION) {
                parser.set_index(index_backup);
                return Err(KaramelErrorType::ReturnMustBeUsedInFunction);
            }

            parser.cleanup_whitespaces();

            let parser_flags = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::IN_RETURN);

            let ast = ExpressionParser::parse(parser)?;
            let return_ast = KaramelAstType::Return(Rc::new(ast));
            parser.flags.set(parser_flags);

            return Ok(return_ast);
        }

        parser.set_index(index_backup);
        Ok(KaramelAstType::None)
    }
}
