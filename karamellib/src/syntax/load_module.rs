use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait};
use crate::compiler::ast::KaramelAstType;
use super::primative::PrimativeParser;
use super::util::map_parser;

pub struct LoadModuleParser;

impl SyntaxParserTrait for LoadModuleParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let index_backup = parser.get_index();
        parser.indentation_check()?;

        if parser.peek_token().is_ok() {
            let module_path = map_parser(parser, &[PrimativeParser::parse_module_path, PrimativeParser::parse_symbol])?;
            match module_path {

                /* module1::module2::module3 */
                KaramelAstType::ModulePath(path) => {
                    parser.cleanup_whitespaces();

                    if parser.match_keyword(KaramelKeywordType::Load) {
                        if path.len() > 0 {
                            return Ok(KaramelAstType::Load(path.to_vec()));
                        }
                    }
                },

                /* module1 */
                KaramelAstType::Symbol(path) => {
                    parser.cleanup_whitespaces();

                    if parser.match_keyword(KaramelKeywordType::Load) {
                        if path.len() > 0 {
                            return Ok(KaramelAstType::Load([path].to_vec()));
                        }
                    }
                }
                _ => ()
            };
        }

        parser.set_index(index_backup);
        return Ok(KaramelAstType::None);
    }
}
