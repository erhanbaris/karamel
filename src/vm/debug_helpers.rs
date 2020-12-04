use crate::types::{VmObject};
use crate::compiler::*;

/* Debug methods */
#[cfg(feature = "dumpExecutionOpcode")]
pub fn dump_opcode_header() {
    let header = format!("{:^20} | {:^25} | {:^25} | {:^25}", "Opcode", "Target", "Left", "Right");
    println!();
    println!("| {} |", "-".repeat(header.len()));
    println!("| {} |", header);
    println!("| {} |", "-".repeat(header.len()));
}

#[cfg(feature = "dumpExecutionOpcode")]
pub fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject) {
    let second_part = match opcode {
        VmOpCode::NativeCall |
        VmOpCode::Subraction |
        VmOpCode::Addition   => format!("{:^25} | {:^25} | {:^25} |", format!("{:?}", target.deref()), format!("{:?}", left.deref()), format!("{:?}", right.deref())),
        _ =>                    format!("{:^25} | {:^25} | {:^25} |", format!("{:?}", target.deref()), format!("{:?}", left.deref()), format!("{:?}", right.deref()))
    };

    println!("| {:20} | {}", format!("{:?}", opcode), second_part);
}


/* Release methods */
#[cfg(not(feature = "dumpExecutionOpcode"))]
pub fn dump_opcode(opcode: VmOpCode, target: VmObject, left: VmObject, right: VmObject) { }

#[cfg(not(feature = "dumpExecutionOpcode"))]
pub fn dump_opcode_header() { }
