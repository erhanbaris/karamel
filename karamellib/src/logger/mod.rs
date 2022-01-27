use log::*;

use crate::compiler::KaramelCompilerContext;

pub struct ConsoleLogger;
pub struct DummyLogger;

pub static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
pub static DUMMY_LOGGER: DummyLogger = DummyLogger;


impl Log for DummyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }
    fn log(&self, _: &Record) {}
    fn flush(&self) {}
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(not(target_arch = "wasm32"), not(test)))]
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn write_stdout(context: &KaramelCompilerContext, data: String) {
    match &context.stdout {
        Some(out) => match out.try_borrow_mut() {
            Ok(mut out_mut) => { out_mut.push_str(&data[..]) },
            _ => ()
        },
        _ => ()
    };
}


pub fn write_stderr(context: &KaramelCompilerContext, data: String) {
    match &context.stderr {
        Some(out) => match out.try_borrow_mut() {
            Ok(mut out_mut) => { out_mut.push_str(&data[..]) },
            _ => ()
        },
        _ => ()
    };
}
