extern crate libtpd;

#[cfg(test)]
mod tests {
    use crate::libtpd::parser::*;
    use crate::libtpd::syntax::*;
    use crate::libtpd::compiler::value::BramaPrimative;
    use crate::libtpd::compiler::ast::BramaAstType;
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

    test_compare!(func_call_7, "print(,2,'erhan')", Err(("Syntax error, undefined syntax", 0, 0)));
    test_compare!(func_call_8, "print(", Err(("Right parantheses missing", 0, 0)));
    test_compare!(func_call_9, "data=print()", Ok(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("data".to_string())),
        operator: libtpd::types::BramaOperatorType::Assign,
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
        operator: libtpd::types::BramaOperatorType::Addition,
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
        operator: libtpd::types::BramaOperatorType::GreaterThan,
        right: Box::new(BramaAstType::FuncCall {
            func_name_expression: Box::new(BramaAstType::Symbol("data2".to_string())),
            arguments: [].to_vec(),
            assign_to_temp: true
        })
    }));
    test_compare!(func_call_12, "gç::satıryaz", Ok(BramaAstType::FunctionMap(["gç".to_string(), "satıryaz".to_string()].to_vec())));
}