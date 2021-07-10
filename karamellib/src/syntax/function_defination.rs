use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag};
use crate::syntax::primative::PrimativeParser;
use crate::compiler::ast::{KaramelAstType};
use crate::syntax::block::{SingleLineBlockParser, MultiLineBlockParser};
use crate::error::KaramelErrorType;
use std::rc::Rc;

pub struct FunctionDefinationParser;

impl SyntaxParserTrait for FunctionDefinationParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        if parser.match_keyword(KaramelKeywordType::Fn) {
            let indentation = parser.get_indentation();

            parser.cleanup_whitespaces();

            let mut arguments = Vec::new();
            let name_expression = PrimativeParser::parse_symbol(parser)?;
            let function_name = match name_expression {
                KaramelAstType::Symbol(text) => text,
                _ => {
                    return Err(KaramelErrorType::FunctionNameNotDefined);
                }
            };

            parser.cleanup_whitespaces();

            /* Arguments */
            if let Some(_) = parser.match_operator(&[KaramelOperatorType::LeftParentheses]) {
                loop {
                    parser.cleanup_whitespaces();

                    if parser.check_operator(&KaramelOperatorType::RightParentheses) {
                        break;
                    }

                    let argument = PrimativeParser::parse_symbol(parser)?;
                    match argument {
                        KaramelAstType::Symbol(text) => arguments.push(text),
                        _ => return Err(KaramelErrorType::ArgumentMustBeText)
                    };

                    parser.cleanup_whitespaces();
                    if let None = parser.match_operator(&[KaramelOperatorType::Comma]) {
                        break;
                    }
                }

                if let None = parser.match_operator(&[KaramelOperatorType::RightParentheses]) {
                    return Err(KaramelErrorType::RightParanthesesMissing);
                }
            }

            parser.cleanup_whitespaces();
            if let None = parser.match_operator(&[KaramelOperatorType::ColonMark]) {
                return Err(KaramelErrorType::ColonMarkMissing);
            }

            parser.cleanup_whitespaces();
            let parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::FUNCTION_DEFINATION);

            let mut body = match parser.get_newline() {
                (true, _) => {
                    parser.in_indication()?;
                    MultiLineBlockParser::parse(parser)
                },
                (false, _) => SingleLineBlockParser::parse(parser)
            }?;

            let has_return = match &body {
                KaramelAstType::Return(_) => true,
                KaramelAstType::Block(blocks) =>
                    match &*blocks[blocks.len() - 1] {
                        KaramelAstType::Return(_) => true,
                        _ => false
                    },
                KaramelAstType::None => return Err(KaramelErrorType::FunctionConditionBodyNotFound),
                _ => false
            };

            if !has_return {
                body = match body {
                    KaramelAstType::Block(mut blocks) => {
                        blocks.push(Rc::new(KaramelAstType::Return(Rc::new(KaramelAstType::None))));
                        KaramelAstType::Block(blocks)
                    },
                    _ => {
                        KaramelAstType::Block([Rc::new(body), Rc::new(KaramelAstType::Return(Rc::new(KaramelAstType::None)))].to_vec())
                    }
                }
            }

            parser.set_indentation(indentation);
            parser.flags.set(parser_flags);

            let function_defination_ast = KaramelAstType::FunctionDefination {
                name: function_name,
                body: Rc::new(body),
                arguments: arguments
            };

            parser.set_indentation(indentation);
            return Ok(function_defination_ast);
        }
        
        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
