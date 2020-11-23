use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::control::ExpressionParser;
use crate::syntax::assignment::AssignmentParser;
use crate::syntax::util::map_parser;

pub struct BlockParser;

impl SyntaxParserTrait for BlockParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut block_asts: Vec<BramaAstType> = Vec::new();

        loop {
            let ast = map_parser(parser, &[AssignmentParser::parse, ExpressionParser::parse])?;
            match ast {
                BramaAstType::None => break,
                _                  => block_asts.push(ast)
            };

            if let Ok(token) = parser.peek_token() {
                match token.token_type {
                    BramaTokenType::NewLine(_) => {
                        parser.consume_newline();
                    },
                    _ => break
                }
            }
            else {
                break;
            }
        }

        if block_asts.len() == 0 {
            return Ok(BramaAstType::None);
        }

        return Ok(BramaAstType::Block(block_asts.to_vec()));
    }
}