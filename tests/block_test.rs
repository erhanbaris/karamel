extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
    use crate::tpd::syntax::*;
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

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(block_1, "+1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    test_compare!(block_2, r#"erhan=1024
baris=2048"#, Ok(BramaAstType::Block([BramaAstType::Assignment {
    variable: Rc::new("erhan".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0))))
},
BramaAstType::Assignment {
    variable: Rc::new("baris".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2048.0))))
}].to_vec())));

test_compare!(block_3, "erhan=1024", Ok(BramaAstType::Assignment {
    variable: Rc::new("erhan".to_string()),
    operator: BramaOperatorType::Assign,
    expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0))))
}));
}