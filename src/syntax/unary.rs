use crate::types::*;
use crate::syntax::SyntaxParser;

pub struct UnaryParser;

impl SyntaxParserTrait for UnaryParser {
    type Item = BramaAstType;
    type In = SyntaxParser;

    fn parse(parser: &Self::In) -> AstResult {
        if let Some(operator) = self.match_operator(&[BramaOperatorType::Addition,
            BramaOperatorType::Subtraction,
            BramaOperatorType::Deccrement,
            BramaOperatorType::Not,
            BramaOperatorType::BitwiseNot]) {

            let mut unary_ast = BramaAstType::None;
            let token         = &self.peek_token().unwrap();

            match operator {
                /* +1024 -1024 */
                BramaOperatorType::Addition | BramaOperatorType::Subtraction => {
                    if token.token_type.is_integer() || token.token_type.is_double() {
                        match PrimativeParser::parse(self) {
                            Ok(BramaAstType::None) => (),
                            Ok(ast) => unary_ast=ast,
                            Err(err) => return err
                        }

                        if let Ok(ast) = PrimativeParser::parse(self) {
                            unary_ast = ast;
                        }
                    }
                },

                /* ! */
                BramaOperatorType::Not => {
                    if token.token_type.is_operator() || token.token_type.is_double() {
                        if let Ok(ast) = PrimativeParser::parse(self) {
                            unary_ast = ast;
                        }
                    }
                },
                _ => ()
            }

            return Ok(BramaAstType::PrefixUnary(Box::new(unary_ast)));
        }

        return Err(("Invalid unary operation", 0, 0))
    }
}