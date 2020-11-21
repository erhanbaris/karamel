extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
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

    test_compare!(unary_1, "+1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    test_compare!(unary_2, "-1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    test_compare!(unary_3, "+1024.0", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    test_compare!(unary_4, "-1024.0", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    test_compare!(unary_5, "değil true", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))))));
    test_compare!(unary_6, "değil false", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))))));
    test_compare!(unary_7, "not doğru", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))))));
    test_compare!(unary_8, "not yanlış", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))))));
    
    test_compare!(unary_9, "+[]", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_10, "++100", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_11, "--100", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_12, "--true", Err(("Invalid unary operation", 0, 0)));

    test_compare!(unary_13, "++data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_14, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_15, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_16, "data--", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_17, "data++", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol("data".to_string())))));

    test_compare!(unary_18, "+ 1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))))));
    //test_compare!(unary_19, "doğru değil", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true))))));
}