use crate::{buildin::{ClassProperty}, compiler::function::NativeCall};
use crate::compiler::{BramaPrimative, function::{FunctionReference}};

use std::{collections::HashMap, sync::Arc};
use crate::compiler::GetType;

#[derive(Default)]
pub struct OpcodeClass {
    name: String,
    storage_index: usize,
    properties: HashMap<String, ClassProperty>,
    is_readonly: bool
 }

impl OpcodeClass {
    pub fn set_name(&mut self, name: String) {
        if self.name.len() == 0 {
            self.name = name;
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_storage_index(&mut self, index: usize) {
        self.storage_index = index;
    }

    pub fn get_storage_index(&self) -> usize {
        self.storage_index
    }
    
    pub fn has_property(&self, name: String) -> bool {
        self.properties.contains_key(&name)
    }

    pub(crate) fn is_readonly(&self) -> bool {
        self.is_readonly
    }

    pub(crate) fn set_readonly(&mut self) {
        self.is_readonly = true;
    }
    
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }
    
    pub fn add_method(&mut self, name: String, function: NativeCall) {
        if self.is_readonly() { return }
        let function_ref = FunctionReference::buildin_function(function, name.to_string());
        self.properties.insert(name.to_string(), ClassProperty::Function(function_ref));
    }

    pub fn add_property(&mut self, name: String, property: Arc<BramaPrimative>) {
        if self.is_readonly() { return }
        self.properties.insert(name, ClassProperty::Field(property));
    }
    
    pub fn get_method(&self, name: &str) -> Option<Arc<FunctionReference>> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(_) => None,
                    ClassProperty::Function(function) => Some(function.clone())
                }
            },
            None => None
        }
    }

    pub fn get_property(&self, name: &str) -> Option<Arc<BramaPrimative>> {
        match self.properties.get(name) {
            Some(property) => {
                match property {
                    ClassProperty::Field(property) => Some(property.clone()),
                    ClassProperty::Function(_) => None
                }
            },
            None => None
        }
    }

    pub fn get_element(&self, name: &str) -> Option<&ClassProperty> {
        self.properties.get(name)
    }
}

impl GetType for OpcodeClass {
    fn get_type(&self) -> String {
        self.get_name()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use crate::compiler::{GetType, function::FunctionParameter};
    use crate::{buildin::opcode_class::OpcodeClass, compiler::{BramaCompiler, EMPTY_OBJECT, function::NativeCallResult}};
    use crate::compiler::{BramaPrimative, function::{FunctionReference}};

    pub fn tmp_func_1(_: FunctionParameter) -> NativeCallResult {        
        Ok(EMPTY_OBJECT)
    }

    pub fn tmp_func_2(_: FunctionParameter) -> NativeCallResult {        
        Ok(EMPTY_OBJECT)
    }

    #[test]
    fn test_opcode_class_1() {
        let opcode_class = OpcodeClass::default();
        assert_eq!(opcode_class.get_name().len(), 0);
        assert_eq!(opcode_class.property_count(), 0);
        assert_eq!(opcode_class.get_storage_index(), 0);
    }

    #[test]
    fn test_opcode_class_2() {
        let mut opcode_class: OpcodeClass = OpcodeClass::default();
        opcode_class.set_name("test_class".to_string());
        opcode_class.set_storage_index(1);

        assert_eq!(opcode_class.get_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 0);
        assert_eq!(opcode_class.get_storage_index(), 1);
        assert_eq!(opcode_class.get_type(), opcode_class.get_name());
        assert_eq!(opcode_class.get_type(), "test_class".to_string());
    }

    #[test]
    fn test_opcode_class_3() {
        let mut opcode_class: OpcodeClass = OpcodeClass::default();
        opcode_class.set_name("test_class".to_string());
        opcode_class.set_storage_index(10);


        opcode_class.add_property("field_1".to_string(), Arc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2".to_string(), Arc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.get_name(), "test_class".to_string());
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
        let mut opcode_class: OpcodeClass = OpcodeClass::default();
        opcode_class.set_name("test_class".to_string());
        opcode_class.set_storage_index(10);

        opcode_class.add_property("field_1".to_string(), Arc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2".to_string(), Arc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.name, "test_class".to_string());
        assert_eq!(opcode_class.properties.len(), 2);
        assert_eq!(opcode_class.storage_index, 10);
    }

    #[test]
    fn test_opcode_class_5() {
        let mut opcode_class: OpcodeClass = OpcodeClass::default();
        opcode_class.set_name("test_class".to_string());
        opcode_class.set_storage_index(2);

        let mut function_1 = FunctionReference::default();
        let mut function_2 = FunctionReference::default();

        function_1.name = "function_1".to_string();
        function_2.name = "function_2".to_string();

        opcode_class.add_method("function_1".to_string(), tmp_func_1);
        opcode_class.add_method("function_2".to_string(), tmp_func_2);

        assert_eq!(opcode_class.get_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 2);
        assert_eq!(opcode_class.get_storage_index(), 2);

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