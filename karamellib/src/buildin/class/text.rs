use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type};
use crate::primative_text;

use unicode_width::UnicodeWidthStr;
use std::{cell::RefCell, sync::Arc};


pub fn get_primative_class() -> BasicInnerClass {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("Yazı");
    
    opcode.add_method("uzunluk", length);
    opcode.add_method("küçükharf", lowercase);
    opcode.add_method("kucukharf", lowercase);
    opcode.add_method("büyükharf", uppercase);
    opcode.add_method("buyukharf", uppercase);
    opcode.add_method("içeriyormu", contains);
    opcode.add_method("iceriyormu", contains);
    opcode.add_method("satırlar", lines);
    opcode.add_method("satirlar", lines);
    opcode.add_method("parçala", split);
    opcode.add_method("parcala", split);
    opcode.add_method("ara", find);
    opcode.add_method("değiştir", replace);
    opcode.add_method("degistir", replace);
    opcode.add_method("kırp", trim);
    opcode.add_method("kirp", trim);
    opcode.add_method("sonukırp", end_trim);
    opcode.add_method("sonukirp", end_trim);
    opcode.add_method("başıkırp", start_trim);
    opcode.add_method("basikirp", start_trim);
    opcode.add_method("parçagetir", substring);
    opcode.add_method("parcagetir", substring);
    opcode
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(text.chars().count() as f64)));
    }
    Ok(EMPTY_OBJECT)
}

fn contains(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("içeriyormu", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  Ok(VmObject::native_convert(BramaPrimative::Bool(text.contains(&search[..])))),
                    _ => expected_parameter_type!("içeriyormu", "Yazı")
                }
            },
            _ => n_parameter_expected!("içeriyormu", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn lowercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        let text:String = text.chars()
        .map(|x| match x { 
            'I' => 'ı', 
            'İ' => 'i', 
            'Ü' => 'ü', 
            'Ğ' => 'ğ', 
            'Ş' => 'ş', 
            'Ç' => 'ç', 
            'Ö' => 'ö',
            _ => x
        }).collect();
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_lowercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn uppercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        let text:String = text.chars()
        .map(|x| match x { 
            'ı' => 'I', 
            'i' => 'İ', 
            'ü' => 'Ü', 
            'ğ' => 'Ğ', 
            'ş' => 'Ş', 
            'ç' => 'Ç', 
            'ö' => 'Ö',
            _ => x
        }).collect();
        return Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.to_uppercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn lines(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        let splits = text.lines().collect::<Vec<_>>();
        let mut lines = Vec::new();

        for line in splits.iter() {
            lines.push(Arc::new(BramaPrimative::Text(Arc::new(line.to_string()))));
        }
        return Ok(VmObject::native_convert(BramaPrimative::List(RefCell::new(lines))));
    }
    Ok(EMPTY_OBJECT)
}

fn split(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("parçala", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  {
                        let splits = text.split(&**search).collect::<Vec<_>>();
                        let mut lines = Vec::new();

                        for line in splits.iter() {
                            lines.push(Arc::new(BramaPrimative::Text(Arc::new(line.to_string()))));
                        }
                        return Ok(VmObject::native_convert(BramaPrimative::List(RefCell::new(lines))));
                    },
                    _ => expected_parameter_type!("parçala", "Yazı")
                }
            },
            _ => n_parameter_expected!("parçala", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn find(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("parçala", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  {
                        match text.find(&**search) {
                            Some(location) => Ok(VmObject::native_convert(BramaPrimative::Number(UnicodeWidthStr::width(&text[..location]) as f64))),
                            _ => Ok(EMPTY_OBJECT)
                        }
                    },
                    _ => expected_parameter_type!("parçala", "Yazı")
                }
            },
            _ => n_parameter_expected!("parçala", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn replace(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("değiştir", 2),
            2 => {
                let mut iter = parameter.iter();
                let (from, to) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap().deref());
                match (&*from, &*to) {
                    (BramaPrimative::Text(from), BramaPrimative::Text(to)) => Ok(VmObject::native_convert(BramaPrimative::Text(Arc::new(text.replace(&**from, &**to))))),
                    _ => expected_parameter_type!("değiştir", "Yazı")
                }
            },
            _ => n_parameter_expected!("değiştir", 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(primative_text!(text.trim())));
    }
    Ok(EMPTY_OBJECT)
}

fn end_trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(primative_text!(text.trim_end())));
    }
    Ok(EMPTY_OBJECT)
}

fn start_trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return Ok(VmObject::native_convert(primative_text!(text.trim_start())));
    }
    Ok(EMPTY_OBJECT)
}

