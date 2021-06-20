use crate::{buildin::Class, compiler::function::{FunctionParameter, NativeCallResult}};
use crate::compiler::value::EMPTY_OBJECT;
use crate::buildin::class::baseclass::BasicInnerClass;
use crate::compiler::value::BramaPrimative;
use crate::types::VmObject;
use crate::{n_parameter_expected, expected_parameter_type, arc_bool, arc_empty, arc_text};
use crate::primative_text;

use unicode_width::UnicodeWidthStr;
use std::{cell::RefCell, rc::Rc};


pub fn get_primative_class() -> Rc<dyn Class> {
    let mut opcode = BasicInnerClass::default();
    opcode.set_name("yazı");
    
    opcode.add_class_method("uzunluk", length);
    opcode.add_class_method("harfleriküçült", lowercase);
    opcode.add_class_method("harflerikucult", lowercase);
    opcode.add_class_method("harfleribüyült", uppercase);
    opcode.add_class_method("harfleribuyult", uppercase);
    opcode.add_class_method("içeriyormu", contains);
    opcode.add_class_method("iceriyormu", contains);
    opcode.add_class_method("satırlar", lines);
    opcode.add_class_method("satirlar", lines);
    opcode.add_class_method("parçala", split);
    opcode.add_class_method("parcala", split);
    opcode.add_class_method("ara", find);
    opcode.add_class_method("değiştir", replace);
    opcode.add_class_method("degistir", replace);
    opcode.add_class_method("kırp", trim);
    opcode.add_class_method("kirp", trim);
    opcode.add_class_method("sonukırp", end_trim);
    opcode.add_class_method("sonukirp", end_trim);
    opcode.add_class_method("başıkırp", start_trim);
    opcode.add_class_method("basikirp", start_trim);
    opcode.add_class_method("parçagetir", substring);
    opcode.add_class_method("parcagetir", substring);
    opcode.add_class_method("sayı", number);
    opcode.add_class_method("sayi", number);
    opcode.set_getter(getter);
    opcode.set_setter(setter);
    Rc::new(opcode)
}


fn getter(source: VmObject, index: f64) -> NativeCallResult {
    let index = match index >= 0.0 {
        true => index as usize,
        false =>  return Ok(EMPTY_OBJECT)
    };
    
    if let BramaPrimative::Text(text) = &*source.deref() {

        return match text.chars().nth(index) {
            Some(item) => Ok(arc_text!(item.to_string())),
            _ => Ok(EMPTY_OBJECT)
        };
    }
    Ok(EMPTY_OBJECT)
}

fn setter(source: VmObject, index: f64, item: VmObject) -> NativeCallResult {
    let index = match index >= 0.0 {
        true => index as usize,
        false =>  return Ok(EMPTY_OBJECT)
    };

    if let BramaPrimative::Text(text) = &*source.deref() {
        return match text.chars().nth(index) {
            Some(old_char) => {
                match &*item.deref() {
                    BramaPrimative::Text(data) => {
                        if data.chars().count() != 1 {
                            return Ok(EMPTY_OBJECT);
                        }

                        let new_char = data.chars().nth(0).unwrap();
                        let mut real_index = 0;
                        let mut real_total = 0;

                        for (i, ch) in text.chars().enumerate() {
                            if i < index{
                                real_index += ch.len_utf8();
                            }
                            real_total += ch.len_utf8();
                        }
                        
                        /* full text size + new char size - old char size */
                        let mut new_string = String::with_capacity(real_total + data.len() - old_char.len_utf8());
                        new_string.push_str(&text[0..real_index]);
                        new_string.push(new_char);
                        new_string.push_str(&text[real_index+old_char.len_utf8()..]);

                        unsafe {
                            /* Update text with new one */
                            let text_ptr = text as *const Rc<String> as *mut Rc<String>;
                            *Rc::make_mut(&mut *text_ptr) = new_string;
                        }

                        Ok(EMPTY_OBJECT)
                    },
                    _ => Ok(EMPTY_OBJECT) //We cant use other types in text
                }
            },
            None => Ok(EMPTY_OBJECT)
        };
    }
    Ok(EMPTY_OBJECT)
}

