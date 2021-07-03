use crate::types::*;
use crate::syntax::{SyntaxParser};
use crate::syntax::ParseType;
use crate::compiler::ast::BramaAstType;
use crate::error::BramaErrorType;

// https://github.com/rust-lang/rust/issues/75429

pub fn map_parser(parser: &SyntaxParser, parser_funcs: &[ParseType]) -> AstResult {
    for parser_func in parser_funcs {
        match parser_func(parser) {
            Ok(BramaAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(BramaAstType::None)
}

pub fn is_ast_empty(ast: &AstResult) -> bool {
    match ast {
        Ok(BramaAstType::None) => true,
        Ok(_) => false,
        Err(_) => true
    }
}

pub fn err_or_message(ast: &AstResult, error: BramaErrorType) -> AstResult {
    match &ast {
        Ok(BramaAstType::None) => Err(error),
        Ok(_) => Ok(BramaAstType::None),
        Err(error) => Err(*error)
    }
}

pub fn update_functions_for_temp_return(ast: &BramaAstType) {
    match ast {
        BramaAstType::FuncCall { func_name_expression: _, arguments: _, assign_to_temp } => {
            assign_to_temp.set(true);
        },
        BramaAstType::AccessorFuncCall {
            source,
            indexer,
            assign_to_temp
        } => {
            update_functions_for_temp_return(source);
            update_functions_for_temp_return(indexer);
            assign_to_temp.set(true);
        },
        BramaAstType::Block(blocks) => {
            for block in blocks {
                update_functions_for_temp_return(&block);
            }
        },
        _ => ()
    };
}