use crate::types::*;
use crate::syntax::{SyntaxParser};
use crate::syntax::ParseType;
use crate::compiler::ast::KaramelAstType;
use crate::error::KaramelErrorType;
use crate::syntax::SyntaxFlag;

// https://github.com/rust-lang/rust/issues/75429

pub fn map_parser<'a>(parser: &SyntaxParser<'a>, parser_funcs: &[ParseType::<'a>]) -> AstResult<'a> {
    for parser_func in parser_funcs {
        match parser_func(parser) {
            Ok(KaramelAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(KaramelAstType::None)
}

pub fn map_parser_with_flag<'a>(flag: SyntaxFlag, parser: &SyntaxParser<'a>, parser_funcs: &[ParseType::<'a>]) -> AstResult<'a> {
    for parser_func in parser_funcs {
        match with_flag(flag, parser, || parser_func(parser)) {
            Ok(KaramelAstType::None) => (),
            Ok(ast) => return Ok(ast),
            Err(err) => return Err(err)
        }
    }

    Ok(KaramelAstType::None)
}

pub fn is_ast_empty<'a>(ast: &AstResult<'a>) -> bool {
    match ast {
        Ok(KaramelAstType::None) => true,
        Ok(_) => false,
        Err(_) => true
    }
}

pub fn err_or_message<'a>(ast: AstResult<'a>, none_error: KaramelErrorType) -> AstResult<'a> {
    match ast {
        Ok(KaramelAstType::None) => Err(none_error),
        Ok(_) => Ok(KaramelAstType::None),
        Err(error) => Err(error)
    }
}

pub fn update_functions_for_temp_return<'a>(ast: &KaramelAstType<'a>) {
    match ast {
        KaramelAstType::FuncCall { func_name_expression: _, arguments: _, assign_to_temp } => {
            assign_to_temp.set(true);
        },
        KaramelAstType::AccessorFuncCall {
            source,
            indexer,
            assign_to_temp
        } => {
            update_functions_for_temp_return(source);
            update_functions_for_temp_return(indexer);
            assign_to_temp.set(true);
        },
        KaramelAstType::Block(blocks) => {
            for block in blocks {
                update_functions_for_temp_return(&block);
            }
        },
        _ => ()
    };
}

pub fn with_flag<'a, F: Fn() -> AstResult<'a>>(flag: SyntaxFlag, parser: &SyntaxParser<'a>, func: F) -> AstResult<'a> {
    let parser_flags  = parser.flags.get();
    parser.flags.set(parser_flags | flag);
    let loop_control = func()?;
    parser.flags.set(parser_flags);
    Ok(loop_control)
}

pub fn mut_with_flag<'a, F: FnMut() -> AstResult<'a>>(flag: SyntaxFlag, parser: &SyntaxParser<'a>, mut func: F) -> AstResult<'a> {
    let parser_flags  = parser.flags.get();
    parser.flags.set(parser_flags | flag);
    let loop_control = func()?;
    parser.flags.set(parser_flags);
    Ok(loop_control)
}