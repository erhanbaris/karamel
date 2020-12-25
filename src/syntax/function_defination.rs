use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::primative::PrimativeParser;
use crate::compiler::ast::{BramaAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};

pub struct FunctionDefinationParser;

impl SyntaxParserTrait for FunctionDefinationParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parser.backup();
        parser.indentation_check()?;

        if parser.match_keyword(BramaKeywordType::Fn) {
            let indentation = parser.get_indentation();
            let parser_flags  = parser.flags.get();

            parser.cleanup_whitespaces();

            let mut arguments = Vec::new();
            let name_expression = PrimativeParser::parse_symbol(parser)?;
            let name = match name_expression {
                BramaAstType::Symbol(text) => text,
                _ => return Err(("Function name not defined", 0, 0))
            };

            parser.cleanup_whitespaces();

            /* Arguments */
            if let Some(_) = parser.match_operator(&[BramaOperatorType::LeftParentheses]) {
                loop {
                    parser.cleanup_whitespaces();

                    if parser.check_operator(&BramaOperatorType::RightParentheses) {
                        break;
                    }

                    let argument = PrimativeParser::parse_symbol(parser)?;
                    match argument {
                        BramaAstType::None => return Err(("Argument must be a text", 0, 0)),
                        BramaAstType::Symbol(text) => arguments.push(text),
                        _ => return Err(("Argument not found", 0, 0))
                    };

                    parser.cleanup_whitespaces();
                    if let None = parser.match_operator(&[BramaOperatorType::Comma]) {
                        break;
                    }
                }

                if let None = parser.match_operator(&[BramaOperatorType::RightParentheses]) {
                    return Err(("')' missing", 0, 0));
                }
            }

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[BramaOperatorType::ColonMark]) {
                return Err(("':' missing", 0, 0));
            }

            parser.cleanup_whitespaces();
            parser.flags.set(parser_flags | SyntaxFlag::FUNCTION_DEFINATION);

            let mut body = match parser.get_newline() {
                (true, _) => {
                    parser.in_indication()?;
                    MultiLineBlockParser::parse(parser)
                },
                (false, _) => SingleLineBlockParser::parse(parser)
            }?;

            let has_return = match &body {
                BramaAstType::Return(_) => true,
                BramaAstType::Block(blocks) => {
                    if let BramaAstType::Return(_) = blocks[blocks.len() - 1] {
                        true
                    } else {
                        false
                    }
                },
                BramaAstType::None => return Err(("Function condition body not found", 0, 0)),
                _ => false
            };

            if !has_return {
                body = match body {
                    BramaAstType::Block(mut blocks) => {
                        blocks.push(BramaAstType::Return(Box::new(BramaAstType::None)));
                        BramaAstType::Block(blocks)
                    },
                    _ => {
                        BramaAstType::Block([body, BramaAstType::Return(Box::new(BramaAstType::None))].to_vec())
                    }
                }
            }

            parser.set_indentation(indentation);
            parser.flags.set(parser_flags);

            let function_defination_ast = BramaAstType::FunctionDefination {
                name: name.to_string(),
                body: Box::new(body),
                arguments: arguments.to_vec()
            };

            parser.set_indentation(indentation);
            return Ok(function_defination_ast);
        }
        
        parser.restore();
        return Ok(BramaAstType::None);
    }
}
