use crate::types::*;
pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer > 0 && *r_integer > 0),
                    (VmObjectType::Bool(l_bool),       VmObjectType::Bool(r_bool))       => VmObjectType::Bool(*l_bool && *r_bool),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Or { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer > 0  || *r_integer > 0),
                    (VmObjectType::Double(l_double),   VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double > 0.0 || *r_double > 0.0),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer > 0   || *r_double > 0.0),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double > 0.0 || *r_integer > 0),
                    (VmObjectType::Bool(l_bool),       VmObjectType::Bool(r_bool))       => VmObjectType::Bool(*l_bool && *r_bool),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Integer(*l_integer + *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Double(*l_double + *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Double(*l_integer as f64 + *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Double(*l_double + *r_integer as f64),
                    (VmObjectType::Text(l_text),       VmObjectType::Text(r_text))       => VmObjectType::Text(l_text.to_owned() + r_text),
                    (VmObjectType::Text(l_text),       VmObjectType::Integer(r_integer)) => VmObjectType::Text(l_text.to_owned() + &r_integer.to_string()[..]),
                    (VmObjectType::Text(l_text),       VmObjectType::Double(r_double))   => VmObjectType::Text(l_text.to_owned() + &r_double.to_string()[..]),
                    (VmObjectType::Text(l_text),       VmObjectType::Bool(r_bool))       => VmObjectType::Text(l_text.to_owned() + if *r_bool { "doğru" } else { "yanlış" }),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Multiply { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Integer(*l_integer * *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Double(*l_double * *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Double(*l_integer as f64 * *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Double(*l_double * *r_integer as f64),
                    (VmObjectType::Text(l_text),       VmObjectType::Integer(r_integer)) => VmObjectType::Text(l_text.repeat(*r_integer as usize)),
                    (VmObjectType::Text(l_text),       VmObjectType::Double(r_double))   => VmObjectType::Text(l_text.repeat(*r_double as usize)),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Division { target, left, right } => {
                let calculation = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => (*l_integer as f64 / *r_integer as f64),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => (*l_double / *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => (*l_integer as f64 / *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => (*l_double / *r_integer as f64),
                    _ => std::f64::NAN
                };

                memory[*target as usize] = if calculation.is_nan() {
                    VmObjectType::Empty
                }
                else {
                    VmObjectType::Double(calculation)
                }
            },
            BramaVmOpCode::Subraction { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Integer(*l_integer - *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Double(*l_double - *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Double(*l_integer as f64 - *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Double(*l_double - *r_integer as f64),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::Equal { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Bool(l_bool),       VmObjectType::Bool(r_bool))       => VmObjectType::Bool(*l_bool == *r_bool),
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer == *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double == *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer as f64 == *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double == *r_integer as f64),
                    (VmObjectType::Text(l_text),       VmObjectType::Text(r_text))       => VmObjectType::Bool(l_text == r_text),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::NotEqual { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Bool(l_bool),       VmObjectType::Bool(r_bool))       => VmObjectType::Bool(*l_bool != *r_bool),
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer != *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double != *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer as f64 != *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double != *r_integer as f64),
                    (VmObjectType::Text(l_text),       VmObjectType::Text(r_text))       => VmObjectType::Bool(l_text != r_text),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::GreaterThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer > *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double > *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer as f64 > *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double > *r_integer as f64),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::GreaterEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer >= *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double >= *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer as f64 >= *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double >= *r_integer as f64),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::LessThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer < *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double < *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer < *r_double as i64),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double < *r_integer as f64),
                    _ => VmObjectType::Empty
                };
            },
            BramaVmOpCode::LessEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize], &memory[*right as usize]) {
                    (VmObjectType::Integer(l_integer), VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_integer <= *r_integer),
                    (VmObjectType::Double (l_double),  VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_double <= *r_double),
                    (VmObjectType::Integer(l_integer), VmObjectType::Double(r_double))   => VmObjectType::Bool(*l_integer as f64 <= *r_double),
                    (VmObjectType::Double(l_double),   VmObjectType::Integer(r_integer)) => VmObjectType::Bool(*l_double <= *r_integer as f64),
                    _ => VmObjectType::Empty
                };
            },
            _ => ()
        }
    }

    println!("{:?}", memory);
}