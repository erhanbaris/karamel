extern crate karamellib;

use js_sys::*;
use karamellib::{
    compiler::KaramelPrimative,
    vm::executer::{ExecutionParameters, ExecutionSource},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn execute_code(name: &str) -> Object {
    let response = js_sys::Object::new();

    /* JS referance object */
    let status_ref = JsValue::from("status");
    let results_ref = JsValue::from("results");
    let stdout_ref = JsValue::from("stdout");
    let stderr_ref = JsValue::from("stderr");
    let opcode_dump_ref = JsValue::from("code_dump");
    let memory_dump_ref = JsValue::from("memory_dump");

    let parameters = ExecutionParameters {
        source: ExecutionSource::Code(name.to_string()),
        return_opcode: true,
        return_output: true,
        dump_opcode: true,
        dump_memory: true,
    };

    let result = karamellib::vm::executer::code_executer(parameters);
    match result.compiled && result.executed {
        true => {
            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(true).as_ref()).unwrap();
            let results = Array::new();
            let stdouts = Array::new();
            if let Some(opjects) = result.memory_output {
                for object in opjects.iter() {
                    match &*object.deref() {
                        KaramelPrimative::Text(text) => results.push(&JsValue::from(&**text)),
                        KaramelPrimative::Number(number) => results.push(&JsValue::from_f64(*number)),
                        KaramelPrimative::Bool(bool) => results.push(&JsValue::from_bool(*bool)),
                        KaramelPrimative::Empty => results.push(&JsValue::undefined()),
                        _ => 0,
                    };
                }
            };

            if let Some(stdout) = result.stdout {
                stdouts.push(&JsValue::from(stdout.borrow().clone()));
            };

            if let Some(memory_dump) = result.memory_dump {
                Reflect::set(response.as_ref(), memory_dump_ref.as_ref(), JsValue::from_str(&memory_dump).as_ref()).unwrap();
            }

            if let Some(opcode_dump) = result.opcode_dump {
                Reflect::set(response.as_ref(), opcode_dump_ref.as_ref(), JsValue::from_str(&opcode_dump).as_ref()).unwrap();
            }

            Reflect::set(response.as_ref(), results_ref.as_ref(), results.as_ref()).unwrap();
            Reflect::set(response.as_ref(), results_ref.as_ref(), results.as_ref()).unwrap();
            Reflect::set(response.as_ref(), stdout_ref.as_ref(), stdouts.as_ref()).unwrap();
        }
        false => {
            let stderrs = Array::new();
            let stdouts = Array::new();

            if let Some(stdout) = result.stdout {
                stdouts.push(&JsValue::from(stdout.borrow().clone()));
            };

            if let Some(stderr) = result.stderr {
                stderrs.push(&JsValue::from(stderr.borrow().clone()));
            };

            Reflect::set(response.as_ref(), status_ref.as_ref(), JsValue::from_bool(false).as_ref()).unwrap();
            Reflect::set(response.as_ref(), stdout_ref.as_ref(), stdouts.as_ref()).unwrap();
            Reflect::set(response.as_ref(), stderr_ref.as_ref(), stderrs.as_ref()).unwrap();
        }
    };

    response
}
