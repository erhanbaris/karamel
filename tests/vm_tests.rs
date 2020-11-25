extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::compiler::*;
    use crate::tpd::vm::*;
    use crate::tpd::syntax::*;

    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_last_memory {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: BramaCompilerOption<StaticStorage> = BramaCompilerOption::new();
                let ast = &syntax_result.unwrap();

                opcode_compiler.prepare_variable_store(ast, &mut compiler_options);
                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    interpreter::run_vm(&mut compiler_options);
                    assert_eq!(*compiler_options.storages[0].get_memory().last().unwrap().deref(), $result);
                }
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! test_variable_value {
        ($name:ident, $variable:expr, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: BramaCompilerOption<StaticStorage> = BramaCompilerOption::new();
                let ast = &syntax_result.unwrap();

                opcode_compiler.prepare_variable_store(ast, &mut compiler_options);
                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    interpreter::run_vm(&mut compiler_options);
                    match compiler_options.storages[0].get_variable_value(&$variable.to_string()) {
                        Some(ast) => assert_eq!(*ast, $result),
                        None => assert!(false)
                    }
                }
            }
        };
    }

    test_last_memory!(vm_1, "10 + 10", BramaPrimative::Number(20.0));
    test_last_memory!(vm_2, "10 + 20 + 30", BramaPrimative::Number(60.0));
    test_last_memory!(vm_3, "'erhan' + 'barış'", BramaPrimative::Text(Rc::new("erhanbarış".to_string())));
    test_last_memory!(vm_4, "'erhan' + 10", BramaPrimative::Text(Rc::new("erhan10".to_string())));
    test_last_memory!(vm_5, "123.456 + 123.456", BramaPrimative::Number(246.912));
    test_last_memory!(vm_6, "123 + 123.456", BramaPrimative::Number(246.45600000000002));
    test_last_memory!(vm_7, "123.456 + 123", BramaPrimative::Number(246.45600000000002));
    test_last_memory!(vm_8, "'erhan' + 10.1", BramaPrimative::Text(Rc::new("erhan10.1".to_string())));
    test_last_memory!(vm_9, "'erhan' + doğru", BramaPrimative::Text(Rc::new("erhandoğru".to_string())));
    test_last_memory!(vm_10, "'erhan' + false", BramaPrimative::Text(Rc::new("erhanyanlış".to_string())));
    test_last_memory!(vm_11, "10 - 10", BramaPrimative::Number(0.0));
    test_last_memory!(vm_12, "110.0 - 10.0", BramaPrimative::Number(100.0));
    test_last_memory!(vm_13, "110 - 10.0", BramaPrimative::Number(100.0));
    test_last_memory!(vm_14, "110.0 - 10", BramaPrimative::Number(100.0));
    test_last_memory!(vm_15, "'data' - 10", BramaPrimative::Empty);
    test_last_memory!(vm_16, "(doğru ve doğru)", BramaPrimative::Bool(true));
    test_last_memory!(vm_17, "(doğru ve yanlış)", BramaPrimative::Bool(false));
    test_last_memory!(vm_18, "(doğru == yanlış)", BramaPrimative::Bool(false));
    test_last_memory!(vm_19, "(doğru == doğru)", BramaPrimative::Bool(true));
    test_last_memory!(vm_20, "100 == 100.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_21, "'erhan' == 'erhan'", BramaPrimative::Bool(true));
    test_last_memory!(vm_22, "100 == 110.0", BramaPrimative::Bool(false));
    test_last_memory!(vm_23, "10 + 20 == 40.0 - 10", BramaPrimative::Bool(true));
    test_last_memory!(vm_24, "(doğru != yanlış)", BramaPrimative::Bool(true));
    test_last_memory!(vm_25, "(doğru != doğru)", BramaPrimative::Bool(false));
    test_last_memory!(vm_26, "100 != 100.0", BramaPrimative::Bool(false));
    test_last_memory!(vm_27, "'erhan' != 'erhan'", BramaPrimative::Bool(false));
    test_last_memory!(vm_28, "100 != 110.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_29, "10 + 20 != 40.0 - 10", BramaPrimative::Bool(false));
    test_last_memory!(vm_30, "100 > 110.0", BramaPrimative::Bool(false));
    test_last_memory!(vm_31, "100 < 110.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_32, "100 >= 110.0", BramaPrimative::Bool(false));
    test_last_memory!(vm_33, "100 <= 110.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_34, "110 >= 110.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_35, "110 <= 110.0", BramaPrimative::Bool(true));
    test_last_memory!(vm_36, "'erhan' * 2", BramaPrimative::Text(Rc::new("erhanerhan".to_string())));
    test_last_memory!(vm_37, "2 * 2", BramaPrimative::Number(4.0));
    test_last_memory!(vm_38, "2.0 * 20", BramaPrimative::Number(40.0));
    test_last_memory!(vm_39, "'erhan' * 2 == 'erhanbaris'", BramaPrimative::Bool(false));
    test_last_memory!(vm_40, "'erhan' * 2 == 'erhanerhan'", BramaPrimative::Bool(true));
    test_last_memory!(vm_41, "10/2", BramaPrimative::Number(5.0));
    test_last_memory!(vm_42, "9/2", BramaPrimative::Number(4.5));
    test_last_memory!(vm_43, "0/0", BramaPrimative::Empty);
    test_last_memory!(vm_44, ":erhan eşittir :erhan", BramaPrimative::Bool(true));
    test_last_memory!(vm_45, "10 küçüktür 100 ve 'erhan' eşitdeğildir 'barış' eşittir doğru", BramaPrimative::Bool(true));
    test_last_memory!(vm_46, ":erhan eşitdeğildir :erhan", BramaPrimative::Bool(false));
    test_last_memory!(vm_47, ":erhan eşittir :barış", BramaPrimative::Bool(false));
    test_last_memory!(vm_48, ":erhan eşitdeğildir :barış", BramaPrimative::Bool(true));
    test_last_memory!(vm_49, "1_024 * 1_024 == 1_048_576", BramaPrimative::Bool(true));
    test_last_memory!(vm_50, "empty == empty", BramaPrimative::Bool(true));
    test_last_memory!(vm_51, "empty != empty", BramaPrimative::Bool(false));
    test_last_memory!(vm_52, "yok == yok", BramaPrimative::Bool(true));
    test_last_memory!(vm_53, "yok != yok", BramaPrimative::Bool(false));
    test_last_memory!(vm_54, ":ok - 1 == yok", BramaPrimative::Bool(true));
    test_last_memory!(vm_55, "test_1 == test_2", BramaPrimative::Bool(true));
    test_variable_value!(vm_56, "text", "text = 1024", BramaPrimative::Number(1024.0));
    test_variable_value!(vm_57, "result", r#"text = 1024
result = text *2"#, BramaPrimative::Number(2048.0));
    test_variable_value!(vm_58, "full_text", r#"text_1 = 'erhan'
text_2 = 'baris'
full_text = text_1 + ' ' + text_2"#, BramaPrimative::Text(Rc::new("erhan baris".to_string())));
}