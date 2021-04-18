extern crate karamellib;

use karamellib::{compiler::BramaPrimative, vm::executer::{ExecutionParameters, ExecutionSource}};
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

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(name.to_string()),
        return_opcode: true,
        return_output: true
    };

    let result = karamellib::vm::executer::code_executer(parameters);
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
                            _ => 0
                        };
                    }
                },
                None=> ()
            };

            match result.stdout {
                Some(stdout) => { stdouts.push(&JsValue::from(stdout.borrow().clone()).into()); },
                _ => ()
            };
            
            Reflect::set(response.as_ref(), results_ref.as_ref(), results.as_ref()).unwrap();
            Reflect::set(response.as_ref(), stdout_ref.as_ref(),  stdouts.as_ref()).unwrap();
        },
        false => {
            let stderrs = Array::new();

            match result.stderr {
                Some(stderr) => { stderrs.push(&JsValue::from(stderr.borrow().clone()).into()); },
                _ => ()
            };

            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(false).as_ref()).unwrap();
            Reflect::set(response.as_ref(), stderr_ref.as_ref(),  stderrs.as_ref()).unwrap();
        }
    };

    response
}
