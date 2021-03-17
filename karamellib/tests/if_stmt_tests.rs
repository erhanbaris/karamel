extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{BramaError, BramaErrorType};

    use crate::karamellib::types::*;
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::BramaPrimative;
    use crate::karamellib::compiler::ast::{BramaAstType, BramaIfStatementElseItem};
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

    test_compare!(if_1, r#"eğer 1024 * 123:
    erhan=123  
yada: 
  erhan=1234
"#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234.0))))
    })),
    else_if: Vec::new()
    }));

    test_compare!(if_2, r#"eğer 1024 * 123:
    erhan=123   "#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    else_body:None,
    else_if: Vec::new()
    }));

    test_compare!(if_3, r#"eğer 1024 * 123:
    erhan=123   
    print(1)"#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Block([BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    },
    BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: false
    }
    ].to_vec())),
    else_body:None,
    else_if: Vec::new()
    }));

    test_compare!(if_4, r#"eğer 1024 * 123 : 
    erhan=123    
    print(1) 
yada : 
    erhan=321 
    print(2)"#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Block([BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    },
    BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
        assign_to_temp: false
    }
    ].to_vec())),
    else_body: Some(Box::new(BramaAstType::Block([BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(321.0))))
    },
    BramaAstType::FuncCall {
        func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))].to_vec(),
        assign_to_temp: false
    }
    ].to_vec()))),
    else_if: Vec::new()
    }));

    test_compare!(if_5, r#"eğer 1024 * 123:
    erhan=123
yada 1024 * 123 > 10_000_000:
    erhan=12345
yada:
    erhan=1234
"#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234.0))))
    })),
    else_if: [Box::new(BramaIfStatementElseItem::new(Box::new(BramaAstType::Control {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))),
            operator: BramaOperatorType::Multiplication,
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        }),
        operator: BramaOperatorType::GreaterThan,
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10000000.0)))),
    }), Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(12345.0))))
    })))].to_vec()
    }));

    test_compare!(if_6, r#"eğer 1024 * 123:
    erhan=123
yada 1024 * 123 > 10_000_000:
    erhan=12345
yada 1024 * 123 < 10_000_000:
    erhan=123456
yada:
    erhan=1234
"#, Ok(BramaAstType::IfStatement {
    condition: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
        operator: BramaOperatorType::Multiplication, 
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    body: Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    else_body: Some(Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1234.0))))
    })),
    else_if: [Box::new(BramaIfStatementElseItem::new(Box::new(BramaAstType::Control {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))),
            operator: BramaOperatorType::Multiplication,
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        }),
        operator: BramaOperatorType::GreaterThan,
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10000000.0)))),
    }), Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(12345.0))))
    }))),
    Box::new(BramaIfStatementElseItem::new(Box::new(BramaAstType::Control {
        left: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))),
            operator: BramaOperatorType::Multiplication,
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        }),
        operator: BramaOperatorType::LessThan,
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10000000.0)))),
    }), Box::new(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123456.0))))
    })))].to_vec()
    }));

    test_compare!(if_7, r#"eğer 1024 * 123:
    erhan=123
yada 1024 * 123 > 10_000_000:
    erhan=12345
yada 1024 * 123 < 10_000_000:
    erhan=123456
yada:
    erhan=1234
yada:
    erhan=1234
"#, Err(BramaError {
    error_type: BramaErrorType::ElseIsUsed,
    column: 4,
    line: 0
}));

test_compare!(if_8, r#"eğer 1024 * 123:
    erhan=123
yada:
    erhan=1234
yada 1024 * 123 > 10_000_000:
    erhan=12345
yada 1024 * 123 < 10_000_000:
    erhan=123456
"#, Err(BramaError {
    error_type: BramaErrorType::ElseIsUsed,
    column: 4,
    line: 0
}));



test_compare!(if_9, r#"eğer 1024 * 123:
    erhan=123
yada 1024 * 123 > 10_000_000:
    erhan=12345
"#, Ok(BramaAstType::IfStatement {
condition: Box::new(BramaAstType::Binary {
    left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))), 
    operator: BramaOperatorType::Multiplication, 
    right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
}),
body: Box::new(BramaAstType::Assignment {
    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
}),
else_body: None,
else_if: [Box::new(BramaIfStatementElseItem::new(Box::new(BramaAstType::Control {
    left: Box::new(BramaAstType::Binary {
        left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))),
        operator: BramaOperatorType::Multiplication,
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    }),
    operator: BramaOperatorType::GreaterThan,
    right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(10000000.0)))),
}), Box::new(BramaAstType::Assignment {
    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(12345.0))))
})))].to_vec()
}));
}
