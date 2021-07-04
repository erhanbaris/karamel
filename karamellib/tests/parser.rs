extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;

    #[warn(unused_macros)]
    macro_rules! test_number {
        ($name:ident, $type:ident, $text:expr, $result:expr) => {
            // The macro will expand into the contents of this block.
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };
                let tokens = parser.tokens();

                assert_eq!(1, tokens.len());
                match &tokens[0].token_type {
                    KaramelTokenType::$type(num) => assert_eq!(*num, $result),
                    _ => assert_eq!(true, false)
                }
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! test_keyword {
        ($name:ident, $text:expr, $result:expr) => {
            // The macro will expand into the contents of this block.
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };
                let tokens = parser.tokens();
                assert_eq!(1, tokens.len());
                match &tokens[0].token_type {
                    KaramelTokenType::Keyword(keyword) => assert_eq!(*keyword, $result),
                    _ => assert_eq!(true, false)
                }
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! test_comment {
        ($name:ident, $text:expr) => {
            // The macro will expand into the contents of this block.
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };
                let tokens = parser.tokens();
                assert_eq!(0, tokens.len());
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! parse_failed {
        ($name:ident, $text:expr) => {
            // The macro will expand into the contents of this block.
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, true),
                    _ => assert_eq!(false, true)
                }
            }
        };
    }

    #[test]
    fn get_text_1() {
        let mut parser = Parser::new("\"erhan barış\"");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();
        assert_eq!(1, tokens.len());
        for item in tokens.iter() {
            match &item.token_type {
                KaramelTokenType::Text(text) => assert_eq!(**text, "erhan barış"),
                _ => assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn get_text_2() {
        let mut parser = Parser::new("\"erhan barış\"\"\"");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();
        assert_eq!(2, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::Text(text) => assert_eq!(**text, "erhan barış"),
            _ => assert_eq!(true, false)
        }
        match &tokens[1].token_type {
            KaramelTokenType::Text(text) => assert_eq!(**text, ""),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn get_text_3() {
        let mut parser = Parser::new("'erhan barış'\"\"");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();
        assert_eq!(2, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::Text(text) => assert_eq!(**text, "erhan barış"),
            _ => assert_eq!(true, false)
        }
        match &tokens[1].token_type {
            KaramelTokenType::Text(text) => assert_eq!(**text, ""),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn keywords() {
        let mut parser = Parser::new("_test_");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::Symbol(symbol) => assert_eq!("_test_", **symbol),
            _ => assert_eq!(true, false)
        }

        let mut parser = Parser::new("$");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::Symbol(symbol) => assert_eq!("$", **symbol),
            _ => assert_eq!(true, false)
        }

        let mut parser = Parser::new("$$erhan$$");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::Symbol(symbol) => assert_eq!("$$erhan$$", **symbol),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn new_line_1() {
        let mut parser = Parser::new("\n");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::NewLine(count) => assert_eq!(*count == 0, true),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn new_line_2() {
        let mut parser = Parser::new("\n     \n    \n   ");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(3, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::NewLine(count) => assert_eq!(*count == 5, true),
            _ => assert_eq!(true, false)
        }

        match &tokens[1].token_type {
            KaramelTokenType::NewLine(count) => assert_eq!(*count == 4, true),
            _ => assert_eq!(true, false)
        }

        match &tokens[2].token_type {
            KaramelTokenType::NewLine(count) => assert_eq!(*count == 3, true),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn whitespace() {
        let mut parser = Parser::new("     ");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            KaramelTokenType::WhiteSpace(count) => assert_eq!(*count == 5, true),
            _ => assert_eq!(true, false)
        }
    }


    parse_failed!(text_1, "'merhaba dünya");
    parse_failed!(text_2, "\"merhaba dünya");

    test_comment!(comment_1, "//");
    test_comment!(comment_2, "// merhaba dünya");
    test_comment!(comment_3, "/**/");
    test_comment!(comment_4, "/* merhaba dünya */");
    test_comment!(comment_5, "/* // */");
    parse_failed!(comment_6, "/*");

    parse_failed!(operator_1, "#");

    test_number!(integer_1, Integer, "1024", 1024);
    test_number!(integer_2, Integer, "1024000", 1024000);
    test_number!(integer_3, Integer, "123", 123);
    test_number!(integer_4, Integer, "9223372036854775807", 9223372036854775807);
    test_number!(integer_5, Integer, "0999999", 999999);
    test_number!(integer_6, Integer, "1_234_567", 1234567);
    test_number!(integer_7, Integer, "1_234_5_6_7", 1234567);
    test_number!(integer_8, Integer, "1_234_5_6_7_", 1234567);
    parse_failed!(integer_9, "1024erhan");

    test_number!(hex_1, Integer, "0x12", 18);
    test_number!(hex_2, Integer, "0xffffff", 16777215);
    test_number!(hex_3, Integer, "0x1FFFFFFFFFFFFF", 9007199254740991);

    test_number!(oct_1, Integer, "062", 50);
    test_number!(oct_2, Integer, "06211111111111", 430723863113);

    test_number!(binary_1, Integer, "0b10000000000000000000000000000000", 2147483648);
    test_number!(binary_2, Integer, "0b01111111100000000000000000000000", 2139095040);
    test_number!(binary_3, Integer, "0b01", 1);
    test_number!(binary_4, Integer, "0B00000000011111111111111111111111", 8388607);


    test_number!(double_1, Double, "1024.0", 1024.0);
    #[test]
    fn double_2() {
        let mut parser = Parser::new(" .1024000 ");
        match parser.parse() {
            Err(_) => assert_eq!(true, false),
            _ => ()
        };
        let tokens = parser.tokens();

        assert_eq!(3, tokens.len());
        match &tokens[1].token_type {
            KaramelTokenType::Double(num) => assert_eq!(0.1024 - *num < 1e-10, true),
            _ => assert_eq!(true, false)
        }
    }
    test_number!(double_3, Double, "099999.9", 99999.9);
    test_number!(double_4, Double, "123.4e+4", 1234000.0);
    test_number!(double_5, Double, "1_23.4e+4", 1234000.0);
    test_number!(double_6, Double, "1_23.4_e+4_", 1234000.0);
    test_number!(double_7, Double, "09__9_999.9_", 99999.9);

    test_keyword!(keyword_2, "doğru", KaramelKeywordType::True);
    test_keyword!(keyword_4, "yanlış", KaramelKeywordType::False);
}