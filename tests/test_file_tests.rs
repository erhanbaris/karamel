extern crate tpd;

#[cfg(test)]
mod tests {
    use crate::tpd::parser::*;
    use crate::tpd::compiler::*;
    use crate::tpd::vm::*;
    use crate::tpd::syntax::*;

    use std::rc::Rc;

    #[test]
    fn test_files_executer() {
        use std::env;
        use std::fs;
        use std::path::Path;
        use termion::{color, style};

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
                                match executer::code_executer(&file_content) {
                                    Ok(_) => {
                                        if !is_pass {
                                            println!("{}{}# {} failed ({}){}", color::Fg(color::Red), style::Bold, __path, "Not failed", style::Reset);
                                            test_status = false;
                                        } else {
                                            println!("{}# {} passed{}",  color::Fg(color::Green), __path, style::Reset);
                                        }
                                    },
                                    Err(error) => {
                                        if is_pass {
                                            println!("{}{}# {} failed ({}){}", color::Fg(color::Red), style::Bold, __path, error, style::Reset);
                                            test_status = false;
                                        } else {
                                            println!("{}# {} passed{}",  color::Fg(color::Green), __path, style::Reset);
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