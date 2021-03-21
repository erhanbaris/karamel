use log::*;

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