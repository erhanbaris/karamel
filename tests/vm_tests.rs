extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
    use crate::tpd::compiler::*;
    use crate::tpd::vm::*;

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
                let mut compiler_options = BramaCompilerOption::new();
                let ast = &syntax_result.unwrap();

                opcode_compiler.prepare_variable_store(ast, &mut compiler_options);
                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    vm::run_vm(&mut compiler_options);
                    assert_eq!(*compiler_options.storages[0].memory.last().unwrap(), $result);
                }
            }
        };
    }

    test_last_memory!(vm_1, "10 + 10", VmObjectType::Integer(20));
    test_last_memory!(vm_2, "10 + 20 + 30", VmObjectType::Integer(60));
    test_last_memory!(vm_3, "'erhan' + 'barış'", VmObjectType::Text("erhanbarış".to_string()));
    test_last_memory!(vm_4, "'erhan' + 10", VmObjectType::Text("erhan10".to_string()));
    test_last_memory!(vm_5, "123.456 + 123.456", VmObjectType::Double(246.912));
    test_last_memory!(vm_6, "123 + 123.456", VmObjectType::Double(246.45600000000002));
    test_last_memory!(vm_7, "123.456 + 123", VmObjectType::Double(246.45600000000002));
    test_last_memory!(vm_8, "'erhan' + 10.1", VmObjectType::Text("erhan10.1".to_string()));
    test_last_memory!(vm_9, "'erhan' + doğru", VmObjectType::Text("erhandoğru".to_string()));
    test_last_memory!(vm_10, "'erhan' + false", VmObjectType::Text("erhanyanlış".to_string()));
    test_last_memory!(vm_11, "10 - 10", VmObjectType::Integer(0));
    test_last_memory!(vm_12, "110.0 - 10.0", VmObjectType::Double(100.0));
    test_last_memory!(vm_13, "110 - 10.0", VmObjectType::Double(100.0));
    test_last_memory!(vm_14, "110.0 - 10", VmObjectType::Double(100.0));
    test_last_memory!(vm_15, "'data' - 10", VmObjectType::Empty);
    test_last_memory!(vm_16, "(doğru ve doğru)", VmObjectType::Bool(true));
    test_last_memory!(vm_17, "(doğru ve yanlış)", VmObjectType::Bool(false));
    test_last_memory!(vm_18, "(doğru == yanlış)", VmObjectType::Bool(false));
    test_last_memory!(vm_19, "(doğru == doğru)", VmObjectType::Bool(true));
    test_last_memory!(vm_20, "100 == 100.0", VmObjectType::Bool(true));
    test_last_memory!(vm_21, "'erhan' == 'erhan'", VmObjectType::Bool(true));
    test_last_memory!(vm_22, "100 == 110.0", VmObjectType::Bool(false));
    test_last_memory!(vm_23, "10 + 20 == 40.0 - 10", VmObjectType::Bool(true));
    test_last_memory!(vm_24, "(doğru != yanlış)", VmObjectType::Bool(true));
    test_last_memory!(vm_25, "(doğru != doğru)", VmObjectType::Bool(false));
    test_last_memory!(vm_26, "100 != 100.0", VmObjectType::Bool(false));
    test_last_memory!(vm_27, "'erhan' != 'erhan'", VmObjectType::Bool(false));
    test_last_memory!(vm_28, "100 != 110.0", VmObjectType::Bool(true));
    test_last_memory!(vm_29, "10 + 20 != 40.0 - 10", VmObjectType::Bool(false));
    test_last_memory!(vm_30, "100 > 110.0", VmObjectType::Bool(false));
    test_last_memory!(vm_31, "100 < 110.0", VmObjectType::Bool(true));
    test_last_memory!(vm_32, "100 >= 110.0", VmObjectType::Bool(false));
    test_last_memory!(vm_33, "100 <= 110.0", VmObjectType::Bool(true));
    test_last_memory!(vm_34, "110 >= 110.0", VmObjectType::Bool(true));
    test_last_memory!(vm_35, "110 <= 110.0", VmObjectType::Bool(true));
    test_last_memory!(vm_36, "'erhan' * 2", VmObjectType::Text("erhanerhan".to_string()));
    test_last_memory!(vm_37, "2 * 2", VmObjectType::Integer(4));
    test_last_memory!(vm_38, "2.0 * 20", VmObjectType::Double(40.0));
    test_last_memory!(vm_39, "'erhan' * 2 == 'erhanbaris'", VmObjectType::Bool(false));
    test_last_memory!(vm_40, "'erhan' * 2 == 'erhanerhan'", VmObjectType::Bool(true));
    test_last_memory!(vm_41, "10/2", VmObjectType::Double(5.0));
    test_last_memory!(vm_42, "9/2", VmObjectType::Double(4.5));
    test_last_memory!(vm_43, "0/0", VmObjectType::Empty);
    test_last_memory!(vm_44, ":erhan eşittir :erhan", VmObjectType::Bool(true));
    test_last_memory!(vm_45, "10 küçüktür 100 ve 'erhan' eşitdeğildir 'barış' eşittir doğru", VmObjectType::Bool(true));
    test_last_memory!(vm_46, ":erhan eşitdeğildir :erhan", VmObjectType::Bool(false));
    test_last_memory!(vm_47, ":erhan eşittir :barış", VmObjectType::Bool(false));
    test_last_memory!(vm_48, ":erhan eşitdeğildir :barış", VmObjectType::Bool(true));
    test_last_memory!(vm_49, "1_024 * 1_024 == 1_048_576", VmObjectType::Bool(true));
    test_last_memory!(vm_50, "empty == empty", VmObjectType::Bool(true));
    test_last_memory!(vm_51, "empty != empty", VmObjectType::Bool(false));
    test_last_memory!(vm_52, "yok == yok", VmObjectType::Bool(true));
    test_last_memory!(vm_53, "yok != yok", VmObjectType::Bool(false));
    test_last_memory!(vm_54, ":ok - 1 == yok", VmObjectType::Bool(true));

}