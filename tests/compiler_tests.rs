extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
    use crate::tpd::compiler::*;

    #[warn(unused_macros)]
    macro_rules! test_first_memory {
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

                opcode_compiler.prepare_variable_store(&syntax_result.unwrap(), &mut compiler_options);
                println!("{:#?}", compiler_options.storages[0]);
                assert_eq!(compiler_options.storages[0].memory, $result);
            }
        };
    }

    test_first_memory!(memory_1, "10 + 10", vec![VmObjectType::Integer(10), VmObjectType::Empty]);
    test_first_memory!(memory_2, "10 + 123", vec![VmObjectType::Integer(10), VmObjectType::Integer(123), VmObjectType::Empty]);
    test_first_memory!(memory_3, "11 + 12 + 13", vec![VmObjectType::Integer(11), VmObjectType::Integer(12), VmObjectType::Integer(13), VmObjectType::Empty, VmObjectType::Empty]);
    test_first_memory!(memory_4, "11 + 12 + 13 + 14", vec![VmObjectType::Integer(11), VmObjectType::Integer(12), VmObjectType::Integer(13), VmObjectType::Integer(14), VmObjectType::Empty, VmObjectType::Empty, VmObjectType::Empty]);
    test_first_memory!(memory_5, "'erhan' + 'barış'", vec![VmObjectType::Text("erhan".to_string()), VmObjectType::Text("barış".to_string()), VmObjectType::Empty]);
    test_first_memory!(memory_6, "'erhan' + '-' + 'barış'", vec![VmObjectType::Text("erhan".to_string()),VmObjectType::Text("-".to_string()), VmObjectType::Text("barış".to_string()), VmObjectType::Empty, VmObjectType::Empty]);
    test_first_memory!(memory_7, "doğru == yanlış", vec![VmObjectType::Bool(true), VmObjectType::Bool(false), VmObjectType::Empty]);
}