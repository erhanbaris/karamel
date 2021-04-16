use std::sync::Arc;
use std::cell::RefCell;

use crate::{buildin::{Class, ClassConfig, ClassProperty}, compiler::{GetType, function::{FunctionParameter, IndexerGetCall, IndexerSetCall, NativeCall, NativeCallResult}}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type, arc_bool, primative_list};

#[derive(Default)]
pub struct DictClass {
    base: BasicInnerClass
 }

 impl GetType for DictClass {
    fn get_type(&self) -> String {
        "Sözlük".to_string()
    }
}

impl DictClass {
    pub fn new() -> Self {
        let mut dict = DictClass::default();
        dict.add_method("getir", get);
        dict.add_method("güncelle", set);
        dict.add_method("guncelle", set);
        dict.add_method("uzunluk", length);
        dict.add_method("ekle", add);
        dict.add_method("temizle", clear);
        dict.add_method("sil", remove);
        dict.add_method("anahtarlar", keys);
        dict
    }
}

 impl Class for DictClass {
    fn set_class_config(&mut self, config: ClassConfig) {
        self.base.set_class_config(config);
    }

    fn get_class_name(&self) -> String {
        self.get_type()
    }

    fn has_element(&self, source: Option<VmObject>, field: Arc<String>) -> bool {
        self.base.has_element(source, field)
    }
    
    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty> {
        self.base.properties()
    }

    fn get_element(&self, source: Option<VmObject>, field: Arc<String>) -> Option<ClassProperty> {
        match self.base.get_element(source, field.clone()) {
            Some(property) => Some(property),
            None => match source {
                Some(object) => {
                    match &*object.deref() {
                        BramaPrimative::Dict(dict) => match dict.borrow().get(&*field.clone()) {
                            Some(data) => Some(ClassProperty::Field(data.deref())),
                            None => None
                        },
                        _ => None
                    }
                },
                None => None
            }
        }
    }
    
    fn property_count(&self) -> usize {
        self.base.property_count()
    }

    fn add_method(&mut self, name: &str, function: NativeCall) {
        self.base.add_method(name, function);
    }

    fn add_property(&mut self, name: &str, property: Arc<BramaPrimative>) {
        self.base.add_property(name, property);
    }

    fn set_getter(&mut self, indexer: IndexerGetCall) {
        self.base.set_getter(indexer);
    }

    fn get_getter(&self) -> Option<IndexerGetCall> {
        self.base.get_getter()
    }

    fn set_setter(&mut self, indexer: IndexerSetCall) {
        self.base.set_setter(indexer);
    }

    fn get_setter(&self) -> Option<IndexerSetCall> {
        self.base.get_setter()
    }
 }


pub fn get_primative_class() -> Box<dyn Class + Send + Sync> {
    Box::new(DictClass::new())
}

fn get(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("getir", 1),
            1 => {
                let key = match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(yazi) => yazi.clone(),
                    _ => return expected_parameter_type!("anahtar", "Yazı")
                };
                
                return match dict.borrow().get(&*key) {
                    Some(item) => Ok(*item),
                    _ => Ok(EMPTY_OBJECT)
                };
            },
            _ => n_parameter_expected!("getir", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn set(parameter: FunctionParameter) -> NativeCallResult {
    insert_or_update(parameter, "güncelle")
}

fn add(parameter: FunctionParameter) -> NativeCallResult {
    insert_or_update(parameter, "ekle")
}

fn insert_or_update(parameter: FunctionParameter, function_name: &str) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!(function_name, 2),
            2 => {
                let mut iter = parameter.iter();
                let (position_object, item) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap());

                let position = match position_object {
                    BramaPrimative::Text(text) => text.clone(),
                    _ => return expected_parameter_type!("anahtar", "Yazı")
                };
                *dict.borrow_mut().entry((&position).to_string()).or_insert(*item) = *item;
                Ok(arc_bool!(true))
            },
            _ => n_parameter_expected!(function_name, 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        let length = dict.borrow().len() as f64;
        return Ok(VmObject::native_convert(BramaPrimative::Number(length)));
    }
    Ok(EMPTY_OBJECT)
}

fn clear(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        dict.borrow_mut().clear();
    }
    Ok(EMPTY_OBJECT)
}

fn remove(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 => n_parameter_expected!("sil", 1),
            1 => {
                let key = match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(text) => text.clone(),
                    _ => return expected_parameter_type!("anahtar", "Yazı")
                };
                
                Ok(match dict.borrow_mut().remove(&key.to_string()) {
                    Some(_) => arc_bool!(true),
                    None => arc_bool!(false)
                })
            },
            _ => n_parameter_expected!("sil", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn keys(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        let mut keys = Vec::new();
        for key in dict.borrow().keys() {
            keys.push(VmObject::native_convert(BramaPrimative::Text(Arc::new(key.to_string()))));
        }

        return Ok(VmObject::native_convert(primative_list!(keys)));
    }

    Ok(EMPTY_OBJECT)
}