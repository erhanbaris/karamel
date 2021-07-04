extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::parser::*;
    use crate::karamellib::syntax::*;
    use crate::karamellib::compiler::*;
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

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: KaramelCompilerContext = KaramelCompilerContext::new();

                if let Ok(_) = opcode_compiler.compile(syntax_result.unwrap().clone(), &mut compiler_options) {
                    let memory = compiler_options.storages[0].get_memory();
                    for object in &*memory {
                        converted_memory.push((*object.deref()).clone());
                    }
                    assert_eq!(converted_memory, $result);
                }
                else {
                    assert!(false);
                }
            }
        }
    }

    memory_check!(memory_1, "10 + 10", vec![KaramelPrimative::Number(10.0)]);
    memory_check!(memory_2, "10 + 123", vec![KaramelPrimative::Number(10.0), KaramelPrimative::Number(123.0)]);
    memory_check!(memory_3, "11 + 12 + 13", vec![KaramelPrimative::Number(11.0), KaramelPrimative::Number(12.0), KaramelPrimative::Number(13.0)]);
    memory_check!(memory_4, "11 + 12 + 13 + 14", vec![KaramelPrimative::Number(11.0), KaramelPrimative::Number(12.0), KaramelPrimative::Number(13.0), KaramelPrimative::Number(14.0)]);
    memory_check!(memory_5, "'erhan' + 'barış'", vec![KaramelPrimative::Text(Rc::new("erhan".to_string())), KaramelPrimative::Text(Rc::new("barış".to_string()))]);
    memory_check!(memory_6, "'erhan' + '-' + 'barış'", vec![KaramelPrimative::Text(Rc::new("erhan".to_string())), KaramelPrimative::Text(Rc::new("-".to_string())), KaramelPrimative::Text(Rc::new("barış".to_string()))]);
    memory_check!(memory_7, "doğru == yanlış", vec![KaramelPrimative::Bool(true), KaramelPrimative::Bool(false)]);
}