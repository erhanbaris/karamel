use crate::types::*;
pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)), Some(BramaPrimative::Number(r_number))) => BramaPrimative::Bool(*l_number > 0.0 && *r_number > 0.0).convert(),
                    (Some(BramaPrimative::Bool(l_bool)),     Some(BramaPrimative::Bool(r_bool)))     => BramaPrimative::Bool(*l_bool && *r_bool).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Or { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)),   Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number > 0.0 || *r_number > 0.0).convert(),
                    (Some(BramaPrimative::Bool(l_bool)),       Some(BramaPrimative::Bool(r_bool)))       => BramaPrimative::Bool(*l_bool && *r_bool).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Number(*l_number + *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Multiply { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Number(*l_number * *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Division { target, left, right } => {
                let calculation = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)),  Some(BramaPrimative::Number(r_number)))   => (*l_number / *r_number),
                    _ => std::f64::NAN
                };

                memory[*target as usize] = if calculation.is_nan() {
                    BramaPrimative::Empty.convert()
                }
                else {
                    BramaPrimative::Number(calculation).convert()
                }
            },
            BramaVmOpCode::Subraction { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number(l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Number(*l_number - *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Equal { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Empty),              Some(BramaPrimative::Empty))              => BramaPrimative::Bool(true).convert(),
                    (Some(BramaPrimative::Atom(l_atom)),       Some(BramaPrimative::Atom(r_atom)))       => BramaPrimative::Bool(*l_atom == *r_atom).convert(),
                    (Some(BramaPrimative::Bool(l_bool)),       Some(BramaPrimative::Bool(r_bool)))       => BramaPrimative::Bool(*l_bool == *r_bool).convert(),
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number == *r_number).convert(),
                    (Some(BramaPrimative::Text(l_text)),       Some(BramaPrimative::Text(r_text)))       => BramaPrimative::Bool(*l_text == *r_text).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::NotEqual { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Empty),              Some(BramaPrimative::Empty))              => BramaPrimative::Bool(false).convert(),
                    (Some(BramaPrimative::Atom(l_atom)),       Some(BramaPrimative::Atom(r_atom)))       => BramaPrimative::Bool(*l_atom != *r_atom).convert(),
                    (Some(BramaPrimative::Bool(l_bool)),       Some(BramaPrimative::Bool(r_bool)))       => BramaPrimative::Bool(*l_bool != *r_bool).convert(),
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number != *r_number).convert(),
                    (Some(BramaPrimative::Text(l_text)),       Some(BramaPrimative::Text(r_text)))       => BramaPrimative::Bool(*l_text != *r_text).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::GreaterThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number > *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::GreaterEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number >= *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::LessThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number < *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::LessEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (Some(BramaPrimative::Number (l_number)),  Some(BramaPrimative::Number(r_number)))   => BramaPrimative::Bool(*l_number <= *r_number).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            _ => ()
        }
    }

    options.storages[0].dump();
}