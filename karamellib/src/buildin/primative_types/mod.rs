pub mod number;
pub mod text;

use crate::buildin::opcode_class::OpcodeClass;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PRIMATIVE_CLASSES: Vec<OpcodeClass> = {
        let mut m = Vec::new();
        m.push(number::get_primative_class());
        m.push(text::get_primative_class());
        m
    };
}

#[macro_export]
macro_rules! nativecall_test {
    ($name:ident, $function_name:ident, $query:expr, $result:expr) => {
        #[test]
        fn $name () {
            let result = $function_name(&mut BramaCompiler::new(), Some(Arc::new($query)), 0, 0);
            assert!(result.is_ok());
            let object = result.unwrap().deref();
            assert_eq!(*object, $result);
        }
    };
}