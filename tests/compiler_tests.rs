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

    test_first_memory!(memory_1, "10 + 10", vec![BramaPrimative::Number(10.0).convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_2, "10 + 123", vec![BramaPrimative::Number(10.0).convert(), BramaPrimative::Number(123.0).convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_3, "11 + 12 + 13", vec![BramaPrimative::Number(11.0).convert(), BramaPrimative::Number(12.0).convert(), BramaPrimative::Number(13.0).convert(), BramaPrimative::Empty.convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_4, "11 + 12 + 13 + 14", vec![BramaPrimative::Number(11.0).convert(), BramaPrimative::Number(12.0).convert(), BramaPrimative::Number(13.0).convert(), BramaPrimative::Number(14.0).convert(), BramaPrimative::Empty.convert(), BramaPrimative::Empty.convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_5, "'erhan' + 'barış'", vec![BramaPrimative::Text("erhan").convert(), BramaPrimative::Text("barış").convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_6, "'erhan' + '-' + 'barış'", vec![BramaPrimative::Text("erhan").convert(), BramaPrimative::Text("-").convert(), BramaPrimative::Text("barış").convert(), BramaPrimative::Empty.convert(), BramaPrimative::Empty.convert()]);
    test_first_memory!(memory_7, "doğru == yanlış", vec![BramaPrimative::Bool(true).convert(), BramaPrimative::Bool(false).convert(), BramaPrimative::Empty.convert()]);
}