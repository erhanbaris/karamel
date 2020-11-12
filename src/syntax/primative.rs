use crate::types::*;
use crate::syntax::SyntaxParser;

pub struct PrimativeParser;

impl PrimativeParser {
    fn parse_basic_primatives(parser: &SyntaxParser) -> AstResult {
        let token = parser.peek_token();
        if token.is_err() {
            return Err(("No more token", 0, 0))
        }

        let result = match &token.unwrap().token_type {
            BramaTokenType::Integer(int)      => Ok(BramaAstType::Primative(BramaPrimative::Integer(*int))),
            BramaTokenType::Double(double)    => Ok(BramaAstType::Primative(BramaPrimative::Double(*double))),
            BramaTokenType::Text(text)        => Ok(BramaAstType::Primative(BramaPrimative::String(text.to_string()))),
            BramaTokenType::Keyword(keyword)  => {
                match keyword {
                    BramaKeywordType::True  => Ok(BramaAstType::Primative(BramaPrimative::Bool(true))),
                    BramaKeywordType::False => Ok(BramaAstType::Primative(BramaPrimative::Bool(false))),
                    _ => Ok(BramaAstType::None)
                }
            },
            BramaTokenType::Operator(BramaOperatorType::ColonMark) => {
                let next_token = parser.next_token();
                if next_token.is_err() {
                    return Ok(BramaAstType::None)
                }

                match &next_token.unwrap().token_type {
                    BramaTokenType::Symbol(symbol) => {
                        parser.consume_token();
                        Ok(BramaAstType::Primative(BramaPrimative::Atom(symbol.to_string())))
                    },
                    _ => Ok(BramaAstType::None)
                }
            },
            _ => Ok(BramaAstType::None)
        };

        match result {
            Ok(BramaAstType::None) => (),
            Ok(_) => {parser.consume_token();},
            _ => () 
        };
        result
    }
}

impl SyntaxParserTrait for PrimativeParser {
    type Item = BramaAstType;
    type In = SyntaxParser;

    fn parse(parser: &Self::In) -> AstResult {
        match Self::parse_basic_primatives(parser) {
            Ok(BramaAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(data) => return Err(data)
        };

        if parser.match_operator(&[BramaOperatorType::SquareBracketStart]).is_some() {
            let mut ast_vec   = Vec::new();
            loop {
                if parser.check_operator(&BramaOperatorType::SquareBracketEnd) {
                    break;
                }

                parser.clear_whitespaces();

                if let Ok(ast) = Self::parse(parser) {
                    ast_vec.push(Box::new(ast));
                }

                parser.clear_whitespaces();
                if parser.match_operator(&[BramaOperatorType::Comma]).is_none()  {
                    break;
                }
            }

            if parser.match_operator(&[BramaOperatorType::SquareBracketEnd]).is_none() {
                return Err(("Array not closed", 0, 0));
            }

            return Ok(BramaAstType::Primative(BramaPrimative::List(ast_vec)));
        }

        Err(("not implemented", 0, 0))
    }
}