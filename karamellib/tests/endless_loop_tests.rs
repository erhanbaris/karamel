extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{BramaError, BramaErrorType};

    use crate::karamellib::types::*;
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::BramaPrimative;
    use crate::karamellib::compiler::ast::{BramaAstType};
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

    test_compare!(endless_1, r#"sonsuz:
    erhan=123
"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Assignment {
    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
}))));
test_compare!(endless_2, r#"sonsuz:
    erhan=123   
    print(1)"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Block([BramaAstType::Assignment {
    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
},
BramaAstType::FuncCall {
    func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
    arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: false
}
].to_vec())))));
test_compare!(endless_3, r#"sonsuz
    erhan=123   
    print(1)"#, Err(BramaError {
        error_type: BramaErrorType::ColonMarkMissing,
        line: 0,
        column: 6
    }));
test_compare!(endless_4, r#"sonsuz:
    erhan=123   
    print(1)
    kır"#, Ok(BramaAstType::EndlessLoop(Box::new(BramaAstType::Block([BramaAstType::Assignment {
    variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(123.0))))
},
BramaAstType::FuncCall {
    func_name_expression: Box::new(BramaAstType::Symbol("print".to_string())),
    arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: false
},
BramaAstType::Break
].to_vec())))));
test_compare!(endless_5, r#"kır"#, Err(BramaError {
    error_type: BramaErrorType::BreakAndContinueBelongToLoops,
    column: 3,
    line: 0
}));
test_compare!(endless_6, r#"devam"#, Err(BramaError {
    error_type: BramaErrorType::BreakAndContinueBelongToLoops,
    column: 5,
    line: 0
}));
}
