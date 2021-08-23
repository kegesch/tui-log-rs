use log::{LevelFilter, SetLoggerError, set_max_level, Log, Metadata, Record};
use std::sync::{Mutex, Arc};

pub trait Writable {
    fn write_line(&mut self, message: &str);
    fn flush(&mut self);
}

pub struct TuiLogger<W: Writable + Send + 'static> {
    level: LevelFilter,
    writable: Arc<Mutex<W>>,
}

impl <W: Writable + Send + 'static> TuiLogger<W> {
    pub fn init(log_level: LevelFilter, writable: Arc<Mutex<W>>) -> Result<(), SetLoggerError> {
        set_max_level(log_level);
        log::set_boxed_logger(TuiLogger::new(log_level, writable))
    }

    pub fn new(log_level: LevelFilter, writable: Arc<Mutex<W>>) -> Box<TuiLogger<W>> {
        Box::new(TuiLogger {
            level: log_level,
            writable
        })
    }
}

impl<W: Writable + Send + 'static> Log for TuiLogger<W> {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let target = if !record.target().is_empty() {
                record.target()
            } else {
                record.module_path().unwrap_or_default()
            };

            let mut write_lock = self.writable.lock().unwrap();

            write_lock.write_line(format!(
                "{:<5}: [{}] {}",
                record.level(),
                target,
                record.args()
            ).as_str());
        }
    }

    fn flush(&self) {
        let _ = self.writable.lock().unwrap().flush();
    }
}