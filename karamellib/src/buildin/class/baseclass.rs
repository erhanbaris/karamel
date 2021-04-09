use crate::{buildin::{Class, ClassProperty}, compiler::function::{IndexerGetCall, IndexerSetCall, NativeCall}};
use crate::compiler::{BramaPrimative, function::{FunctionReference}};

use std::{sync::Arc};
use crate::compiler::GetType;
use crate::buildin::ClassConfig;

#[derive(Default)]
pub struct BasicInnerClass {
    config: ClassConfig
 }

 impl Class for BasicInnerClass {
    fn set_class_config(&mut self, config: ClassConfig) {
        self.config = config;
    }

    fn get_class_name(&self) -> String {
        self.config.name.clone()
    }

    fn has_element(&self, field: Arc<String>) -> bool {
        self.config.properties.get(&*field).is_some()
    }
    
    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty> {
        self.config.properties.iter()
    }

    fn get_element(&self, field: Arc<String>) -> Option<&ClassProperty> {
        self.config.properties.get(&*field)
    }
    
    fn property_count(&self) -> usize {
        self.config.properties.len()
    }

    fn add_method(&mut self, name: &str, function: NativeCall) {
        self.config.properties.insert(name.to_string(), ClassProperty::Function(FunctionReference::buildin_function(function, name.to_string())));
    }

    fn add_property(&mut self, name: &str, property: Arc<BramaPrimative>) {
        self.config.properties.insert(name.to_string(), ClassProperty::Field(property));
    }

    fn get_method(&self, name: &str) -> Option<Arc<FunctionReference>> {
        match self.config.properties.get(name) {
            Some(property) => match property {
                ClassProperty::Function(function) => Some(function.clone()),
                _ => None
            },
            _ => None
        }
    }

    fn get_property(&self, name: &str) -> Option<Arc<BramaPrimative>> {
        match self.config.properties.get(name) {
            Some(property) => match property {
                ClassProperty::Field(field) => Some(field.clone()),
                _ => None
            },
            _ => None
        }
    }

    fn set_getter(&mut self, indexer: IndexerGetCall) {
        self.config.indexer.get = Some(indexer);
    }

    fn get_getter(&self) -> Option<IndexerGetCall> {
        match &self.config.indexer.get {
            Some(indexer) => Some(*indexer),
            None => None
        }
    }

    fn set_setter(&mut self, indexer: IndexerSetCall) {
        self.config.indexer.set = Some(indexer);
    }

    fn get_setter(&self) -> Option<IndexerSetCall> {
        match &self.config.indexer.set {
            Some(indexer) => Some(*indexer),
            None => None
        }
    }
 }

impl BasicInnerClass {
    pub fn set_name(&mut self, name: &str) {
        if self.config.name.len() == 0 {
            self.config.name = name.to_string();
        }
    }
}

impl GetType for BasicInnerClass {
    fn get_type(&self) -> String {
        self.config.name.clone()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use crate::buildin::Class;
    use crate::compiler::{GetType, function::FunctionParameter};
    use crate::{buildin::class::baseclass::BasicInnerClass, compiler::{EMPTY_OBJECT, function::NativeCallResult}};
    use crate::compiler::{BramaPrimative, function::{FunctionReference}};

    pub fn tmp_func_1(_: FunctionParameter) -> NativeCallResult {        
        Ok(EMPTY_OBJECT)
    }

    pub fn tmp_func_2(_: FunctionParameter) -> NativeCallResult {        
        Ok(EMPTY_OBJECT)
    }

    #[test]
    fn test_opcode_class_1() {
        let opcode_class = BasicInnerClass::default();
        assert_eq!(opcode_class.get_type().len(), 0);
        assert_eq!(opcode_class.property_count(), 0);
    }

    #[test]
    fn test_opcode_class_2() {
        let mut opcode_class: BasicInnerClass = BasicInnerClass::default();
        opcode_class.set_name("test_class");

        assert_eq!(opcode_class.get_class_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 0);
        assert_eq!(opcode_class.get_type(), opcode_class.get_class_name());
        assert_eq!(opcode_class.get_type(), "test_class".to_string());
    }

    #[test]
    fn test_opcode_class_3() {
        let mut opcode_class: BasicInnerClass = BasicInnerClass::default();
        opcode_class.set_name("test_class");


        opcode_class.add_property("field_1", Arc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2", Arc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.get_class_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 2);

        let field_1 = opcode_class.get_property("field_1");
        let field_2 = opcode_class.get_property("field_2");

        assert_eq!(field_1.is_some(), true);
        assert_eq!(field_2.is_some(), true);
        
        assert_eq!(opcode_class.get_method("field_1").is_none(), true);
        assert_eq!(opcode_class.get_method("field_2").is_none(), true);

        match &*field_1.unwrap() {
            BramaPrimative::Number(number) => assert_eq!(1024.0, *number),
            _ => assert_eq!(false, true),
        };
        match &*field_2.unwrap() {
            BramaPrimative::Number(number) => assert_eq!(2048.0, *number),
            _ => assert_eq!(false, true),
        };

        assert_eq!(opcode_class.get_property("field_3").is_none(), true);
    }

    #[test]
    fn test_opcode_class_4() {
        let mut opcode_class: BasicInnerClass = BasicInnerClass::default();
        opcode_class.set_name("test_class");

        opcode_class.add_property("field_1", Arc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2", Arc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.get_class_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 2);
    }

    #[test]
    fn test_opcode_class_5() {
        let mut opcode_class: BasicInnerClass = BasicInnerClass::default();
        opcode_class.set_name("test_class");

        let mut function_1 = FunctionReference::default();
        let mut function_2 = FunctionReference::default();

        function_1.name = "function_1".to_string();
        function_2.name = "function_2".to_string();

        opcode_class.add_method("function_1", tmp_func_1);
        opcode_class.add_method("function_2", tmp_func_2);

        assert_eq!(opcode_class.get_class_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 2);

        let function_1 = opcode_class.get_method("function_1");
        let function_2 = opcode_class.get_method("function_2");

        assert_eq!(function_1.is_some(), true);
        assert_eq!(function_2.is_some(), true);

        assert_eq!(function_1.unwrap().name, "function_1".to_string());
        assert_eq!(function_2.unwrap().name, "function_2".to_string());
        
        assert_eq!(opcode_class.get_property("function_1").is_none(), true);
        assert_eq!(opcode_class.get_property("function_2").is_none(), true);
    }
}