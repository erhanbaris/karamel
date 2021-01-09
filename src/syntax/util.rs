use crate::types::*;
use crate::syntax::SyntaxParser;
use crate::syntax::ParseType;
use crate::compiler::ast::BramaAstType;

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

pub fn err_or_message(ast: &AstResult, message: &'static str) -> AstResult {
    match &ast {
        Ok(BramaAstType::None) => Err((message, 0, 0,)),
        Ok(_) => Ok(BramaAstType::None),
        Err((m, l, c)) => Err((m, *l, *c))
    }
}

pub fn update_functions_for_temp_return(ast: &mut BramaAstType) {
    match ast {
        BramaAstType::FuncCall { func_name_expression: _, arguments: _, assign_to_temp } => {
            *assign_to_temp = true;
        },
        BramaAstType::AccessorFuncCall {
            source,
            target,
            assign_to_temp
        } => {
            update_functions_for_temp_return(source);
            update_functions_for_temp_return(target);
            *assign_to_temp = true;
        },
        BramaAstType::Block(blocks) => {
            for mut block in blocks {
                update_functions_for_temp_return(&mut block);
            }
        },
        _ => ()
    };
}