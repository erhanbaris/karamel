use std::{borrow::Borrow, fs::File};
use std::io::prelude::*;
use std::path::Path;
use std::fs::canonicalize;

use crate::compiler::KaramelCompilerContext;

pub fn read_file<T: Borrow<str>>(file_name: T) -> Result<String, String> {
    match File::open(file_name.borrow()) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            Ok(contents)
        },
        Err(error) => return Err(format!("Dosya okuma hatası oldu. Hata : {:?}", error))
    }
}

pub fn compute_path_and_read_file<T: Borrow<str>>(file_name: T, context: &KaramelCompilerContext) -> Result<String, String> {
    let path = Path::new(file_name.borrow());

    if path.exists() && path.is_file() {
        return read_file(file_name);
    }

    let script_path = Path::new(&context.script_path);
    let source_file_path = Path::new(file_name.borrow());
    
    match canonicalize(script_path.join(source_file_path)) {
        Ok(path) => read_file(path.to_str().unwrap()),
        Err(error) => Err(error.to_string())
    }
}