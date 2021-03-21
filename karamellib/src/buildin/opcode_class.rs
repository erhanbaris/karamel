use crate::buildin::{ClassProperty};
use crate::compiler::{BramaPrimative, function::{FunctionReference}};

use std::collections::HashMap;
use crate::compiler::GetType;
use std::rc::Rc;

#[derive(Default)]
pub struct OpcodeClass {
    pub name: String,
    pub storage_index: usize,
    pub properties: HashMap<String, ClassProperty>,
    pub is_readonly: bool,
    pub is_buildin: bool,
    pub is_static: bool
 }

impl OpcodeClass {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn get_class_name(&self) -> String {
        self.name.to_string()
    }
    
    pub fn has_property(&self, name: String) -> bool {
        self.properties.contains_key(&name)
    }
    
    pub fn add_method(&mut self, name: String, function: Rc<FunctionReference>) {
        self.properties.insert(name, ClassProperty::Function(function));
    }

    pub fn add_property(&mut self, name: String, property: Rc<BramaPrimative>) {
        self.properties.insert(name, ClassProperty::Field(property));
    }
    
    pub fn get_method(&self, name: &str) -> Option<Rc<FunctionReference>> {
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

    pub fn get_property(&self, name: &str) -> Option<Rc<BramaPrimative>> {
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
}

impl GetType for OpcodeClass {
    fn get_type(&self) -> String {
        self.get_class_name()
    }
}

#[cfg(test)]
mod test {
    use crate::buildin::{Class};
    use crate::buildin::opcode_class::OpcodeClass;
    use crate::compiler::{BramaPrimative, function::{FunctionReference}};

    use crate::compiler::GetType;
    use std::rc::Rc;

    #[test]
    fn test_opcode_class_1() {
        let opcode_class = OpcodeClass::default();
        assert_eq!(opcode_class.name.len(), 0);
        assert_eq!(opcode_class.properties.len(), 0);
        assert_eq!(opcode_class.storage_index, 0);
    }

    #[test]
    fn test_opcode_class_2() {
        let opcode_class = OpcodeClass::new("test_class".to_string(), 1);
        assert_eq!(opcode_class.name, "test_class".to_string());
        assert_eq!(opcode_class.properties.len(), 0);
        assert_eq!(opcode_class.storage_index, 1);
        assert_eq!(opcode_class.get_type(), opcode_class.get_class_name());
        assert_eq!(opcode_class.get_type(), "test_class".to_string());
    }

    #[test]
    fn test_opcode_class_3() {
        let mut opcode_class: OpcodeClass = OpcodeClass::new("test_class".to_string(), 10);

        opcode_class.add_property("field_1".to_string(), Rc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2".to_string(), Rc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.name, "test_class".to_string());
        assert_eq!(opcode_class.properties.len(), 2);

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
        let mut opcode_class: OpcodeClass = OpcodeClass::new("test_class".to_string(), 10);

        opcode_class.add_property("field_1".to_string(), Rc::new(BramaPrimative::Number(1024.0)));
        opcode_class.add_property("field_2".to_string(), Rc::new(BramaPrimative::Number(2048.0)));

        assert_eq!(opcode_class.name, "test_class".to_string());
        assert_eq!(opcode_class.properties.len(), 2);
        assert_eq!(opcode_class.storage_index, 10);
    }

    #[test]
    fn test_opcode_class_5() {
        let mut opcode_class: OpcodeClass = OpcodeClass::new("test_class".to_string(), 2);
        let mut function_1 = FunctionReference::default();
        let mut function_2 = FunctionReference::default();

        function_1.name = "function_1".to_string();
        function_2.name = "function_2".to_string();

        opcode_class.add_method("function_1".to_string(), Rc::new(function_1));
        opcode_class.add_method("function_2".to_string(), Rc::new(function_2));

        assert_eq!(opcode_class.name, "test_class".to_string());
        assert_eq!(opcode_class.properties.len(), 2);
        assert_eq!(opcode_class.storage_index, 2);

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