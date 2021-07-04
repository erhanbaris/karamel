use crate::compiler::KaramelPrimative;
use crate::{
    buildin::{Class, ClassProperty},
    compiler::function::{IndexerGetCall, IndexerSetCall, NativeCall, FunctionFlag},
    types::VmObject,
};

use crate::buildin::ClassConfig;
use crate::compiler::GetType;
use std::rc::Rc;

#[derive(Default)]
pub struct ProxyClass {
    config: ClassConfig
}

impl Class for ProxyClass {
    fn set_class_config(&mut self, _: ClassConfig) {}

    fn get_class_name(&self) -> String {
        "".to_string()
    }

    fn has_element(&self, source: Option<VmObject>, field: Rc<String>) -> bool {
        match source {
            Some(source_object) => match &*source_object.deref() {
                KaramelPrimative::Class(class) => class.has_element(source, field),
                _ => false
            },
            None => false,
        }
    }

    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty> {
        self.config.properties.iter()
    }

fn get_element(&self, source: Option<VmObject>, field: Rc<String>) -> Option<ClassProperty> {
        match source {
            Some(source_object) => match &*source_object.deref() {
                KaramelPrimative::Class(class) => class.get_element(source, field),
                _ => None
            },
            None => None,
        }
    }

    fn property_count(&self) -> usize {
        0
    }

    fn add_method(&mut self, _: &str, _: NativeCall, _: FunctionFlag) {}

    fn add_property(&mut self, _: &str, _: Rc<KaramelPrimative>) {}

    fn set_getter(&mut self, _: IndexerGetCall) {}

    fn get_getter(&self) -> Option<IndexerGetCall> {
        None
    }

    fn set_setter(&mut self, _: IndexerSetCall) {}

    fn get_setter(&self) -> Option<IndexerSetCall> {
        None
    }
}

pub fn get_primative_class() -> Rc<dyn Class> {
    Rc::new(ProxyClass { 
        config: ClassConfig::default()
    })
}

impl ProxyClass {
    pub fn set_name(&mut self, _: &str) {}
}

impl GetType for ProxyClass {
    fn get_type(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::buildin::class::baseclass::BasicInnerClass;
    use crate::buildin::Class;
    use crate::compiler::KaramelPrimative;
    use crate::compiler::GetType;
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
