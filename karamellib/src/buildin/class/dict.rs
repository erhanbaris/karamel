use std::rc::Rc;
use std::cell::RefCell;

use crate::{buildin::{Class, ClassConfig, ClassProperty}, compiler::{GetType, function::{FunctionParameter, IndexerGetCall, IndexerSetCall, NativeCall, NativeCallResult, FunctionFlag}}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::KaramelPrimative;
use crate::error::KaramelErrorType;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type, arc_bool, primative_list};

use crate::buildin::class::PRIMATIVE_CLASS_NAMES;

#[derive(Default)]
pub struct DictClass {
    base: BasicInnerClass
 }

 impl GetType for DictClass {
    fn get_type(&self) -> String {
        "sözlük".to_string()
    }
}

impl DictClass {
    pub fn new() -> Self {
        let mut dict = DictClass::default();
        dict.add_class_method("getir", get);
        dict.add_class_method("güncelle", set);
        dict.add_class_method("guncelle", set);
        dict.add_class_method("içeriyormu", contains);
        dict.add_class_method("iceriyormu", contains);
        dict.add_class_method("uzunluk", length);
        dict.add_class_method("ekle", add);
        dict.add_class_method("temizle", clear);
        dict.add_class_method("sil", remove);
        dict.add_class_method("anahtarlar", keys);

        PRIMATIVE_CLASS_NAMES.lock().unwrap().insert(dict.get_type());

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

    fn has_element(&self, source: Option<VmObject>, field: Rc<String>) -> bool {
        self.base.has_element(source, field)
    }
    
    fn properties(&self) -> std::collections::hash_map::Iter<'_, String, ClassProperty> {
        self.base.properties()
    }

    fn get_element(&self, source: Option<VmObject>, field: Rc<String>) -> Option<ClassProperty> {
        match self.base.get_element(source, field.clone()) {
            Some(property) => Some(property),
            None => match source {
                Some(object) => {
                    match &*object.deref() {
                        KaramelPrimative::Dict(dict) => match dict.borrow().get(&*field.clone()) {
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

    fn add_method(&mut self, name: &str, function: NativeCall, flags: FunctionFlag) {
        self.base.add_method(name, function, flags);
    }

    fn add_property(&mut self, name: &str, property: Rc<KaramelPrimative>) {
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


pub fn get_primative_class() -> Rc<dyn Class> {
    Rc::new(DictClass::new())
}

fn get(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("getir".to_string(), 1),
            1 => {
                let key = match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Text(yazi) => yazi.clone(),
                    _ => return expected_parameter_type!("anahtar".to_string(), "Yazı".to_string())
                };
                
                return match dict.borrow().get(&*key) {
                    Some(item) => Ok(*item),
                    _ => Ok(EMPTY_OBJECT)
                };
            },
            _ => n_parameter_expected!("getir".to_string(), 1, parameter.length())
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
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!(function_name.to_string(), 2),
            2 => {
                let mut iter = parameter.iter();
                let (position_object, item) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap());

                let position = match position_object {
                    KaramelPrimative::Text(text) => text.clone(),
                    _ => return expected_parameter_type!("anahtar".to_string(), "Yazı".to_string())
                };
                *dict.borrow_mut().entry((&position).to_string()).or_insert(*item) = *item;
                Ok(EMPTY_OBJECT)
            },
            _ => n_parameter_expected!(function_name.to_string(), 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        let length = dict.borrow().len() as f64;
        return Ok(VmObject::from(length));
    }
    Ok(EMPTY_OBJECT)
}

fn clear(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        dict.borrow_mut().clear();
    }
    Ok(EMPTY_OBJECT)
}

fn remove(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 => n_parameter_expected!("sil".to_string(), 1),
            1 => {
                let key = match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Text(text) => text.clone(),
                    _ => return expected_parameter_type!("anahtar".to_string(), "Yazı".to_string())
                };
                
                Ok(match dict.borrow_mut().remove(&key.to_string()) {
                    Some(_) => arc_bool!(true),
                    None => arc_bool!(false)
                })
            },
            _ => n_parameter_expected!("sil".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn keys(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        let mut keys = Vec::new();
        for key in dict.borrow().keys() {
            keys.push(VmObject::native_convert(KaramelPrimative::Text(Rc::new(key.to_string()))));
        }

        return Ok(VmObject::native_convert(primative_list!(keys)));
    }

    Ok(EMPTY_OBJECT)
}

fn contains(parameter: FunctionParameter) -> NativeCallResult {
    if let KaramelPrimative::Dict(dict) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("içeriyormu".to_string(), 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    KaramelPrimative::Text(search) =>  Ok(VmObject::from(dict.borrow().contains_key(&**search))),
                    _ => expected_parameter_type!("içeriyormu".to_string(), "Yazı".to_string())
                }
            },
            _ => n_parameter_expected!("içeriyormu".to_string(), 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

impl DictClass {
    pub fn add_static_method(&mut self, name: &str, function: NativeCall) {
        self.base.add_method(name, function, FunctionFlag::IN_CLASS & FunctionFlag::STATIC);
    }

    pub fn add_class_method(&mut self, name: &str, function: NativeCall) {
        self.base.add_method(name, function, FunctionFlag::IN_CLASS);
    }
}