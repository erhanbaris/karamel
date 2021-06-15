extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{BramaError, BramaErrorType};

    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;
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

    test_compare!(func_def_1, r#"
fonk test():
    erhan=123"#, Ok(BramaAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: [].to_vec(),
        body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
            variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
            operator: BramaOperatorType::Assign,
            expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        },
        BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
    }));
    test_compare!(func_def_2, r#"
fonk test(a):
    erhan=123"#, Ok(BramaAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: ["a".to_string()].to_vec(),
        body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
            variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
            operator: BramaOperatorType::Assign,
            expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        },
        BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
    }));
    test_compare!(func_def_3, r#"
fonk test(a, b    ,   c):
    erhan=123"#, Ok(BramaAstType::FunctionDefination {
        name: "test".to_string(),
        arguments: ["a".to_string(), "b".to_string(), "c".to_string()].to_vec(),
        body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
            variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
            operator: BramaOperatorType::Assign,
            expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
        },
        BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
    }));
    test_compare!(func_def_4, r#"
fonk test:
    erhan=123"#, Ok(BramaAstType::FunctionDefination {
            name: "test".to_string(),
            arguments: [].to_vec(),
            body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
                variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
                operator: BramaOperatorType::Assign,
                expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
            },
            BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
        }));
        test_compare!(func_def_6, r#"
fonk test   :
    
    
    
        erhan=123"#, Ok(BramaAstType::FunctionDefination {
                name: "test".to_string(),
                arguments: [].to_vec(),
                body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
                    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
                    operator: BramaOperatorType::Assign,
                    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
                },
                BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
            }));
            test_compare!(func_def_7, r#"
fonk test
    erhan=123"#, Err(BramaError {
        error_type: BramaErrorType::ColonMarkMissing,
        column: 9,
        line: 1
    }));
    test_compare!(func_def_8, r#"
fonk test(:
    erhan=123"#, Err(BramaError {
        error_type: BramaErrorType::ArgumentMustBeText,
        column: 11,
        line: 1
    }));
    test_compare!(func_def_9, r#"
fonk test(a:
    erhan=123"#, Err(BramaError {
        error_type: BramaErrorType::RightParanthesesMissing,
        column: 12,
        line: 1
    }));
    test_compare!(func_def_10, r#"
fonk test(a):
"#, Err(BramaError {
    error_type: BramaErrorType::FunctionConditionBodyNotFound,
    column: 13,
    line: 1
}));
test_compare!(func_def_11, r#"
fonk (a):
  a=1
"#, Err(BramaError {
    error_type: BramaErrorType::FunctionNameNotDefined,
    column: 6,
    line: 1
}));
test_compare!(func_def_12, r#"
fonk :
  a=1
"#, Err(BramaError {
    error_type: BramaErrorType::FunctionNameNotDefined,
    column: 6,
    line: 1
}));
test_compare!(func_def_13, r#"
fonk test(1):
  a=1
"#, Err(BramaError {
    error_type: BramaErrorType::ArgumentMustBeText,
    column: 11,
    line: 1
}));
test_compare!(func_def_14, r#"
test=1
döndür test
"#, Err(BramaError {
    error_type: BramaErrorType::ReturnMustBeUsedInFunction,
    column: 6,
    line: 2
}));
test_compare!(func_def_15, r#"
fonk test():
    erhan=123
    döndür erhan"#, Ok(BramaAstType::FunctionDefination {
    name: "test".to_string(),
    arguments: [].to_vec(),
    body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    },
    BramaAstType::Return(Box::new(BramaAstType::Symbol("erhan".to_string())))].to_vec()))
}));
test_compare!(func_def_16, r#"
fonk test():
    erhan=123
    döndür"#, Ok(BramaAstType::FunctionDefination {
    name: "test".to_string(),
    arguments: [].to_vec(),
    body: Rc::new(BramaAstType::Block([BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
    },
    BramaAstType::Return(Box::new(BramaAstType::None))].to_vec()))
}));
}