fn length(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(BramaPrimative::Number(text.chars().count() as f64)));
    }
    Ok(EMPTY_OBJECT)
}

fn number(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return match text.parse::<f64>() {
            Ok(num) => Ok(VmObject::native_convert(BramaPrimative::Number(num))),
            _ => Ok(EMPTY_OBJECT),
        };
    }
    Ok(EMPTY_OBJECT)
}

fn contains(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("içeriyormu", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  Ok(VmObject::from(text.contains(&search[..]))),
                    _ => expected_parameter_type!("içeriyormu", "Yazı")
                }
            },
            _ => n_parameter_expected!("içeriyormu", 1, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn lowercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
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
        return Ok(VmObject::native_convert(BramaPrimative::Text(Rc::new(text.to_lowercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn uppercase(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
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
        return Ok(VmObject::native_convert(BramaPrimative::Text(Rc::new(text.to_uppercase()))));
    }
    Ok(EMPTY_OBJECT)
}

fn lines(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        let splits = text.lines().collect::<Vec<_>>();
        let mut lines = Vec::new();

        for line in splits.iter() {
            lines.push(VmObject::native_convert(BramaPrimative::Text(Rc::new(line.to_string()))));
        }
        return Ok(VmObject::native_convert(BramaPrimative::List(RefCell::new(lines))));
    }
    Ok(EMPTY_OBJECT)
}

fn split(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("parçala", 1),
            1 => {
                match &*parameter.iter().next().unwrap().deref() {
                    BramaPrimative::Text(search) =>  {
                        let splits = text.split(&**search).collect::<Vec<_>>();
                        let mut lines = Vec::new();

                        for line in splits.iter() {
                            lines.push(VmObject::native_convert(BramaPrimative::Text(Rc::new(line.to_string()))));
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
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
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
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return match parameter.length() {
            0 =>  n_parameter_expected!("değiştir", 2),
            2 => {
                let mut iter = parameter.iter();
                let (from, to) = (&*iter.next().unwrap().deref(), &*iter.next().unwrap().deref());
                match (&*from, &*to) {
                    (BramaPrimative::Text(from), BramaPrimative::Text(to)) => Ok(VmObject::native_convert(BramaPrimative::Text(Rc::new(text.replace(&**from, &**to))))),
                    _ => expected_parameter_type!("değiştir", "Yazı")
                }
            },
            _ => n_parameter_expected!("değiştir", 2, parameter.length())
        };
    }
    Ok(EMPTY_OBJECT)
}

fn trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(primative_text!(text.trim())));
    }
    Ok(EMPTY_OBJECT)
}

fn end_trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(primative_text!(text.trim_end())));
    }
    Ok(EMPTY_OBJECT)
}

fn start_trim(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
        return Ok(VmObject::native_convert(primative_text!(text.trim_start())));
    }
    Ok(EMPTY_OBJECT)
}

