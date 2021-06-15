extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::parser::*;
    use crate::karamellib::compiler::*;
    use crate::karamellib::vm::*;
    use crate::karamellib::syntax::*;

    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_last_memory {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert!(false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert!(false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: BramaCompiler = BramaCompiler::new();
                let ast = &syntax_result.unwrap();

                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    if unsafe { interpreter::run_vm(&mut compiler_options).is_ok() } {
                        let memory = compiler_options.storages[0].get_stack().first().unwrap().deref_clean();
                        assert_eq!(memory, $result);
                    } else {
                        assert!(false);
                    }
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
                    Err(_) => assert!(false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert!(false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: BramaCompiler = BramaCompiler::new();
                let ast = &syntax_result.unwrap();

                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    if unsafe { interpreter::run_vm(&mut compiler_options).is_ok() } {
                        match compiler_options.storages[0].get_variable_value(&$variable.to_string()) {
                            Some(ast) => assert_eq!(*ast, $result),
                            None => assert!(false)
                        }
                    } else {
                        assert!(false)
                    }
                }
            }
        };
    }

    #[warn(unused_macros)]
    macro_rules! execute {
        ($name:ident, $text:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert!(false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(parser.tokens().to_vec());
                let syntax_result = syntax.parse();
                match syntax_result {
                    Err(_) => assert!(false),
                    _ => ()
                };

                let opcode_compiler  = InterpreterCompiler {};
                let mut compiler_options: BramaCompiler = BramaCompiler::new();
                let ast = &syntax_result.unwrap();

                if let Ok(_) = opcode_compiler.compile(ast, &mut compiler_options) {
                    if unsafe { interpreter::run_vm(&mut compiler_options).is_ok() } {
                        assert!(true);
                        return;
                    }
                }
                assert!(false);
            }
        };
    }


    test_last_memory!(vm_1, "10 + 10", BramaPrimative::Number(20.0));
    test_last_memory!(vm_2, "10 + 20 + 30", BramaPrimative::Number(60.0));
    test_last_memory!(vm_3, "'erhan' + 'barış'", BramaPrimative::Text(Rc::new("erhanbarış".to_string())));
    //test_last_memory!(vm_4, "'erhan' + 10", BramaPrimative::Text(Rc::new("erhan10".to_string())));
    test_last_memory!(vm_5, "123.456 + 123.456", BramaPrimative::Number(246.912));
    test_last_memory!(vm_6, "123 + 123.456", BramaPrimative::Number(246.45600000000002));
    test_last_memory!(vm_7, "123.456 + 123", BramaPrimative::Number(246.45600000000002));
    //test_last_memory!(vm_8, "'erhan' + 10.1", BramaPrimative::Text(Rc::new("erhan10.1".to_string())));
    //test_last_memory!(vm_9, "'erhan' + doğru", BramaPrimative::Text(Rc::new("erhandoğru".to_string())));
    //test_last_memory!(vm_10, "'erhan' + false", BramaPrimative::Text(Rc::new("erhanyanlış".to_string())));
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
    test_last_memory!(vm_45, "10 küçüktür 100 ve 'erhan' eşitdeğildir 'barış' eşittir doğru", BramaPrimative::Bool(true));
    test_last_memory!(vm_49, "1_024 * 1_024 == 1_048_576", BramaPrimative::Bool(true));
    test_last_memory!(vm_50, "empty == empty", BramaPrimative::Bool(true));
    test_last_memory!(vm_51, "empty != empty", BramaPrimative::Bool(false));
    test_last_memory!(vm_52, "yok == yok", BramaPrimative::Bool(true));
    test_last_memory!(vm_53, "yok != yok", BramaPrimative::Bool(false));
    test_last_memory!(vm_55, "test_1 == test_2", BramaPrimative::Bool(true));
    test_variable_value!(vm_56, "text", "text = 1024", BramaPrimative::Number(1024.0));
    test_variable_value!(vm_57, "result", r#"text = 1024
result = text *2"#, BramaPrimative::Number(2048.0));
    test_variable_value!(vm_58, "full_text", r#"text_1 = 'erhan'
text_2 = 'baris'
full_text = text_1 + ' ' + text_2"#, BramaPrimative::Text(Rc::new("erhan baris".to_string())));
    test_variable_value!(vm_59, "erhan", r#"erhan=100
++erhan
++erhan
++erhan"#, BramaPrimative::Number(103.0));
    test_variable_value!(vm_60, "erhan", r#"erhan=100
--erhan
--erhan
--erhan"#, BramaPrimative::Number(97.0));
    test_variable_value!(vm_61, "erhan", r#"erhan=doğru
erhan=!erhan"#, BramaPrimative::Bool(false));
    test_variable_value!(vm_62, "erhan", r#"erhan=yanlış
erhan=!erhan"#, BramaPrimative::Bool(true));
    test_variable_value!(vm_63, "erhan", r#"erhan=!yanlış"#, BramaPrimative::Bool(true));
    test_variable_value!(vm_64, "erhan", r#"erhan=!doğru"#, BramaPrimative::Bool(false));
    test_variable_value!(vm_65, "erhan", r#"erhan=!-100"#, BramaPrimative::Bool(true));
    test_variable_value!(vm_66, "erhan", r#"erhan=1
barış=erhan++"#, BramaPrimative::Number(2.0));
    test_variable_value!(vm_67, "barış", r#"erhan=1
barış=erhan++"#, BramaPrimative::Number(1.0));
    test_variable_value!(vm_68, "erhan", r#"erhan=1
erhan+=10"#, BramaPrimative::Number(11.0));
    test_variable_value!(vm_69, "erhan", r#"erhan=11
erhan-=1"#, BramaPrimative::Number(10.0));
    test_variable_value!(vm_70, "erhan", r#"erhan=10
erhan/=2"#, BramaPrimative::Number(5.0));
    test_variable_value!(vm_71, "erhan", r#"erhan=5
erhan*=2"#, BramaPrimative::Number(10.0));
    test_variable_value!(vm_72, "erhan", r#"erhan=9-3"#, BramaPrimative::Number(6.0));
    test_variable_value!(vm_73, "erhan", r#"erhan=9/3"#, BramaPrimative::Number(3.0));
    test_variable_value!(vm_74, "erhan", r#"
erhan=1
erhan == 1 ise:
    erhan=2"#, BramaPrimative::Number(2.0));
    test_variable_value!(vm_75, "erhan", r#"
erhan=1
erhan != 1 ise:
    erhan=2"#, BramaPrimative::Number(1.0));
    test_variable_value!(vm_76, "baris", r#"
erhan=1
baris=2
erhan > 0 ise:
 erhan=2
 baris=3"#, BramaPrimative::Number(3.0));
 test_variable_value!(vm_77, "erhan", r#"
erhan=1
doğru ise:
  erhan=2"#, BramaPrimative::Number(2.0));
  test_variable_value!(vm_78, "erhan", r#"
erhan=1
yanlış ise:
    erhan=2
veya:
   erhan=3"#, BramaPrimative::Number(3.0));
   test_variable_value!(vm_79, "erhan", r#"
veri = 'erhan'
veri != 'erhan' ise:
    erhan = "oldu"
    io::printline('Oldu')
veya veri ise:
    erhan = "olmadi"
    io::printline('1 == 1')"#, BramaPrimative::Text(Rc::new("olmadi".to_string())));

    execute!(vm_80, r#"
erhan=1
barış=1
hataayıklama::doğrula(erhan, barış)"#);

execute!(vm_81, r#"hataayıklama::doğrula([] == [])"#);
execute!(vm_82, r#"hataayıklama::doğrula([1] != [])"#);
execute!(vm_83, r#"hataayıklama::doğrula([0] != [1])"#);
execute!(vm_84, r#"hataayıklama::doğrula([1,2,3] == [1,2,3])"#);
execute!(vm_85, r#"
a = 10 * 2
b = 5 * 4
hataayıklama::doğrula(a, b)"#);
execute!(vm_86, r#"
a = 'erhan'
b = 'barış'
hataayıklama::doğrula(a != b)"#);
execute!(vm_87, r#"hataayıklama::doğrula(doğru ve doğru)"#);
execute!(vm_88, r#"hataayıklama::doğrula(doğru veya yanlış)"#);
execute!(vm_89, r#"
veri = 'erhan'
veri != 'erhan' ise:
    erhan = "oldu"
veya veri ise:
    erhan = "olmadi"
hataayıklama::doğrula(erhan, 'olmadi')
"#);
execute!(vm_90, r#"hataayıklama::doğrula([1,2,3,[4,5]], [1,2,3,[4,5]])"#);
execute!(vm_91, r#"
veri={
    'veri1' : '1', 
    'veri2' : 2
}

hataayıklama::doğrula(veri['veri1'], '1')
hataayıklama::doğrula(veri['veri2'], 2)
"#);
execute!(vm_92, r#"
veri1 = {
    'veri1' : '1', 
    'veri2' : 2
}

veri2 = {
    'veri1' : '1', 
    'veri2' : 2
}

hataayıklama::doğrula(veri1 == veri2)
"#);
execute!(vm_93, r#"
veri1 = {
    'veri1' : '1', 
    'veri2' : 2
}

veri2 = {
    'veri1' : '2', 
    'veri2' : 2
}

hataayıklama::doğrula(veri1 != veri2)
"#);
execute!(vm_94, r#"
fonk test:
    döndür 10
hataayıklama::doğrula(test() == 10)
"#);
execute!(vm_95, r#"
fonk test:
    döndür 10
hataayıklama::doğrula(test() * 10, 100)
"#);
execute!(vm_96, r#"
fonk test:
    döndür 10
hataayıklama::doğrula(test() + test(), 20)
"#);
execute!(vm_97, r#"
fonk test:
    döndür
hataayıklama::doğrula(test(), yok)
"#);
execute!(vm_98, r#"
fonk test:
    döndür yok
hataayıklama::doğrula(test(), yok)
"#);
execute!(vm_99, r#"
fonk test_1:
    döndür 'erhan'

fonk test_2:
    döndür test_1()
hataayıklama::doğrula(test_2(), 'erhan')
"#);
execute!(vm_100, r#"
fonk test_1:
    döndür 'erhan'
fonk test_2:
    döndür test_1()
hataayıklama::doğrula(test_2() + " barış", 'erhan barış')
"#);
execute!(vm_101, r#"
fonk test_2:
    fonk test_1:
        döndür 'erhan'
    döndür test_1()
hataayıklama::doğrula(test_2() + " barış", 'erhan barış')
"#);
execute!(vm_102, r#"
fonk test:
    fonk test_erhan:
        döndür 'erhan'

    fonk test_barış:
        döndür 'barış'

    döndür test_erhan() + " " + test_barış()
hataayıklama::doğrula(test(), 'erhan barış')
"#);
execute!(vm_103, r#"
fonk test(a, b):
    döndür a
data = test(123, 321)
hataayıklama::doğrula(123, data)
"#);
execute!(vm_104, r#"
my_list = {
    'ad': 'erhan',
    'soyad': 'barış',
    'doğum tarihi': 1985
}

fonk read_data(list, key):
    döndür list[key]

adı          = read_data(my_list, 'ad')
soyadı       = read_data(my_list, 'soyad')
doğum_tarihi = read_data(my_list, 'doğum tarihi')

hataayıklama::doğrula(adı,         'erhan')
hataayıklama::doğrula(soyadı,      'barış')
hataayıklama::doğrula(doğum_tarihi, 1985)
"#);
execute!(vm_105, r#"
fonk test(list):
    döndür list['ad']

data = test({
    'ad': 'erhan'
})
hataayıklama::doğrula("erhan", data)
"#);
execute!(vm_106, r#"
fonk test(a):
    fonk __test_1(a):
        fonk __test_2(a):
            döndür a
        döndür __test_2(a)
    döndür __test_1(a)
hataayıklama::doğrula(test("erhan"), 'erhan')
"#);
execute!(vm_107, r#"
fonk Fibonacci(n):
    n <= 1 ise:
        döndür n
    veya:
        döndür(Fibonacci(n-1) + Fibonacci(n-2))
hataayıklama::doğrula(Fibonacci(10), 55)
hataayıklama::doğrula(Fibonacci(20), 6765)
"#);
}