use crate::types::*;
pub fn run_vm(options: &mut BramaCompilerOption)
{
    let memory = &mut options.storages[0].memory;
    for op in &options.opcodes {
        match op {
            BramaVmOpCode::Equal { target, left, right } => {
                let left_data  = memory[*right as usize].deref();
                let right_data = memory[*left as usize].deref();
                println!("{:?}", left_data);
            },
            _ => ()
        }
    }

    options.storages[0].dump();
}