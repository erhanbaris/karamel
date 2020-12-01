use crate::types::{VmObject};
use crate::compiler::*;

/* Debug methods */
#[cfg(feature = "dumpExecutionOpcode")]
pub fn dump_opcode_header() {
    let header = format!("{:^20} | {:^25} | {:^25} | {:^25} | {:^3}", "Opcode", "Target", "Left", "Right", "IDX");
    println!();
    println!("| {} |", "-".repeat(header.len()));
    println!("| {} |", header);
    println!("| {} |", "-".repeat(header.len()));
}

#[cfg(feature = "dumpExecutionOpcode")]
pub fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject, tmp_index: usize) {
    let second_part = match opcode {
        VmOpCode::Load |
        VmOpCode::Store      => format!("{:^25} | {:^25} | {:^25} | {:^3} |", format!("{:?}", target.deref()), "", "", tmp_index),
        VmOpCode::NativeCall |
        VmOpCode::Subraction |
        VmOpCode::Addition   => format!("{:^25} | {:^25} | {:^25} | {:^3} |", format!("{:?}", target.deref()), format!("{:?}", left.deref()), format!("{:?}", right.deref()), tmp_index),
        _ =>                    format!("{:^25} | {:^25} | {:^25} | {:^3} |", format!("{:?}", target.deref()), format!("{:?}", left.deref()), format!("{:?}", right.deref()), tmp_index)
    };

    println!("| {:20} | {}", format!("{:?}", opcode), second_part);
}


/* Release methods */
#[cfg(not(feature = "dumpExecutionOpcode"))]
pub fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject, tmp_index: usize) { }

#[cfg(not(feature = "dumpExecutionOpcode"))]
pub fn dump_opcode_header(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject, tmp_index: usize) { }
