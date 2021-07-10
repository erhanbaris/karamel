use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::func_call::FuncCallParser;
use crate::syntax::unary::UnaryParser;
use crate::syntax::control::OrParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::KaramelAstType;
use crate::compiler::value::KaramelPrimative;
use crate::error::KaramelErrorType;

use super::util::{mut_with_flag, with_flag};

pub struct ExpressionParser;

impl SyntaxParserTrait for ExpressionParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut ast = OrParser::parse(parser)?;
    
        loop {
            let index_backup = parser.get_index();

            /* parse for 'object()()' */
            if FuncCallParser::parsable(parser) {
                update_functions_for_temp_return(&ast);
                ast = mut_with_flag(SyntaxFlag::IN_DICT_INDEXER, parser, || FuncCallParser::parse_suffix(&mut ast, parser))?;
            }
            
            /* parse for 'object.method' */
            else if let Some(_) = parser.match_operator(&[KaramelOperatorType::Dot]) {

                let sub_ast = with_flag(SyntaxFlag::IN_DICT_INDEXER, parser, || ExpressionParser::parse(parser))?;
                ast = match &sub_ast {
                    KaramelAstType::Symbol(symbol) => {
                        KaramelAstType::Indexer 
                        { 
                            body: Rc::new(ast),
                            
                            /* Convert symbol to text */
                            indexer: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new(symbol.to_string()))))) 
                        }
                    },
                    _ => return Err(KaramelErrorType::FunctionCallSyntaxNotValid)
                };
            }
            
            /* parse for '["data"]' */
            else if parser.check_operator(&KaramelOperatorType::SquareBracketStart) {
                ast = UnaryParser::parse_indexer(Rc::new(ast), parser)?;
            } else {
                parser.set_index(index_backup);
                break;
            }
        }

        Ok(ast)
    }
}
