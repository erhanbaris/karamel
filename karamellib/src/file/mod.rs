use std::{borrow::Borrow, fs::File};
use std::io::prelude::*;
use std::path::Path;
use std::fs::canonicalize;

use crate::compiler::KaramelCompilerContext;
use crate::constants::STARTUP_MODULE_NAME;

pub fn read_file<T: Borrow<str>>(file_name: T) -> Result<String, String> {
    match File::open(file_name.borrow()) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(contents)
        },
        Err(error) => return Err(format!("Dosya okuma hatası oldu.\r\nDosya: {}\r\nHata : {:?}", file_name.borrow(), error))
    }
}

pub fn compute_path_and_read_file<T: Borrow<str>>(file_name: T, context: &KaramelCompilerContext) -> Result<String, String> {
    let path = Path::new(file_name.borrow());

    if path.exists() && path.is_file() {
        return read_file(file_name);
    }

    let script_path = Path::new(&context.execution_path.path);
    let calculated_path = script_path.join(Path::new(file_name.borrow()));
    
    match canonicalize(&calculated_path) {
        Ok(path) => match path.exists() && path.is_file() {
            true => return read_file(path.to_str().unwrap()),
            false => (),
        },
        Err(_) => ()
    };

    match canonicalize(calculated_path.join(STARTUP_MODULE_NAME)) {
        Ok(path) => return read_file(path.to_str().unwrap()),
        Err(error) => Err(error.to_string())
    }
}