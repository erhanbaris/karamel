extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::compiler::ast::KaramelAstType;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::SyntaxParser;
    use crate::karamellib::types::*;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name() {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => (),
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(
        assignment_1,
        "erhan = 2020",
        Ok(Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2020.0))))
        }))
    );

    test_compare!(
        assignment_2,
        "erhan = ('erhan' * 2)",
        Ok(Rc::new(KaramelAstType::Assignment {
            variable: Rc::new(KaramelAstType::Symbol("erhan".to_string())),
            operator: KaramelOperatorType::Assign,
            expression: Rc::new(KaramelAstType::Binary {
                left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new("erhan".to_string()))))),
                operator: KaramelOperatorType::Multiplication,
                right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
            })
        }))
    );
}
