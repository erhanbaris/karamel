use crate::types::*;
pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::And { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number(l_integer), BramaPrimative::Number(r_integer)) => BramaPrimative::Bool(*l_integer > 0.0 && *r_integer > 0.0).convert(),
                    (BramaPrimative::Bool(l_bool),       BramaPrimative::Bool(r_bool))       => BramaPrimative::Bool(*l_bool && *r_bool).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Or { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number(l_double),   BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double > 0.0 || *r_double > 0.0).convert(),
                    (BramaPrimative::Bool(l_bool),       BramaPrimative::Bool(r_bool))       => BramaPrimative::Bool(*l_bool && *r_bool).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Addition { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Number(*l_double + *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Multiply { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Number(*l_double * *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Division { target, left, right } => {
                let calculation = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => (*l_double / *r_double),
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
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Number(*l_double - *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::Equal { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Empty,              BramaPrimative::Empty)              => BramaPrimative::Bool(true).convert(),
                    (BramaPrimative::Atom(l_atom),       BramaPrimative::Atom(r_atom))       => BramaPrimative::Bool(*l_atom == *r_atom).convert(),
                    (BramaPrimative::Bool(l_bool),       BramaPrimative::Bool(r_bool))       => BramaPrimative::Bool(*l_bool == *r_bool).convert(),
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double == *r_double).convert(),
                    (BramaPrimative::Text(l_text),       BramaPrimative::Text(r_text))       => BramaPrimative::Bool(*l_text == *r_text).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::NotEqual { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Empty,              BramaPrimative::Empty)              => BramaPrimative::Bool(false).convert(),
                    (BramaPrimative::Atom(l_atom),       BramaPrimative::Atom(r_atom))       => BramaPrimative::Bool(*l_atom != *r_atom).convert(),
                    (BramaPrimative::Bool(l_bool),       BramaPrimative::Bool(r_bool))       => BramaPrimative::Bool(*l_bool != *r_bool).convert(),
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double != *r_double).convert(),
                    (BramaPrimative::Text(l_text),       BramaPrimative::Text(r_text))       => BramaPrimative::Bool(*l_text != *r_text).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::GreaterThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double > *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::GreaterEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double >= *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::LessThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double < *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            BramaVmOpCode::LessEqualThan { target, left, right } => {
                memory[*target as usize] = match (&memory[*left as usize].convert(), &memory[*right as usize].convert()) {
                    (BramaPrimative::Number (l_double),  BramaPrimative::Number(r_double))   => BramaPrimative::Bool(*l_double <= *r_double).convert(),
                    _ => BramaPrimative::Empty.convert()
                };
            },
            _ => ()
        }
    }

    println!("{:?}", memory);
}