fn substring(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("parçagetir", 2),
            2 => {
                let mut iter = parameter.iter();
                let (from, to) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap().deref());
                match (&*from, &*to) {
                    (BramaPrimative::Number(start), BramaPrimative::Number(end)) => {
                        let start_size = if (*start as i64) < 0 {
                            0 as usize
                        } else {
                            *start as usize
                        };

                        let end_size = if (*end as usize) < text.len() {
                            *end as usize
                        } else {
                            text.len() as usize
                        };
                        Ok(VmObject::native_convert(primative_text!(text.get(start_size..end_size).unwrap_or(""))))
                    },
                    _ => expected_parameter_type!("parçagetir", "Sayı")
                }
            },
            _ => n_parameter_expected!("parçagetir", 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test;
    use crate::nativecall_test_with_params;
    use crate::primative_text;


    nativecall_test!{test_length_1, length, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Number(7.0)}
    nativecall_test!{test_length_2, length, BramaPrimative::Text(Arc::new("".to_string())), BramaPrimative::Number(0.0)}
    nativecall_test!{test_length_3, length, BramaPrimative::Text(Arc::new("12345".to_string())), BramaPrimative::Number(5.0)}
    nativecall_test!{test_lowercase_1, lowercase, BramaPrimative::Text(Arc::new("TÜRKİYE".to_string())), BramaPrimative::Text(Arc::new("türkiye".to_string()))}
    nativecall_test!{test_lowercase_2, lowercase, BramaPrimative::Text(Arc::new("IĞÜİŞÇÖ".to_string())), BramaPrimative::Text(Arc::new("ığüişçö".to_string()))}
    nativecall_test!{test_lowercase_3, lowercase, BramaPrimative::Text(Arc::new("ERHAN".to_string())), BramaPrimative::Text(Arc::new("erhan".to_string()))}
    nativecall_test!{test_uppercase_1, uppercase, BramaPrimative::Text(Arc::new("türkiye".to_string())), BramaPrimative::Text(Arc::new("TÜRKİYE".to_string()))}
    nativecall_test!{test_uppercase_2, uppercase, BramaPrimative::Text(Arc::new("ığüişçö".to_string())), BramaPrimative::Text(Arc::new("IĞÜİŞÇÖ".to_string()))}
    nativecall_test!{test_uppercase_3, uppercase, BramaPrimative::Text(Arc::new("erhan".to_string())), BramaPrimative::Text(Arc::new("ERHAN".to_string()))}
    nativecall_test!{test_lines_1, lines, BramaPrimative::Text(Arc::new("erhan\r\n".to_string())), BramaPrimative::List(RefCell::new([Arc::new(BramaPrimative::Text(Arc::new("erhan".to_string())))].to_vec()))}
    nativecall_test!{test_lines_2, lines, BramaPrimative::Text(Arc::new("\r\n".to_string())), BramaPrimative::List(RefCell::new([Arc::new(BramaPrimative::Text(Arc::new("".to_string())))].to_vec()))}
    nativecall_test!{test_lines_3, lines, BramaPrimative::Text(Arc::new("erhan\r\nbarış".to_string())), BramaPrimative::List(RefCell::new([Arc::new(BramaPrimative::Text(Arc::new("erhan".to_string()))), Arc::new(BramaPrimative::Text(Arc::new("barış".to_string())))].to_vec()))}
    nativecall_test!{test_lines_4, lines, BramaPrimative::Text(Arc::new("erhan\r\nbarış\r\n".to_string())), BramaPrimative::List(RefCell::new([Arc::new(BramaPrimative::Text(Arc::new("erhan".to_string()))), Arc::new(BramaPrimative::Text(Arc::new("barış".to_string())))].to_vec()))}
    nativecall_test!{test_lines_5, lines, BramaPrimative::Text(Arc::new("erhan\r\nbarış\r\nkaramel".to_string())), BramaPrimative::List(RefCell::new([Arc::new(BramaPrimative::Text(Arc::new("erhan".to_string()))), Arc::new(BramaPrimative::Text(Arc::new("barış".to_string()))), Arc::new(BramaPrimative::Text(Arc::new("karamel".to_string())))].to_vec()))}
    
    nativecall_test_with_params!{test_split_1, split, primative_text!("erhan\r\n"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::List(RefCell::new([Arc::new(primative_text!("")), Arc::new(primative_text!("\r\n"))].to_vec()))}
    nativecall_test_with_params!{test_split_2, split, primative_text!("erhanbarışerhan"), [VmObject::native_convert(primative_text!("barış"))], BramaPrimative::List(RefCell::new([Arc::new(primative_text!("erhan")), Arc::new(primative_text!("erhan"))].to_vec()))}
    nativecall_test_with_params!{test_split_3, split, primative_text!("karamel"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::List(RefCell::new([Arc::new(primative_text!("karamel"))].to_vec()))}

    nativecall_test_with_params!{test_contains_1, contains, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::Bool(false)}
    nativecall_test_with_params!{test_contains_2, contains, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("merhaba"))], BramaPrimative::Bool(true)}
    nativecall_test_with_params!{test_contains_3, contains, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("dünya"))], BramaPrimative::Bool(true)}
    nativecall_test_with_params!{test_contains_4, contains, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!(" "))], BramaPrimative::Bool(true)}
    nativecall_test_with_params!{test_contains_5, contains, primative_text!("bir karamel miyav dedi minik fare kükredi"), [VmObject::native_convert(primative_text!("minik fare"))], BramaPrimative::Bool(true)}

    nativecall_test_with_params!{test_find_1, find, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::Empty}
    nativecall_test_with_params!{test_find_2, find, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("merhaba"))], BramaPrimative::Number(0.0)}
    nativecall_test_with_params!{test_find_3, find, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("dünya"))], BramaPrimative::Number(8.0)}
    nativecall_test_with_params!{test_find_4, find, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!(" "))], BramaPrimative::Number(7.0)}
    nativecall_test_with_params!{test_find_5, find, primative_text!("bir karamel miyav dedi minik fare kükredi"), [VmObject::native_convert(primative_text!("minik fare"))], BramaPrimative::Number(23.0)}
    nativecall_test_with_params!{test_find_6, find, primative_text!("kütüphaneciler haftası"), [VmObject::native_convert(primative_text!("hafta"))], BramaPrimative::Number(15.0)}
    nativecall_test_with_params!{test_find_7, find, primative_text!("şaşkın şakir Gündüz"), [VmObject::native_convert(primative_text!("Gündüz"))], BramaPrimative::Number(13.0)}

    nativecall_test_with_params!{test_replace_1, replace, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("dünya")), VmObject::native_convert(primative_text!("erhan"))], primative_text!("merhaba erhan")}
    nativecall_test_with_params!{test_replace_2, replace, primative_text!("merhaba dünya"), [VmObject::native_convert(primative_text!("test")), VmObject::native_convert(primative_text!("erhan"))], primative_text!("merhaba dünya")}
    
    nativecall_test!{test_trim_1, trim, primative_text!(" merhaba dünya "), primative_text!("merhaba dünya")}
    nativecall_test!{test_trim_2, trim, primative_text!("merhaba dünya "), primative_text!("merhaba dünya")}
    nativecall_test!{test_trim_3, trim, primative_text!(" merhaba dünya"), primative_text!("merhaba dünya")}

    nativecall_test!{test_start_trim_1, start_trim, primative_text!(" merhaba dünya "), primative_text!("merhaba dünya ")}
    nativecall_test!{test_start_trim_2, start_trim, primative_text!("merhaba dünya "), primative_text!("merhaba dünya ")}
    nativecall_test!{test_start_trim_3, start_trim, primative_text!(" merhaba dünya"), primative_text!("merhaba dünya")}

    nativecall_test!{test_end_trim_1, end_trim, primative_text!(" merhaba dünya "), primative_text!(" merhaba dünya")}
    nativecall_test!{test_end_trim_2, end_trim, primative_text!("merhaba dünya "), primative_text!("merhaba dünya")}
    nativecall_test!{test_end_trim_3, end_trim, primative_text!(" merhaba dünya"), primative_text!(" merhaba dünya")}

    nativecall_test_with_params!{test_substring_1, substring, primative_text!("merhaba dünya"), [VmObject::native_convert(BramaPrimative::Number(0.0)), VmObject::native_convert(BramaPrimative::Number(7.0))], primative_text!("merhaba")}
    nativecall_test_with_params!{test_substring_2, substring, primative_text!("merhaba dünya"), [VmObject::native_convert(BramaPrimative::Number(0.0)), VmObject::native_convert(BramaPrimative::Number(0.0))], primative_text!("")}
    nativecall_test_with_params!{test_substring_3, substring, primative_text!("merhaba dünya"), [VmObject::native_convert(BramaPrimative::Number(0.0)), VmObject::native_convert(BramaPrimative::Number(11110.0))], primative_text!("merhaba dünya")}
    nativecall_test_with_params!{test_substring_4, substring, primative_text!("merhaba dünya"), [VmObject::native_convert(BramaPrimative::Number(-100.0)), VmObject::native_convert(BramaPrimative::Number(11110.0))], primative_text!("merhaba dünya")}
    nativecall_test_with_params!{test_substring_5, substring, primative_text!("merhaba dünya"), [VmObject::native_convert(BramaPrimative::Number(8.0)), VmObject::native_convert(BramaPrimative::Number(14.0))], primative_text!("dünya")}

}