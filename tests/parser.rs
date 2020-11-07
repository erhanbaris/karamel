extern crate TPD;

#[cfg(test)]
mod tests {
    use crate::TPD::parser::*;
    use crate::TPD::types::*;

    #[warn(unused_macros)]
    macro_rules! test_number {
        ($name:ident, $type:ident, $text:expr, $result:expr) => {
            // The macro will expand into the contents of this block.
            #[test]
            fn $name () {
                let mut parser = Parser::new();
                parser.parse($text);
                let tokens = parser.tokens();

                assert_eq!(1, tokens.len());
                match &tokens[0].token_type {
                    BramaTokenType::$type(num) => assert_eq!(*num, $result),
                    _ => assert_eq!(true, false)
                }
            }
        };
    }

    #[test]
    fn get_text_1() {
        let mut parser = Parser::new();
        parser.parse("\"erhan barış\"");
        let tokens = parser.tokens();
        assert_eq!(1, tokens.len());
        for item in tokens.iter() {
            match &item.token_type {
                BramaTokenType::Text(text) => assert_eq!(text, "erhan barış"),
                _ => assert_eq!(true, false)
            }
        }
    }

    #[test]
    fn get_text_2() {
        let mut parser = Parser::new();
        parser.parse("\"erhan barış\"\"\"");
        let tokens = parser.tokens();
        assert_eq!(2, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, "erhan barış"),
            _ => assert_eq!(true, false)
        }
        match &tokens[1].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, ""),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn get_text_3() {
        let mut parser = Parser::new();
        parser.parse("'erhan barış'\"\"");
        let tokens = parser.tokens();
        assert_eq!(2, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, "erhan barış"),
            _ => assert_eq!(true, false)
        }
        match &tokens[1].token_type {
            BramaTokenType::Text(text) => assert_eq!(text, ""),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn keywords() {
        for (keyword, keyword_enum) in &KEYWORDS {
            let mut parser = Parser::new();
            parser.parse(&keyword);
            let tokens = parser.tokens();

            assert_eq!(1, tokens.len());
            match &tokens[0].token_type {
                BramaTokenType::Keyword(keyword) => assert_eq!(keyword_enum, keyword),
                _ => assert_eq!(true, false)
            }
        }

        let mut parser = Parser::new();
        parser.parse("_test_");
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Symbol(symbol) => assert_eq!("_test_", symbol),
            _ => assert_eq!(true, false)
        }

        let mut parser = Parser::new();
        parser.parse("$");
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Symbol(symbol) => assert_eq!("$", symbol),
            _ => assert_eq!(true, false)
        }

        let mut parser = Parser::new();
        parser.parse("$$erhan$$");
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Symbol(symbol) => assert_eq!("$$erhan$$", symbol),
            _ => assert_eq!(true, false)
        }
    }

    #[test]
    fn new_line() {
        let mut parser = Parser::new();
        parser.parse("\n");
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::NewLine => assert_eq!(true, true),
            _ => assert_eq!(true, false)
        }
    }

    test_number!(integer_1, Integer, "1024", 1024);
    test_number!(integer_2, Integer, " 1024000 ", 1024000);
    test_number!(integer_3, Integer, "123", 123);
    test_number!(integer_4, Integer, "9223372036854775807", 9223372036854775807);
    test_number!(integer_5, Integer, "0999999", 999999);

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
        let mut parser = Parser::new();
        parser.parse(" .1024000 ");
        let tokens = parser.tokens();

        assert_eq!(1, tokens.len());
        match &tokens[0].token_type {
            BramaTokenType::Double(num) => assert_eq!(0.1024 - *num < 1e-10, true),
            _ => assert_eq!(true, false)
        }
    }
    test_number!(double_3, Double, "099999.9", 99999.9);
    test_number!(double_4, Double, "123.4e+4", 1234000.0);

}