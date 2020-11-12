use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::primative::PrimativeParser;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    type Item = BramaAstType;
    type In = SyntaxParser;

    fn parse(parser: &Self::In) -> AstResult {
        return Self::parse_prefix_unary(parser);
    }
}

impl UnaryParser {
    fn parse_prefix_unary(parser: &SyntaxParser) -> AstResult {
        if let Some(operator) = parser.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction,
            BramaOperatorType::Increment,
            BramaOperatorType::Deccrement,
            BramaOperatorType::Not,
            BramaOperatorType::BitwiseNot]) {

            let mut unary_ast = BramaAstType::None;
            let token         = &parser.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                BramaOperatorType::Addition | BramaOperatorType::Subtraction => {
                    if token.token_type.is_integer() || token.token_type.is_double() {
                        match PrimativeParser::parse(parser) {
                            Ok(BramaAstType::None) => (),
                            Ok(ast) => unary_ast=ast,
                            Err(err) => return Err(err)
                        }
                    }
                },

                /* ! */
                BramaOperatorType::Not => {
                    if token.token_type.is_integer() || token.token_type.is_double() || token.token_type.is_bool() {
                        if let Ok(ast) = PrimativeParser::parse(parser) {
                            unary_ast = ast;
                        }
                    }
                },

                /* ++variable, --variable*/
                BramaOperatorType::Increment | BramaOperatorType::Deccrement => {
                    if token.token_type.is_symbol() {
                        unary_ast = BramaAstType::Symbol(token.token_type.get_symbol().to_string());
                    }
                },
                _ => return Err(("Invalid unary operation", 0, 0))
            }


            return match unary_ast {
                BramaAstType::None => Err(("Invalid unary operation", 0, 0)),
                _ => Ok(BramaAstType::PrefixUnary(operator, Box::new(unary_ast)))
            };
        }

        return Ok(BramaAstType::None);
    }
}