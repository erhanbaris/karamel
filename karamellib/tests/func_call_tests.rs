extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{BramaError, BramaErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::BramaPrimative;
    use crate::karamellib::compiler::ast::BramaAstType;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(func_call_1, "print()", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [].to_vec(),
        assign_to_temp: false
    }));

    test_compare!(func_call_2, "print(1)", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: false
    }));

    test_compare!(func_call_3, "print( 1 )", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: false
    }));

    test_compare!(func_call_4, "print( 1 , 2 )", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: false
    }));

    test_compare!(func_call_5, "print(1,2)", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: false
    }));

    test_compare!(func_call_6, "print(1,2,'erhan')", Ok(BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))),
                    Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0)))),
                    Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("erhan".to_string())))))].to_vec(),
                    assign_to_temp: false
    }));

    test_compare!(func_call_7, "print(,2,'erhan')", Err(BramaError {
        error_type: BramaErrorType::SyntaxError,
        column: 6,
        line: 0
    }));
    test_compare!(func_call_8, "print(", Err(BramaError {
        error_type: BramaErrorType::RightParanthesesMissing,
        column: 6,
        line: 0
    }));
    test_compare!(func_call_9, "data=print()", Ok(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("data".to_string())),
        operator: karamellib::types::BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        })
    }));
    test_compare!(func_call_10, "data1() + data2()", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("data1".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        }),
        operator: karamellib::types::BramaOperatorType::Addition,
        right: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("data2".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        })
    }));
    test_compare!(func_call_11, "data1() > data2()", Ok(BramaAstType::Control {
        left: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("data1".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        }),
        operator: karamellib::types::BramaOperatorType::GreaterThan,
        right: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("data2".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        })
    }));
    test_compare!(func_call_12, "gç::satıryaz", Ok(BramaAstType::FunctionMap(["gç".to_string(), "satıryaz".to_string()].to_vec())));
}