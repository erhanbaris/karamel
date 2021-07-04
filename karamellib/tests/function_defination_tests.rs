extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{KaramelError, KaramelErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::compiler::ast::KaramelAstType;
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

    test_compare!(func_def_1, r#"
fonk test():
    erhan=123"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: Vec::new(),
        body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
            variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
    })));
    test_compare!(func_def_2, r#"
fonk test(a):
    erhan=123"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: ["a".to_string()].to_vec(),
        body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
            variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
    })));
    test_compare!(func_def_3, r#"
fonk test(a, b    ,   c):
    erhan=123"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: ["a".to_string(), "b".to_string(), "c".to_string()].to_vec(),
        body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
            variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
        }),
        Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
    })));
    test_compare!(func_def_4, r#"
fonk test:
    erhan=123"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
            name: "test".to_string(),
            arguments: Vec::new(),
            body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
                variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
                operator: KaramelOperatorType::Assign,
                expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
            }),
            Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
        })));
        test_compare!(func_def_6, r#"
fonk test   :
    
    
    
        erhan=123"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
                name: "test".to_string(),
                arguments: Vec::new(),
                body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
                    variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
                    operator: KaramelOperatorType::Assign,
                    expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
                }),
                Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
            })));
            test_compare!(func_def_7, r#"
fonk test
    erhan=123"#, Err(KaramelError {
        error_type: KaramelErrorType::ColonMarkMissing,
        column: 9,
        line: 1
    }));
    test_compare!(func_def_8, r#"
fonk test(:
    erhan=123"#, Err(KaramelError {
        error_type: KaramelErrorType::ArgumentMustBeText,
        column: 11,
        line: 1
    }));
    test_compare!(func_def_9, r#"
fonk test(a:
    erhan=123"#, Err(KaramelError {
        error_type: KaramelErrorType::RightParanthesesMissing,
        column: 12,
        line: 1
    }));
    test_compare!(func_def_10, r#"
fonk test(a):
"#, Err(KaramelError {
    error_type: KaramelErrorType::FunctionConditionBodyNotFound,
    column: 13,
    line: 1
}));
test_compare!(func_def_11, r#"
fonk (a):
  a=1
"#, Err(KaramelError {
    error_type: KaramelErrorType::FunctionNameNotDefined,
    column: 6,
    line: 1
}));
test_compare!(func_def_12, r#"
fonk :
  a=1
"#, Err(KaramelError {
    error_type: KaramelErrorType::FunctionNameNotDefined,
    column: 6,
    line: 1
}));
test_compare!(func_def_13, r#"
fonk test(1):
  a=1
"#, Err(KaramelError {
    error_type: KaramelErrorType::ArgumentMustBeText,
    column: 11,
    line: 1
}));
test_compare!(func_def_14, r#"
test=1
döndür test
"#, Err(KaramelError {
    error_type: KaramelErrorType::ReturnMustBeUsedInFunction,
    column: 6,
    line: 2
}));
test_compare!(func_def_15, r#"
fonk test():
    erhan=123
    döndür erhan"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
    name: "test".to_string(),
    arguments: Vec::new(),
    body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::Symbol("erhan".to_string()))))].to_vec()))
})));
test_compare!(func_def_16, r#"
fonk test():
    erhan=123
    döndür"#, Ok(Rc::new(KaramelAstType::FunctionDefination {
    name: "test".to_string(),
    arguments: Vec::new(),
    body: Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
        variable: Box::new(KaramelAstType::Symbol("erhan".to_string())),
        operator: KaramelOperatorType::Assign,
        expression: Box::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
    }),
    Rc::new(KaramelAstType::Return(Box::new(KaramelAstType::None)))].to_vec()))
})));
}