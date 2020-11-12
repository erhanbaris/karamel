extern crate TPD;

#[cfg(test)]
mod tests {
    use crate::TPD::parser::*;
    use crate::TPD::syntax::*;
    use crate::TPD::types::*;

    #[warn(unused_macros)]
    macro_rules! test_success {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                parser.parse();
                let tokens = parser.tokens();
                let mut syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(primative::PrimativeParser::parse(&syntax).is_ok(), true);
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! test_fail {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                parser.parse();
                let tokens = parser.tokens();
                let mut syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(primative::PrimativeParser::parse(&syntax).is_ok(), false);
            }
        };
    }


    test_success!(integer_1, "1024");
    test_success!(integer_2, "1024000");
    test_success!(integer_3, "123");
    test_success!(integer_4, "9223372036854775807");
    test_success!(integer_5, "0999999");
    test_success!(integer_6, "1_234_567");
    test_success!(integer_7, "1_234_5_6_7");
    test_success!(integer_8, "1_234_5_6_7_");

    test_success!(hex_1, "0x12");
    test_success!(hex_2, "0xffffff");
    test_success!(hex_3, "0x1FFFFFFFFFFFFF");

    test_success!(oct_1, "062");
    test_success!(oct_2, "06211111111111");

    test_success!(binary_1, "0b10000000000000000000000000000000");
    test_success!(binary_2, "0b01111111100000000000000000000000");
    test_success!(binary_3, "0b01");
    test_success!(binary_4, "0B00000000011111111111111111111111");

    test_success!(test_1, "'merhaba dünya'");
    test_success!(test_2, "\"merhaba dünya\"");
    test_fail!(test_3, "'merhaba dünya");
    test_fail!(test_4, "\"merhaba dünya");
    test_fail!(test_5, "merhaba dünya'");

    test_success!(bool_1, "true");
    test_success!(bool_2, "doğru");
    test_success!(bool_3, "false");
    test_success!(bool_4, "yanlış");

    test_success!(atom_1, ":merhaba");
    test_success!(atom_2, ":dünya");
    test_success!(atom_3, ":_");
    test_success!(atom_4, ":__1__");

    test_success!(list_1, "[]");
    test_success!(list_2, "[1]");
    test_success!(list_3, "[doğru]");
    test_success!(list_4, "[ ]");
    test_success!(list_5, "[123,doğru,:erhan_barış,'merhaba dünya',1.3]");
    test_success!(list_6, "[[]]");

    test_fail!(list_7, "[123");
}