use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::func_call::FuncCallParser;
use crate::syntax::unary::UnaryParser;
use crate::syntax::control::OrParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;
use crate::error::BramaErrorType;

pub struct ExpressionParser;

impl SyntaxParserTrait for ExpressionParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut ast = OrParser::parse(parser)?;
    
        loop {
            let index_backup = parser.get_index();

            /* parse for 'object()()' */
            if FuncCallParser::parsable(parser) {
                update_functions_for_temp_return(&mut ast);

                let inner_parser_flags  = parser.flags.get();
                parser.flags.set(inner_parser_flags | SyntaxFlag::IN_DICT_INDEXER);
                ast = FuncCallParser::parse_suffix(Box::new(ast), parser)?;
                parser.flags.set(inner_parser_flags);
            }
            
            /* parse for 'object.method()' or 'object.method' */
            else if let Some(_) = parser.match_operator(&[BramaOperatorType::Dot]) {

                let inner_parser_flags  = parser.flags.get();
                parser.flags.set(inner_parser_flags | SyntaxFlag::IN_DICT_INDEXER);

                let sub_ast = ExpressionParser::parse(parser)?;
                parser.flags.set(inner_parser_flags);
                
                ast = match &sub_ast {
                    BramaAstType::Symbol(symbol) => {
                        BramaAstType::Indexer 
                        { 
                            body: Box::new(ast),
                            
                            /* Convert symbol to text */
                            indexer: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new(symbol.to_string()))))) 
                        }
                    },
                    BramaAstType::FuncCall {
                        func_name_expression,
                        arguments: _,
                        assign_to_temp: _ 
                    } => {
                        match &**func_name_expression {
                            BramaAstType::Symbol(_) => {
                                update_functions_for_temp_return(&mut ast);
                                BramaAstType::AccessorFuncCall {
                                    source: Box::new(ast),
                                    indexer: Box::new(sub_ast),
                                    assign_to_temp: true
                                }
                            },
                            _ => {
                                log::debug!("Function call syntax not valid {:?}", func_name_expression);
                                return Err(BramaErrorType::FunctionCallSyntaxNotValid);
                            }
                        }
                    },
                    _ => return Err(BramaErrorType::FunctionCallSyntaxNotValid)
                };
            }
            
            /* parse for '["data"]' */
            else if parser.check_operator(&BramaOperatorType::SquareBracketStart) {
                ast = UnaryParser::parse_indexer(Box::new(ast), parser)?;
            } else {
                parser.set_index(index_backup);
                break;
            }
        }

        Ok(ast)
    }
}
