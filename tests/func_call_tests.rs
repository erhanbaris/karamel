extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::syntax::*;
    use crate::tpd::compiler::value::BramaPrimative;
    use crate::tpd::compiler::ast::BramaAstType;
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

    test_compare!(func_call_1, "print()", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [].to_vec()
    }));

    test_compare!(func_call_2, "print(1)", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec()
    }));

    test_compare!(func_call_3, "print( 1 )", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))].to_vec()
    }));

    test_compare!(func_call_4, "print( 1 , 2 )", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))].to_vec()
    }));

    test_compare!(func_call_5, "print(1,2)", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))), Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))].to_vec()
    }));

    test_compare!(func_call_6, "print(1,2,'erhan')", Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0)))),
                    Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0)))),
                    Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("erhan".to_string())))))].to_vec()
    }));

    test_compare!(func_call_7, "print(,2,'erhan')", Err(("Syntax error, undefined syntax", 0, 0)));
    test_compare!(func_call_8, "print(", Err(("Right parantheses missing", 0, 0)));
}