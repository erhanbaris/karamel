extern crate karamellib;

use karamellib::compiler::BramaPrimative;
use wasm_bindgen::prelude::*;
use js_sys::*;

#[wasm_bindgen]
pub fn execute_code(name: &str) -> Object {
    let response = js_sys::Object::new();

    /* JS referance object */
    let status_ref      = JsValue::from("status");
    let error_ref       = JsValue::from("error");
    let results_ref     = JsValue::from("results");

    let result = karamellib::vm::executer::code_executer(&name.to_string());
    match result {
        Ok(objects) => {
            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(true).as_ref()).unwrap();
            let results = Array::new();
            for object in objects.iter() {
                match &*object.deref() {
                    BramaPrimative::Text(text) => results.push(&JsValue::from(&**text).into()),
                    BramaPrimative::Number(number) => results.push(&JsValue::from_f64(*number).into()),
                    BramaPrimative::Bool(bool) => results.push(&JsValue::from_bool(*bool).into()),
                    BramaPrimative::Empty => results.push(&JsValue::undefined().into()),
                    BramaPrimative::Atom(atom) => results.push(&JsValue::from_f64(*atom as f64).into()),
                    _ => 0
                };
            };
            Reflect::set(response.as_ref(), results_ref.as_ref(), results.as_ref()).unwrap();
        },
        Err(error) => {
            Reflect::set(response.as_ref(), error_ref.as_ref(), JsValue::from(error).as_ref()).unwrap();
            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(false).as_ref()).unwrap();

        }
    };

    response
}
