use crate::types::*;
use crate::syntax::util::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::compiler::ast::BramaAstType;
use crate::syntax::control::ExpressionParser;

pub struct FunctionReturnParser;

impl SyntaxParserTrait for FunctionReturnParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.cleanup_whitespaces();

        if parser.match_keyword(BramaKeywordType::Return) {
            if !parser.flags.get().contains(SyntaxFlag::FUNCTION_DEFINATION) {
                return Err(("return must be used in function", 0, 0));
            }

            parser.cleanup_whitespaces();
            
            let ast = ExpressionParser::parse(parser)?;
            let return_ast = BramaAstType::Return(Box::new(ast));
            return Ok(return_ast);
        }

        parser.restore();
        return Ok(BramaAstType::None);
    }
}