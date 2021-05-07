extern crate karamellib;

#[cfg(test)]
mod tests {
    use crate::karamellib::vm::*;
    use crate::karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

    #[test]
    fn test_files_executer() {
        use std::env;
        use std::fs;
        use std::path::Path;
        use colored::*;
        use log;

        let mut test_status = true;
        let current_dir = env::current_dir().unwrap();
        let paths = fs::read_dir(Path::new(&current_dir).join("test_files")).unwrap();

        for path in paths {
            match path {
                Ok(_path) => {
                    let is_file = match _path.metadata() {
                        Ok(metadata) => metadata.is_file(),
                        _ => false
                    };
                    
                    let is_pass = _path.path().file_name().unwrap().to_str().unwrap().starts_with("pass_");

                    if is_file {
                        match _path.path().to_str() {
                            Some(__path) => {
                                let file_content = fs::read_to_string(__path).unwrap();
                                let parameters = ExecutionParameters {
                                    source: ExecutionSource::Code(file_content.to_string()),
                                    return_opcode: false,
                                    return_output: false
                                };

                                let result = executer::code_executer(parameters);
                                match result.compiled && result.executed {
                                    true => {
                                        if !is_pass {
                                            log::error!("# {} failed ({})", __path, "Not failed".red());
                                            test_status = false;
                                        }
                                    },
                                    false => {
                                        if is_pass {
                                            log::error!("# {} failed", __path);
                                            test_status = false;
                                        }
                                    }
                                }
                            },
                            _ => ()
                        }
                    }
                },
                _ => ()
            };
        }

        assert_eq!(true, test_status);
    }
}