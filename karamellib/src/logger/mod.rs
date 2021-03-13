use log::*;
use std::cell::RefCell;

pub struct ConsoleLogger;
pub struct DummyLogger;
pub struct CollectorLogger {
    pub logs: RefCell<Vec<String>>
}

unsafe impl Send for CollectorLogger {}
unsafe impl Sync for CollectorLogger {}

pub static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;
pub static DUMMY_LOGGER: DummyLogger = DummyLogger;
pub static COLLECTOR_LOGGER: CollectorLogger = CollectorLogger { logs: RefCell::new(Vec::new()) };

impl Log for CollectorLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(not(target_arch = "wasm32"), not(test)))]
            println!("[{}] {}", record.level(), record.args());

            self.logs.borrow_mut().push(record.args().to_string());
        }
    }

    fn flush(&self) {}
}

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