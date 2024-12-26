use crate::compiler::{function::FunctionReference, KaramelPrimative};
use crate::{
    buildin::{Class, ClassProperty},
    compiler::function::{FunctionFlag, IndexerGetCall, IndexerSetCall, NativeCall},
    types::VmObject,
};

use crate::buildin::ClassConfig;
use crate::compiler::GetType;
use std::rc::Rc;

#[derive(Default)]
pub struct BasicInnerClass {
    config: ClassConfig,
}

impl Class for BasicInnerClass {
    fn set_class_config(&mut self, config: ClassConfig) {
        self.config = config;
    }

    fn get_class_name(&self) -> String {
        self.config.name.clone()
    }

    fn has_element(&self, _: Option<VmObject>, field: Rc<String>) -> bool {
        self.config.properties.contains_key(&*field)
    }

    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty> {
        self.config.properties.iter()
    }

    fn get_element(&self, _: Option<VmObject>, field: Rc<String>) -> Option<ClassProperty> {
        self.config
            .properties
            .get(&*field)
            .map(|data| (*data).clone())
    }

    fn property_count(&self) -> usize {
        self.config.properties.len()
    }

    fn add_method(&mut self, name: &str, function: NativeCall, flags: FunctionFlag) {
        self.config
            .properties
            .insert(name.to_string(), ClassProperty::Function(FunctionReference::buildin_function(function, name.to_string(), flags)));
    }

    fn add_property(&mut self, name: &str, property: Rc<KaramelPrimative>) {
        self.config
            .properties
            .insert(name.to_string(), ClassProperty::Field(property));
    }

    fn set_getter(&mut self, indexer: IndexerGetCall) {
        self.config.indexer.get = Some(indexer);
    }

    fn get_getter(&self) -> Option<IndexerGetCall> {
        self.config.indexer.get.as_ref().map(|indexer| *indexer)
    }

    fn set_setter(&mut self, indexer: IndexerSetCall) {
        self.config.indexer.set = Some(indexer);
    }

    fn get_setter(&self) -> Option<IndexerSetCall> {
        self.config.indexer.set.as_ref().map(|indexer| *indexer)
    }
}

impl BasicInnerClass {
    pub fn set_name(&mut self, name: &str) {
        if self.config.name.is_empty() {
            self.config.name = name.to_string();
        }
    }

    pub fn add_static_method(&mut self, name: &str, function: NativeCall) {
        self.add_method(name, function, FunctionFlag::IN_CLASS & FunctionFlag::STATIC);
    }

    pub fn add_class_method(&mut self, name: &str, function: NativeCall) {
        self.add_method(name, function, FunctionFlag::IN_CLASS);
    }
}

impl GetType for BasicInnerClass {
    fn get_type(&self) -> String {
        self.config.name.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::buildin::class::baseclass::BasicInnerClass;
    use crate::buildin::Class;
    use crate::compiler::GetType;
    use crate::compiler::KaramelPrimative;
    use std::rc::Rc;

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
    fn test_opcode_class_4() {
        let mut opcode_class: BasicInnerClass = BasicInnerClass::default();
        opcode_class.set_name("test_class");

        opcode_class.add_property("field_1", Rc::new(KaramelPrimative::Number(1024.0)));
        opcode_class.add_property("field_2", Rc::new(KaramelPrimative::Number(2048.0)));

        assert_eq!(opcode_class.get_class_name(), "test_class".to_string());
        assert_eq!(opcode_class.property_count(), 2);
    }
}
