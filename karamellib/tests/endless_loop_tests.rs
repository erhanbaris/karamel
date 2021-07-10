extern crate karamellib;

#[cfg(test)]
mod tests {
    use karamellib::error::{KaramelError, KaramelErrorType};

    use crate::karamellib::types::*;
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
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

    /*test_compare!(endless_1, r#"sonsuz:
    erhan=123
"#, Ok(Rc::new(KaramelAstType::EndlessLoop(Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
})))));*/
/*test_compare!(endless_2, r#"sonsuz:
    erhan=123   
    print(1)"#, Ok(Rc::new(KaramelAstType::EndlessLoop(Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
}),
Rc::new(KaramelAstType::FuncCall {
    func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
    arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: Cell::new(false)
})
].to_vec()))))));
test_compare!(endless_3, r#"sonsuz
    erhan=123   
    print(1)"#, Err(KaramelError {
        error_type: KaramelErrorType::ColonMarkMissing,
        line: 0,
        column: 6
    }));*/
/*test_compare!(endless_4, r#"sonsuz:
    erhan=123   
    print(1)
    kır"#, Ok(Rc::new(KaramelAstType::EndlessLoop(Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))
}),
Rc::new(KaramelAstType::FuncCall {
    func_name_expression: Rc::new(KaramelAstType::Symbol("print".to_string())),
    arguments: [Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec(),
    assign_to_temp: Cell::new(false)
}),
Rc::new(KaramelAstType::Break)
].to_vec()))))));*/
test_compare!(endless_5, r#"kır"#, Err(KaramelError {
    error_type: KaramelErrorType::BreakAndContinueBelongToLoops,
    column: 3,
    line: 0
}));
test_compare!(endless_6, r#"devam"#, Err(KaramelError {
    error_type: KaramelErrorType::BreakAndContinueBelongToLoops,
    column: 5,
    line: 0
}));
}
