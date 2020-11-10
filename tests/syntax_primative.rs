extern crate TPD;

#[cfg(test)]
mod tests {
    use crate::TPD::parser::*;
    use crate::TPD::syntax::*;
    use crate::TPD::types::*;

    #[warn(unused_macros)]
    macro_rules! test_is_primative {
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


    test_is_primative!(integer_1, "1024");
    test_is_primative!(integer_2, "1024000");
    test_is_primative!(integer_3, "123");
    test_is_primative!(integer_4, "9223372036854775807");
    test_is_primative!(integer_5, "0999999");
    test_is_primative!(integer_6, "1_234_567");
    test_is_primative!(integer_7, "1_234_5_6_7");
    test_is_primative!(integer_8, "1_234_5_6_7_");

    test_is_primative!(hex_1, "0x12");
    test_is_primative!(hex_2, "0xffffff");
    test_is_primative!(hex_3, "0x1FFFFFFFFFFFFF");

    test_is_primative!(oct_1, "062");
    test_is_primative!(oct_2, "06211111111111");

    test_is_primative!(binary_1, "0b10000000000000000000000000000000");
    test_is_primative!(binary_2, "0b01111111100000000000000000000000");
    test_is_primative!(binary_3, "0b01");
    test_is_primative!(binary_4, "0B00000000011111111111111111111111");

    test_is_primative!(test_1, "'merhaba dünya'");
    test_is_primative!(test_2, "\"merhaba dünya\"");

    test_is_primative!(bool_1, "true");
    test_is_primative!(bool_2, "doğru");
    test_is_primative!(bool_3, "false");
    test_is_primative!(bool_4, "yanlış");

    test_is_primative!(atom_1, ":merhaba");
    test_is_primative!(atom_2, ":dünya");
    test_is_primative!(atom_3, ":_");
    test_is_primative!(atom_4, ":__1__");

    test_is_primative!(list_1, "[]");
    test_is_primative!(list_2, "[1]");
    test_is_primative!(list_3, "[doğru]");
    test_is_primative!(list_4, "[ ]");
    test_is_primative!(list_5, "[123,doğru,:erhan_barış,'merhaba dünya',1.3]");
    test_is_primative!(list_6, "[[]]");

}