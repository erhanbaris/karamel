extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::types::*;
    use crate::tpd::syntax::*;
    use crate::tpd::compiler::*;

    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! memory_check {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut converted_memory = Vec::new();
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

                opcode_compiler.prepare_variable_store(&syntax_result.unwrap(), &mut compiler_options);
                for object in compiler_options.storages[0].get_memory() {
                    converted_memory.push((*object.deref()).clone());
                }
                assert_eq!(converted_memory, $result);
            }
        }
    }

    memory_check!(memory_1, "10 + 10", vec![BramaPrimative::Number(10.0), BramaPrimative::Empty]);
    memory_check!(memory_2, "10 + 123", vec![BramaPrimative::Number(10.0), BramaPrimative::Number(123.0), BramaPrimative::Empty]);
    memory_check!(memory_3, "11 + 12 + 13", vec![BramaPrimative::Number(11.0), BramaPrimative::Number(12.0), BramaPrimative::Number(13.0), BramaPrimative::Empty, BramaPrimative::Empty]);
    memory_check!(memory_4, "11 + 12 + 13 + 14", vec![BramaPrimative::Number(11.0), BramaPrimative::Number(12.0), BramaPrimative::Number(13.0), BramaPrimative::Number(14.0), BramaPrimative::Empty, BramaPrimative::Empty, BramaPrimative::Empty]);
    memory_check!(memory_5, "'erhan' + 'barış'", vec![BramaPrimative::Text(Rc::new("erhan".to_string())), BramaPrimative::Text(Rc::new("barış".to_string())), BramaPrimative::Empty]);
    memory_check!(memory_6, "'erhan' + '-' + 'barış'", vec![BramaPrimative::Text(Rc::new("erhan".to_string())), BramaPrimative::Text(Rc::new("-".to_string())), BramaPrimative::Text(Rc::new("barış".to_string())), BramaPrimative::Empty, BramaPrimative::Empty]);
    memory_check!(memory_7, "doğru == yanlış", vec![BramaPrimative::Bool(true), BramaPrimative::Bool(false), BramaPrimative::Empty]);
}