extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::parser::*;
    use crate::karamellib::types::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::value::KaramelPrimative;
    use crate::karamellib::compiler::ast::*;
    use crate::karamellib::error::*;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_success {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser   = Parser::new($text);
                let parse_result = parser.parse();
                match parse_result {
                    Ok(_) => {
                        let syntax = SyntaxParser::new(parser.tokens().to_vec());
                        assert_eq!(syntax.parse(), $result);
                    },
                    Err(error) => {
                        let _err: Result<Rc<KaramelAstType>, KaramelError> = Err(error);
                        assert_eq!(_err, $result);
                    }
                };
            }
        };
    }


    test_success!(integer_1, "1024", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024.0))))));
    test_success!(integer_2, "1024000", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1024000.0))))));
    test_success!(integer_3, "123", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0))))));
    test_success!(integer_4, "9223372036854775807", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(9223372036854775807.0))))));
    test_success!(integer_5, "0999999", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(999999.0))))));
    test_success!(integer_6, "1_234_567", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234567.0))))));
    test_success!(integer_7, "1_234_5_6_7", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234567.0))))));
    test_success!(integer_8, "1_234_5_6_7_", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1234567.0))))));

    test_success!(hex_1, "0x12", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(18.0))))));
    test_success!(hex_2, "0xffffff", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(16777215.0))))));
    test_success!(hex_3, "0x1FFFFFFFFFFFFF", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(9007199254740991.0))))));

    test_success!(oct_1, "062", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(50.0))))));
    test_success!(oct_2, "06211111111111", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(430723863113.0))))));

    test_success!(binary_1, "0b10000000000000000000000000000000", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2147483648.0))))));
    test_success!(binary_2, "0b01111111100000000000000000000000", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2139095040.0))))));
    test_success!(binary_3, "0b01", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))));
    test_success!(binary_4, "0B00000000011111111111111111111111", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(8388607.0))))));

    test_success!(test_1, "'merhaba dünya'", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new("merhaba dünya".to_string())))))));
    test_success!(test_2, "\"merhaba dünya\"", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new("merhaba dünya".to_string())))))));
    test_success!(test_3, "'merhaba dünya", Err(KaramelError {
        error_type: KaramelErrorType::MissingStringDeliminator,
        column: 14,
        line: 0
    }));
    test_success!(test_4, "\"merhaba dünya", Err(KaramelError {
        error_type: KaramelErrorType::MissingStringDeliminator,
        column: 14,
        line: 0
    }));
    test_success!(test_5, "merhaba dünya'", Err(KaramelError {
        error_type: KaramelErrorType::MissingStringDeliminator,
        column: 14,
        line: 0
    }));

    test_success!(bool_2, "doğru", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true))))));
    test_success!(bool_4, "yanlış", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(false))))));

    test_success!(dict_1, "{}", Ok(Rc::new(KaramelAstType::Dict(Vec::new()))));
    test_success!(dict_2, "{'1':1}", Ok(Rc::new(KaramelAstType::Dict([Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
    })].to_vec()))));
    test_success!(dict_3, r#"{
        '1' : 1, 
        '2': 2
}"#, Ok(Rc::new(KaramelAstType::Dict([Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
    }),
    Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("2".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
    })].to_vec()))));
    test_success!(dict_4, r#"{
        '1': 1, 
        '2': 2,
        '1': 2
}"#, Ok(Rc::new(KaramelAstType::Dict([Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))
    }),
    Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("2".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
    }),
    Rc::new(KaramelDictItem {
        key: Rc::new(KaramelPrimative::Text(Rc::new("1".to_string()))),
        value: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(2.0))))
    })].to_vec()))));
    

    test_success!(list_1, "[]", Ok(Rc::new(KaramelAstType::List(Vec::new()))));
    test_success!(list_2, "[1]", Ok(Rc::new(KaramelAstType::List([Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.0))))].to_vec()))));
    test_success!(list_3, "[doğru]", Ok(Rc::new(KaramelAstType::List([Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true))))].to_vec()))));
    test_success!(list_4, "[ ]", Ok(Rc::new(KaramelAstType::List(Vec::new()))));
    test_success!(list_5, "[123,doğru,'merhaba dünya',1.3]", Ok(Rc::new(KaramelAstType::List([Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(123.0)))), Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Bool(true)))), Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Text(Rc::new("merhaba dünya".to_string()))))), Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(1.3))))].to_vec()))));
    test_success!(list_6, "[[]]", Ok(Rc::new(KaramelAstType::List([Rc::new(KaramelAstType::List(Vec::new()))].to_vec()))));

    test_success!(list_7, "[123", Err(KaramelError {
        error_type: KaramelErrorType::ArrayNotClosed,
        column: 4,
        line: 0
    }));
    test_success!(list_8, "[data]", Ok(Rc::new(KaramelAstType::List([Rc::new(KaramelAstType::Symbol("data".to_string()))].to_vec()))));

    test_success!(empty_1, "boş", Ok(Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Empty)))));

    test_success!(symbol_1, "data", Ok(Rc::new(KaramelAstType::Symbol("data".to_string()))));
    test_success!(symbol_2, "data_test", Ok(Rc::new(KaramelAstType::Symbol("data_test".to_string()))));

    test_success!(parenthesis_1, "(10*10)", Ok(Rc::new(KaramelAstType::Binary {
        left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
        operator: KaramelOperatorType::Multiplication, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));
    test_success!(parenthesis_2, "(10+10)-10", Ok(Rc::new(KaramelAstType::Binary {
        left: Rc::new(KaramelAstType::Binary {
            left: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0)))), 
            operator: KaramelOperatorType::Addition, 
            right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
        }), 
        operator: KaramelOperatorType::Subtraction, 
        right: Rc::new(KaramelAstType::Primative(Rc::new(KaramelPrimative::Number(10.0))))
    })));
}