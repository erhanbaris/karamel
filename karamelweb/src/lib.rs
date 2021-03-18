extern crate karamellib;

use std::cell::RefCell;

use karamellib::{compiler::BramaPrimative, logger::CollectorLogger};
use wasm_bindgen::prelude::*;
use js_sys::*;

#[wasm_bindgen]
pub fn execute_code(name: &str) -> Object {
    let response = js_sys::Object::new();

    /* JS referance object */
    let status_ref      = JsValue::from("status");
    let results_ref     = JsValue::from("results");
    let stdout_ref      = JsValue::from("stdout");
    let stderr_ref      = JsValue::from("stderr");
    static COLLECTOR: CollectorLogger = CollectorLogger { 
        stdout: RefCell::new(String::new()), 
        stderr: RefCell::new(String::new())
    };

    let result = karamellib::vm::executer::code_executer(&name.to_string(), &COLLECTOR);
    match result.compiled && result.executed {
        true => {
            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(true).as_ref()).unwrap();
            let results = Array::new();
            let stdouts = Array::new();
            match result.memory_output {
                Some(opjects) => {
                    for object in opjects.iter() {
                        match &*object.deref() {
                            BramaPrimative::Text(text) => results.push(&JsValue::from(&**text).into()),
                            BramaPrimative::Number(number) => results.push(&JsValue::from_f64(*number).into()),
                            BramaPrimative::Bool(bool) => results.push(&JsValue::from_bool(*bool).into()),
                            BramaPrimative::Empty => results.push(&JsValue::undefined().into()),
                            BramaPrimative::Atom(atom) => results.push(&JsValue::from_f64(*atom as f64).into()),
                            _ => 0
                        };
                    }
                },
                None=> ()
            };

            stdouts.push(&JsValue::from(COLLECTOR.stdout.borrow().clone()).into());

            Reflect::set(response.as_ref(), results_ref.as_ref(), results.as_ref()).unwrap();
            Reflect::set(response.as_ref(), stdout_ref.as_ref(),  stdouts.as_ref()).unwrap();
        },
        false => {
            let stderrs = Array::new();

            stderrs.push(&JsValue::from(COLLECTOR.stderr.borrow().clone()).into());

            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(false).as_ref()).unwrap();
            Reflect::set(response.as_ref(), stderr_ref.as_ref(),  stderrs.as_ref()).unwrap();
        }
    };

    COLLECTOR.stdout.borrow_mut().clear();
    COLLECTOR.stderr.borrow_mut().clear();

    response
}
