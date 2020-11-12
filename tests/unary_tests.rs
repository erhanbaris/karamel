extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::syntax::*;
    use crate::tpd::types::*;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                parser.parse();
                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(unary_1, "+1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(BramaPrimative::Integer(1024))))));
    test_compare!(unary_2, "-1024", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Integer(1024))))));
    test_compare!(unary_3, "+1024.0", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Addition, Box::new(BramaAstType::Primative(BramaPrimative::Double(1024.0))))));
    test_compare!(unary_4, "-1024.0", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Subtraction, Box::new(BramaAstType::Primative(BramaPrimative::Double(1024.0))))));
    test_compare!(unary_5, "!true", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))))));
    test_compare!(unary_6, "!false", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(BramaPrimative::Bool(false))))));
    test_compare!(unary_7, "!doğru", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(BramaPrimative::Bool(true))))));
    test_compare!(unary_8, "!yanlış", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(BramaPrimative::Bool(false))))));
    
    test_compare!(unary_9, "+[]", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_10, "++100", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_11, "--100", Err(("Invalid unary operation", 0, 0)));
    test_compare!(unary_12, "--true", Err(("Invalid unary operation", 0, 0)));

    test_compare!(unary_13, "++data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol(String::from("data"))))));
    test_compare!(unary_14, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol(String::from("data"))))));
    test_compare!(unary_15, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol(String::from("data"))))));
}