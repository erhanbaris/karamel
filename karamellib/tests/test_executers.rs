extern crate karamellib;

#[cfg(test)]
mod tests {
    use std::fs::Metadata;

    use crate::karamellib::vm::*;
    use crate::karamellib::{vm::executer::{ExecutionParameters, ExecutionSource}};

    enum ExecuterType {
        File,
        Module
    }

    impl ExecuterType {
        pub fn get_folder_name(&self) -> String {
            match &self {
                ExecuterType::File => "test_files".to_string(),
                ExecuterType::Module => "test_modules".to_string(),
            }
        }

        pub fn is_valid(&self, metadata: &Metadata) -> bool {
            match &self {
                ExecuterType::File => metadata.is_file(),
                ExecuterType::Module => metadata.is_dir(),
            }
        }
    }

    fn executer(executer_type: ExecuterType) {
        use std::env;
        use std::fs;
        use std::path::Path;
        use colored::*;
        use log;

        let mut test_status = true;
        let current_dir = env::current_dir().unwrap();
        let test_files = fs::read_dir(Path::new(&current_dir).join(executer_type.get_folder_name())).unwrap();

        for test_file in test_files {
            match test_file {
                Ok(path) => {
                    let is_valid = match path.metadata() {
                        Ok(metadata) => executer_type.is_valid(&metadata),
                        _ => false
                    };
            
                    if !is_valid { continue; }
                    let is_pass = path.path().file_name().unwrap().to_str().unwrap().starts_with("pass_");

                    match path.path().to_str() {
                        Some(path_str) => {
                            let parameters = ExecutionParameters {
                                source: ExecutionSource::File(path_str.to_string()),
                                return_opcode: false,
                                return_output: false
                            };

                            let result = executer::code_executer(parameters);
                            match result.compiled && result.executed {
                                true => {
                                    if !is_pass {
                                        log::error!("# {} failed ({})", path_str, "Not failed".red());
                                        test_status = false;
                                    }
                                },
                                false => {
                                    if is_pass {
                                        log::error!("# {} failed", path_str);
                                        test_status = false;
                                    }
                                }
                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            };
        }

        assert_eq!(true, test_status);
    }

    #[test]
    fn test_file_executer() {
        executer(ExecuterType::File);
    }


    #[test]
    fn test_module_executer() {
        executer(ExecuterType::Module);
    }
}