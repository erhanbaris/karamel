extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{KaramelError, KaramelErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::compiler::ast::KaramelAstType;
    use std::cell::Cell;
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

    test_compare!(func_call_1, "print()", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: Vec::new(),
        assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_2, "print(1)", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_3, "print( 1 )", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_4, "print( 1 , 2 )", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0)))), Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_5, "print(1,2)", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0)))), Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_6, "print(1,2,'erhan')", Ok(Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0)))),
                    Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0)))),
                    Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new("erhan".to_string())))))].to_vec(),
                    assign_to_temp: Cell::new(false)
    })));

    test_compare!(func_call_7, "print(,2,'erhan')", Err(KaramelError {
        error_type: KaramelErrorType::SyntaxError,
        column: 6,
        line: 0
    }));
    test_compare!(func_call_8, "print(", Err(KaramelError {
        error_type: KaramelErrorType::RightParanthesesMissing,
        column: 6,
        line: 0
    }));
    test_compare!(func_call_9, "data=print()", Ok(Rc::new(KaramelAstType::Assignment {
        variable: Rc::new(KaramelAstType::Symbol("data".to_string())),
        operator: karamellib::types::KaramelOperatorType::Assign,
        expression: Rc::new(KaramelAstType::FuncCall {
            func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
            arguments: Vec::new(),
            assign_to_temp: Cell::new(true)
        })
    })));
    test_compare!(func_call_10, "data1() + data2()", Ok(Rc::new(KaramelAstType::Binary {
        left: Rc::new(KaramelAstType::FuncCall {
            func_name_expression: Rc::new(KaramelAstType::Symbol("data1".to_string())),
            arguments: Vec::new(),
            assign_to_temp: Cell::new(true)
        }),
        operator: karamellib::types::KaramelOperatorType::Addition,
        right: Rc::new(KaramelAstType::FuncCall {
            func_name_expression: Rc::new(KaramelAstType::Symbol("data2".to_string())),
            arguments: Vec::new(),
            assign_to_temp: Cell::new(true)
        })
    })));
    test_compare!(func_call_11, "data1() > data2()", Ok(Rc::new(KaramelAstType::Control {
        left: Rc::new(KaramelAstType::FuncCall {
            func_name_expression: Rc::new(KaramelAstType::Symbol("data1".to_string())),
            arguments: Vec::new(),
            assign_to_temp: Cell::new(true)
        }),
        operator: karamellib::types::KaramelOperatorType::GreaterThan,
        right: Rc::new(KaramelAstType::FuncCall {
            func_name_expression: Rc::new(KaramelAstType::Symbol("data2".to_string())),
            arguments: Vec::new(),
            assign_to_temp: Cell::new(true)
        })
    })));
    test_compare!(func_call_12, "gç::satıryaz", Ok(Rc::new(KaramelAstType::ModulePath(["gç".to_string(), "satıryaz".to_string()].to_vec()))));
}