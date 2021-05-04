extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;

    #[warn(unused_macros)]
    macro_rules! test_success {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser   = Parser::new($text);
                let _parse_result = parser.parse();

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let parse_result = syntax.parse();

                match parse_result {
                    Ok(_) => {
                        assert_eq!(true, true);
                    },
                    Err(_) => {
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
                let _parse_result = parser.parse();

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let parse_result = syntax.parse();

                match parse_result {
                    Ok(_) => {
                        println!("'{}'", $text);
                        assert_eq!(false, true);
                    },
                    Err(_) => {
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
test_success!(indentation_7, r#"1024 * 123 ise:
    erhan=123
veya: 
    erhan=1234"#);
    test_success!(indentation_8, r#"1024 * 123 ise:
    erhan=123
veya: 
  erhan=1234
"#);
test_success!(indentation_9, r#"1024 * 123 ise:
    erhan=123
veya: 
    erhan=1234
erhan=22"#);
test_success!(indentation_10, r#"1024 * 123 ise:
 erhan=123
veya: 
 erhan=1234
erhan=22"#);
test_success!(indentation_11, r#"1024 * 123 ise:
    erhan=123
    doğru ise:
        erhan=123
veya: 
    erhan=1234
erhan=22"#);

test_success!(indentation_12, r#"
1024 * 123 ise:
    erhan=123

    doğru ise:
        io::print('merhaba dünya')
        erhan=123
veya:
    erhan=1234
erhan=22"#);
}