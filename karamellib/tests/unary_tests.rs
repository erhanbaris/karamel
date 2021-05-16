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

    test_compare!(unary_1, "+1024", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))));
    test_compare!(unary_2, "-1024", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-1024.0)))));
    test_compare!(unary_3, "+1024.0", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))));
    test_compare!(unary_4, "-1024.0", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(-1024.0)))));
    test_compare!(unary_5, "değil doğru", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))))));
    test_compare!(unary_6, "değil yanlış", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))))));
    test_compare!(unary_7, "değil doğru", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true)))))));
    test_compare!(unary_8, "değil yanlış", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(false)))))));
    
    test_compare!(unary_9, "+[]", Err(BramaError {
        error_type: BramaErrorType::UnaryWorksWithNumber,
        column: 1,
        line: 0
    }));
    test_compare!(unary_10, "++100", Err(BramaError {
        error_type: BramaErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));
    test_compare!(unary_11, "--100", Err(BramaError {
        error_type: BramaErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));
    test_compare!(unary_12, "--doğru", Err(BramaError {
        error_type: BramaErrorType::InvalidUnaryOperation,
        column: 2,
        line: 0
    }));

    test_compare!(unary_13, "++data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_14, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_15, "--data", Ok(BramaAstType::PrefixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_16, "data--", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Deccrement, Box::new(BramaAstType::Symbol("data".to_string())))));
    test_compare!(unary_17, "data++", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol("data".to_string())))));

    test_compare!(unary_18, "+ 1024", Ok(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1024.0)))));
    test_compare!(unary_19, "++data - 1", Ok(BramaAstType::Binary {
        left: Box::new(BramaAstType::PrefixUnary(BramaOperatorType::Increment, Box::new(BramaAstType::Symbol("data".to_string())))),
        operator: BramaOperatorType::Subtraction,
        right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(1.0))))
    }));
    //test_compare!(unary_19, "doğru değil", Ok(BramaAstType::SuffixUnary(BramaOperatorType::Not, Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Bool(true))))));
}