use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::compiler::ast::BramaAstType;

pub struct LoadModuleParser;

impl SyntaxParserTrait for LoadModuleParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        let token = parser.peek_token();
        if token.is_err() {
            return Ok(BramaAstType::None);
        }

        if let BramaTokenType::Symbol(symbol) = &token.unwrap().token_type {
            let mut symbol_definitions: Vec<String> = Vec::new();
            symbol_definitions.push(symbol.to_string());

            parser.consume_token();
            parser.cleanup_whitespaces();
            
            loop {
                if let Some(_) = parser.match_operator(&[BramaOperatorType::Dot]) {
                    if let BramaTokenType::Symbol(inner_symbol) = &parser.peek_token().unwrap().token_type {
                        parser.consume_token();
                        symbol_definitions.push(inner_symbol.to_string());
                        continue;
                    }
                    else {
                        parser.set_index(index_backup);
                        return Ok(BramaAstType::None);
                    }
                }
                else {
                    break;
                }
            }
            
            parser.cleanup_whitespaces();

            if parser.match_keyword(BramaKeywordType::Load) {
                if symbol_definitions.len() > 0 {
                    return Ok(BramaAstType::Load(symbol_definitions.to_vec()));
                }
            }
        }

        parser.set_index(index_backup);
        return Ok(BramaAstType::None);
    }
}
