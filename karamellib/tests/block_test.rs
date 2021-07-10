extern crate karamellib;

#[cfg(test)]
mod tests {
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

    test_compare!(block_1, "+1024", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))));
    test_compare!(block_2, r#"erhan=1024
baris=2048"#, Ok(Rc::new(KaramelAstType::Block([Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))
}),
Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("baris".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2048.0))))
})].to_vec()))));

test_compare!(block_3, "erhan=1024", Ok(Rc::new(KaramelAstType::Assignment {
    variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
    operator: KaramelOperatorType::Assign,
    expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))
})));
}