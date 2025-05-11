use log::{LevelFilter, Log, Metadata, Record};

const MAX_LOG_LEVEL: LevelFilter = LevelFilter::Off;

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= MAX_LOG_LEVEL
    }

    fn log(&self, record: &Record) {
        println!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init() {
    let _ = unsafe { log::set_logger_racy(&LOGGER) };
}
