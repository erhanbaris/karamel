extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{KaramelError, KaramelErrorType};

    use crate::karamellib::types::*;
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::compiler::ast::{KaramelAstType, KaramelIfStatementElseItem};
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

    test_compare!(if_1, r#"1024 * 123 ise:
    erhan=123  
veya: 
  erhan=1234
"#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234.0))))
    })),
    else_if: Vec::new()
    })));

    test_compare!(if_2, r#"1024 * 123 ise:
    erhan=123   "#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    else_body:None,
    else_if: Vec::new()
    })));

    test_compare!(if_3, r#"1024 * 123 ise:
    erhan=123   
    print(1)"#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Box::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })
    ].to_vec())),
    else_body:None,
    else_if: Vec::new()
    })));

    test_compare!(if_4, r#"1024 * 123 ise : 
    erhan=123    
    print(1) 
veya : 
    erhan=321 
    print(2)"#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Box::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })
    ].to_vec())),
    else_body: Some(Box::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(321.0))))
    }),
    Rc::new(KaramelAstType::FuncCall {
        func_name_expression: Box::new(KaramelAstType::Symbol("print".to_string())),
        arguments: [Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: Cell::new(false)
    })
    ].to_vec()))),
    else_if: Vec::new()
    })));

    test_compare!(if_5, r#"1024 * 123 ise:
    erhan=123
veya 1024 * 123 > 10_000_000 ise:
    erhan=12345
veya:
    erhan=1234
"#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234.0))))
    })),
    else_if: [Box::new(KaramelIfStatementElseItem::new(Box::new(KaramelAstType::Control {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))),
            operator: KaramelOperatorType::Multiplication,
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        operator: KaramelOperatorType::GreaterThan,
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10000000.0)))),
    }), Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(12345.0))))
    })))].to_vec()
    })));

    test_compare!(if_6, r#"1024 * 123 ise:
    erhan=123
veya 1024 * 123 > 10_000_000 ise:
    erhan=12345
veya 1024 * 123 < 10_000_000 ise:
    erhan=123456
veya:
    erhan=1234
"#, Ok(Rc::new(KaramelAstType::IfStatement {
    condition: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    body: Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234.0))))
    })),
    else_if: [Box::new(KaramelIfStatementElseItem::new(Box::new(KaramelAstType::Control {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))),
            operator: KaramelOperatorType::Multiplication,
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        operator: KaramelOperatorType::GreaterThan,
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10000000.0)))),
    }), Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(12345.0))))
    }))),
    Box::new(KaramelIfStatementElseItem::new(Box::new(KaramelAstType::Control {
        left: Box::new(KaramelAstType::Binary {
            left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))),
            operator: KaramelOperatorType::Multiplication,
            right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        operator: KaramelOperatorType::LessThan,
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10000000.0)))),
    }), Box::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123456.0))))
    })))].to_vec()
    })));

    test_compare!(if_7, r#"1024 * 123 ise:
    erhan=123
veya 1024 * 123 > 10_000_000 ise:
    erhan=12345
veya 1024 * 123 < 10_000_000 ise:
    erhan=123456
veya:
    erhan=1234
veya:
    erhan=1234
"#, Err(KaramelError {
    error_type: KaramelErrorType::ElseIsUsed,
    column: 5,
    line: 8
}));

test_compare!(if_8, r#"1024 * 123 ise:
    erhan=123
veya:
    erhan=1234
veya 1024 * 123 > 10_000_000 ise:
    erhan=12345
veya 1024 * 123 < 10_000_000 ise:
    erhan=123456
"#, Err(KaramelError {
    error_type: KaramelErrorType::ElseIsUsed,
    column: 28,
    line: 4
}));



test_compare!(if_9, r#"1024 * 123 ise:
    erhan=123
veya 1024 * 123 > 10_000_000 ise:
    erhan=12345
"#, Ok(Rc::new(KaramelAstType::IfStatement {
condition: Box::new(KaramelAstType::Binary {
    left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))), 
    operator: KaramelOperatorType::Multiplication, 
    right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
}),
body: Box::new(KaramelAstType::Assignment {
    variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
}),
else_body: None,
else_if: [Box::new(KaramelIfStatementElseItem::new(Box::new(KaramelAstType::Control {
    left: Box::new(KaramelAstType::Binary {
        left: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0)))),
        operator: KaramelOperatorType::Multiplication,
        right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    operator: KaramelOperatorType::GreaterThan,
    right: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10000000.0)))),
}), Box::new(KaramelAstType::Assignment {
    variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(12345.0))))
})))].to_vec()
})));
}