fn substring(parameter: FunctionParameter) -> NativeCallResult {
    if let BramaPrimative::Text(text) = &*parameter.source().unwrap().deref() {
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
    use std::rc::Rc;
    use crate::compiler::value::BramaPrimative;
    use super::*;

    use crate::nativecall_test;
    use crate::nativecall_test_with_params;
    use crate::primative_text;


    nativecall_test!{test_length_1, length, BramaPrimative::Text(Rc::new("TÜRKİYE".to_string())), BramaPrimative::Number(7.0)}
    nativecall_test!{test_length_2, length, BramaPrimative::Text(Rc::new("".to_string())), BramaPrimative::Number(0.0)}
    nativecall_test!{test_length_3, length, BramaPrimative::Text(Rc::new("12345".to_string())), BramaPrimative::Number(5.0)}
    nativecall_test!{test_lowercase_1, lowercase, BramaPrimative::Text(Rc::new("TÜRKİYE".to_string())), BramaPrimative::Text(Rc::new("türkiye".to_string()))}
    nativecall_test!{test_lowercase_2, lowercase, BramaPrimative::Text(Rc::new("IĞÜİŞÇÖ".to_string())), BramaPrimative::Text(Rc::new("ığüişçö".to_string()))}
    nativecall_test!{test_lowercase_3, lowercase, BramaPrimative::Text(Rc::new("ERHAN".to_string())), BramaPrimative::Text(Rc::new("erhan".to_string()))}
    nativecall_test!{test_uppercase_1, uppercase, BramaPrimative::Text(Rc::new("türkiye".to_string())), BramaPrimative::Text(Rc::new("TÜRKİYE".to_string()))}
    nativecall_test!{test_uppercase_2, uppercase, BramaPrimative::Text(Rc::new("ığüişçö".to_string())), BramaPrimative::Text(Rc::new("IĞÜİŞÇÖ".to_string()))}
    nativecall_test!{test_uppercase_3, uppercase, BramaPrimative::Text(Rc::new("erhan".to_string())), BramaPrimative::Text(Rc::new("ERHAN".to_string()))}
    nativecall_test!{test_lines_1, lines, BramaPrimative::Text(Rc::new("erhan\r\n".to_string())), BramaPrimative::List(RefCell::new([VmObject::native_convert(BramaPrimative::Text(Rc::new("erhan".to_string())))].to_vec()))}
    nativecall_test!{test_lines_2, lines, BramaPrimative::Text(Rc::new("\r\n".to_string())), BramaPrimative::List(RefCell::new([VmObject::native_convert(BramaPrimative::Text(Rc::new("".to_string())))].to_vec()))}
    nativecall_test!{test_lines_3, lines, BramaPrimative::Text(Rc::new("erhan\r\nbarış".to_string())), BramaPrimative::List(RefCell::new([VmObject::native_convert(BramaPrimative::Text(Rc::new("erhan".to_string()))), VmObject::native_convert(BramaPrimative::Text(Rc::new("barış".to_string())))].to_vec()))}
    nativecall_test!{test_lines_4, lines, BramaPrimative::Text(Rc::new("erhan\r\nbarış\r\n".to_string())), BramaPrimative::List(RefCell::new([VmObject::native_convert(BramaPrimative::Text(Rc::new("erhan".to_string()))), VmObject::native_convert(BramaPrimative::Text(Rc::new("barış".to_string())))].to_vec()))}
    nativecall_test!{test_lines_5, lines, BramaPrimative::Text(Rc::new("erhan\r\nbarış\r\nkaramel".to_string())), BramaPrimative::List(RefCell::new([VmObject::native_convert(BramaPrimative::Text(Rc::new("erhan".to_string()))), VmObject::native_convert(BramaPrimative::Text(Rc::new("barış".to_string()))), VmObject::native_convert(BramaPrimative::Text(Rc::new("karamel".to_string())))].to_vec()))}
    
    nativecall_test_with_params!{test_split_1, split, primative_text!("erhan\r\n"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::List(RefCell::new([VmObject::native_convert(primative_text!("")), VmObject::native_convert(primative_text!("\r\n"))].to_vec()))}
    nativecall_test_with_params!{test_split_2, split, primative_text!("erhanbarışerhan"), [VmObject::native_convert(primative_text!("barış"))], BramaPrimative::List(RefCell::new([VmObject::native_convert(primative_text!("erhan")), VmObject::native_convert(primative_text!("erhan"))].to_vec()))}
    nativecall_test_with_params!{test_split_3, split, primative_text!("karamel"), [VmObject::native_convert(primative_text!("erhan"))], BramaPrimative::List(RefCell::new([VmObject::native_convert(primative_text!("karamel"))].to_vec()))}

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