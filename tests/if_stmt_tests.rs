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

    test_compare!(if_1, r#"eğer 1024 * 123:
    erhan=123  
yada: 
  erhan=1234
"#, Ok(BramaAstType::FuncCall {
        names: ["print".to_string()].to_vec(),
        arguments: [].to_vec()
    }));
}