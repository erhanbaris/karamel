use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;
use crate::syntax::block::{MultiLineBlockParser, SingleLineBlockParser};
use crate::syntax::primative::PrimativeParser;
use crate::syntax::{SyntaxFlag, SyntaxParser, SyntaxParserTrait};
use crate::types::*;
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
            if parser
                .match_operator(&[KaramelOperatorType::LeftParentheses])
                .is_some()
            {
                loop {
                    parser.cleanup_whitespaces();

                    if parser.check_operator(&KaramelOperatorType::RightParentheses) {
                        break;
                    }

                    let argument = PrimativeParser::parse_symbol(parser)?;
                    match argument {
                        KaramelAstType::Symbol(text) => arguments.push(text),
                        _ => return Err(KaramelErrorType::ArgumentMustBeText),
                    };

                    parser.cleanup_whitespaces();
                    if parser
                        .match_operator(&[KaramelOperatorType::Comma])
                        .is_none()
                    {
                        break;
                    }
                }

                if parser
                    .match_operator(&[KaramelOperatorType::RightParentheses])
                    .is_none()
                {
                    return Err(KaramelErrorType::RightParanthesesMissing);
                }
            }

            parser.cleanup_whitespaces();
            if parser
                .match_operator(&[KaramelOperatorType::ColonMark])
                .is_none()
            {
                return Err(KaramelErrorType::ColonMarkMissing);
            }

            parser.cleanup_whitespaces();
            let parser_flags = parser.flags.get();
            parser
                .flags
                .set(parser_flags | SyntaxFlag::FUNCTION_DEFINATION);

            let mut body = match parser.get_newline() {
                (true, _) => {
                    parser.in_indication()?;
                    MultiLineBlockParser::parse(parser)
                }
                (false, _) => SingleLineBlockParser::parse(parser),
            }?;

            let has_return = match &body {
                KaramelAstType::Return(_) => true,
                KaramelAstType::Block(blocks) => {
                    matches!(&*blocks[blocks.len() - 1], KaramelAstType::Return(_))
                }
                KaramelAstType::None => return Err(KaramelErrorType::FunctionConditionBodyNotFound),
                _ => false,
            };

            if !has_return {
                body = match body {
                    KaramelAstType::Block(mut blocks) => {
                        blocks.push(Rc::new(KaramelAstType::Return(Rc::new(KaramelAstType::None))));
                        KaramelAstType::Block(blocks)
                    }
                    _ => KaramelAstType::Block([Rc::new(body), Rc::new(KaramelAstType::Return(Rc::new(KaramelAstType::None)))].to_vec()),
                }
            }

            parser.set_indentation(indentation);
            parser.flags.set(parser_flags);

            let function_defination_ast = KaramelAstType::FunctionDefination { name: function_name, body: Rc::new(body), arguments };

            parser.set_indentation(indentation);
            return Ok(function_defination_ast);
        }

        parser.set_index(index_backup);
        Ok(KaramelAstType::None)
    }
}
