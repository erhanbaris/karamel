extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::compiler::*;
    use crate::tpd::vm::*;
    use crate::tpd::syntax::*;

    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_success {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser   = Parser::new($text);
                let parse_result = parser.parse();

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                let parse_result = syntax.parse();

                match parse_result {
                    Ok(_) => {
                        assert_eq!(true, true);
                    },
                    Err((message, l, c)) => {
                        assert_eq!(false, true);
                    }
                };
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! test_fail {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser   = Parser::new($text);
                let parse_result = parser.parse();

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                let parse_result = syntax.parse();

                match parse_result {
                    Ok(_) => {
                        assert_eq!(false, true);
                    },
                    Err((message, l, c)) => {
                        assert_eq!(true, true);
                    }
                };
            }
        };
    }

    test_success!(indentation_1, "123");
    test_fail!(indentation_2, " 123");
    test_fail!(indentation_3, r#"
 123"#);
    test_success!(indentation_4, r#"
123"#);
    test_fail!(indentation_5, r#" 
123"#);
    test_fail!(indentation_6, r#" 
 123"#);
}